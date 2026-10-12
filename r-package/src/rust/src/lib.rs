use extendr_api::prelude::*;
use futures::StreamExt;
use std::fmt::Write as _;
use std::future::Future;
use std::num::NonZeroU32;
use std::sync::Arc;

// Include the pre-generated API client code
#[allow(missing_docs)]
#[allow(dead_code)]
#[allow(unused_imports)]
#[rustfmt::skip]
mod generated;

// Re-export the client from generated module
pub use generated::Client;

/// Default base URL for the MyHospitals API.
pub const DEFAULT_BASE_URL: &str = "https://myhospitalsapi.aihw.gov.au";

/// Error messages are cut to this many characters, then marked ` [truncated]`.
const MAX_ERROR_CHARS: usize = 2000;

/// At most this many bytes of a response body are quoted in an error message.
const MAX_BODY_EXCERPT: usize = 500;

/// Convert a serde-serializable value to an R list
fn to_r_value(value: &serde_json::Value) -> Robj {
    match value {
        serde_json::Value::Null => ().into_robj(),
        serde_json::Value::Bool(b) => b.into_robj(),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    (i as i32).into_robj()
                } else {
                    (i as f64).into_robj()
                }
            } else if let Some(f) = n.as_f64() {
                f.into_robj()
            } else {
                ().into_robj()
            }
        }
        // R strings can't hold NUL, and converting one raises an R error from
        // inside Rust, so drop NULs from strings and names.
        serde_json::Value::String(s) if s.contains('\0') => s.replace('\0', "").into_robj(),
        serde_json::Value::String(s) => s.into_robj(),
        serde_json::Value::Array(arr) => {
            let items: Vec<Robj> = arr.iter().map(to_r_value).collect();
            List::from_values(items).into_robj()
        }
        serde_json::Value::Object(map) => {
            let values: Vec<Robj> = map.values().map(to_r_value).collect();
            let list = if map.keys().any(|k| k.contains('\0')) {
                List::from_names_and_values(map.keys().map(|k| k.replace('\0', "")), values)
            } else {
                List::from_names_and_values(map.keys().map(String::as_str), values)
            };
            list.map(|l| l.into_robj())
                .unwrap_or_else(|_| ().into_robj())
        }
    }
}

/// Convert a serializable Rust struct to R list
fn to_r_list<T: serde::Serialize>(value: &T) -> Robj {
    match serde_json::to_value(value) {
        Ok(json_value) => to_r_value(&json_value),
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            ().into_robj()
        }
    }
}

/// Run `future` to completion on a new single-threaded runtime.
///
/// A current-thread runtime needs no worker threads, and failing to create
/// one (e.g. when file descriptors run out) is returned as an error: a panic
/// would abort the R session, because the crate is built with
/// `panic = "abort"`.
fn block_on<F: Future>(future: F) -> std::io::Result<F::Output> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    Ok(runtime.block_on(future))
}

/// A `fmt::Write` sink that keeps at most `MAX_ERROR_CHARS` characters and
/// drops NUL bytes, which R strings cannot hold. Once full it returns
/// `fmt::Error`, which stops formatting early (error payloads can be
/// megabytes) and tells the caller that the message was cut short.
#[derive(Default)]
struct BoundedMessage {
    text: String,
    chars: usize,
}

impl std::fmt::Write for BoundedMessage {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars().filter(|&c| c != '\0') {
            if self.chars == MAX_ERROR_CHARS {
                return Err(std::fmt::Error);
            }
            self.text.push(c);
            self.chars += 1;
        }
        Ok(())
    }
}

/// Build an R condition of class `myhospitals_error` with the message that
/// `write` produces.
///
/// The condition is returned, and the R method raises it with `stop()` (see
/// `R/zzz.R`). Raising it from here instead would longjmp over Rust frames,
/// skipping destructors (leaking the error and the client), and
/// `throw_r_error` passes its message to `Rf_error` as a printf format, so a
/// `%` in a URL or response body could crash R.
fn error_condition(write: impl FnOnce(&mut BoundedMessage) -> std::fmt::Result) -> Robj {
    let mut message = BoundedMessage::default();
    // Only the sink fails, and only once the message is full.
    if write(&mut message).is_err() {
        message.text.push_str(" [truncated]");
    }

    let mut condition = list!(message = message.text, call = ());
    // Setting a class on a freshly created list cannot fail.
    let _ = condition.set_class(["myhospitals_error", "error", "condition"]);
    condition.into()
}

/// Write `err` followed by the chain of errors that caused it.
fn write_with_causes(
    out: &mut impl std::fmt::Write,
    err: &dyn std::error::Error,
) -> std::fmt::Result {
    write!(out, "{err}")?;
    let mut source = err.source();
    while let Some(cause) = source {
        write!(out, ": {cause}")?;
        source = cause.source();
    }
    Ok(())
}

/// Convert a failed API call into a `myhospitals_error` condition.
///
/// The cause and the status come first: the URL and the response body can be
/// long enough to push them past `MAX_ERROR_CHARS`.
fn api_error(err: progenitor_client::Error) -> Robj {
    use progenitor_client::Error;

    error_condition(|message| {
        message.write_str("API error: ")?;
        match err {
            Error::CommunicationError(e) => {
                let url = e.url().map(ToString::to_string);
                message.write_str("Communication Error: ")?;
                write_with_causes(message, &e.without_url())?;
                if let Some(url) = url {
                    write!(message, " (url: {url})")?;
                }
                Ok(())
            }
            Error::InvalidResponsePayload(body, cause) => {
                let excerpt = body.slice(..body.len().min(MAX_BODY_EXCERPT));
                write!(
                    message,
                    "Invalid Response Payload: {cause}; body: {excerpt:?}"
                )?;
                if body.len() > MAX_BODY_EXCERPT {
                    write!(message, "... ({} bytes)", body.len())?;
                }
                Ok(())
            }
            Error::UnexpectedResponse(response) => write!(
                message,
                "Unexpected Response: {} (url: {})",
                response.status(),
                response.url()
            ),
            // `#` appends the underlying cause, if any.
            other => write!(message, "{other:#}"),
        }
    })
}

/// Convert a failure to start the async runtime into a `myhospitals_error`
/// condition.
fn runtime_error(err: std::io::Error) -> Robj {
    error_condition(|message| {
        write!(
            message,
            "API error: could not start the async runtime: {err}"
        )
    })
}

/// Send an API request and convert its JSON response to an R list. Every
/// failure becomes a `myhospitals_error` condition.
fn call_json<T, F>(request: F) -> Robj
where
    T: serde::Serialize,
    F: Future<
        Output = std::result::Result<progenitor_client::ResponseValue<T>, progenitor_client::Error>,
    >,
{
    match block_on(request) {
        Ok(Ok(response)) => to_r_list(&response.into_inner()),
        Ok(Err(e)) => api_error(e),
        Err(e) => runtime_error(e),
    }
}

/// Send a download request and return the file as an R raw vector. Every
/// failure becomes a `myhospitals_error` condition.
fn call_bytes<F>(request: F) -> Robj
where
    F: Future<
        Output = std::result::Result<
            progenitor_client::ResponseValue<progenitor_client::ByteStream>,
            progenitor_client::Error,
        >,
    >,
{
    let download = async {
        let response = request.await?;
        let mut bytes = Vec::new();
        // Size the buffer from Content-Length so it isn't regrown (and
        // copied) as chunks arrive. If that much can't be reserved, it grows.
        if let Some(len) = response
            .content_length()
            .and_then(|n| usize::try_from(n).ok())
        {
            let _ = bytes.try_reserve_exact(len);
        }
        let mut stream = response.into_inner();
        while let Some(chunk) = stream.next().await {
            bytes.extend_from_slice(&chunk?);
        }
        Ok::<_, progenitor_client::Error>(bytes)
    };
    match block_on(download) {
        Ok(Ok(bytes)) => Raw::from_bytes(&bytes).into_robj(),
        Ok(Err(e)) => api_error(e),
        Err(e) => runtime_error(e),
    }
}

/// Whether `client` still points at a Rust client.
///
/// A client restored from disk (`saveRDS()`, a saved workspace) or copied to
/// another R process holds a NULL pointer. The R methods check this before
/// calling into Rust (see `R/zzz.R`), because extendr would otherwise raise
/// its own error from inside Rust, leaking the call's arguments.
/// @noRd
#[extendr]
fn client_is_live(client: Robj) -> bool {
    <&MyHospitalsClient>::try_from(&client).is_ok()
}

/// R client for the MyHospitals Data API.
///
/// This client provides methods to access Australian hospital data
/// from the AIHW MyHospitals API.
///
/// @examples
/// \dontrun{
/// client <- MyHospitalsClient$new()
/// categories <- client$get_measure_categories()
/// }
#[extendr]
struct MyHospitalsClient {
    inner: Arc<Client>,
}

#[extendr]
impl MyHospitalsClient {
    /// Create a new MyHospitals API client
    /// @param base_url Optional custom base URL for the API
    /// @return A client object to use with other methods
    fn new(#[default = "NULL"] base_url: Option<String>) -> Self {
        // The generated client appends paths such as "/api/v1/caveats", so a
        // trailing slash would produce "//api".
        let url = base_url
            .as_deref()
            .unwrap_or(DEFAULT_BASE_URL)
            .trim_end_matches('/');
        MyHospitalsClient {
            inner: Arc::new(Client::new(url)),
        }
    }

    // ========================================================================
    // Caveats
    // ========================================================================

    /// Get all caveats
    /// @return List containing caveats
    fn get_caveats(&self) -> Robj {
        call_json(self.inner.get_caveats().send())
    }

    /// Get a specific caveat by code
    /// @param caveat_code The caveat code
    /// @return List containing the caveat
    fn get_caveat_by_code(&self, caveat_code: &str) -> Robj {
        call_json(
            self.inner
                .get_caveats_by_caveat_code()
                .caveat_code(caveat_code)
                .send(),
        )
    }

    // ========================================================================
    // Suppressions
    // ========================================================================

    /// Get all suppressions
    /// @return List containing suppressions
    fn get_suppressions(&self) -> Robj {
        call_json(self.inner.get_suppressions().send())
    }

    /// Get a specific suppression by code
    /// @param suppression_code The suppression code
    /// @return List containing the suppression
    fn get_suppression_by_code(&self, suppression_code: &str) -> Robj {
        call_json(
            self.inner
                .get_suppressions_by_suppression_code()
                .suppression_code(suppression_code)
                .send(),
        )
    }

    // ========================================================================
    // Datasets
    // ========================================================================

    /// Get all datasets
    /// @param measure_code Optional vector of measure codes to filter by
    /// @param reported_measure_code Optional vector of reported measure codes to filter by
    /// @return List containing datasets
    fn get_datasets(
        &self,
        #[default = "NULL"] measure_code: Nullable<Vec<String>>,
        #[default = "NULL"] reported_measure_code: Nullable<Vec<String>>,
    ) -> Robj {
        let mut builder = self.inner.get_datasets();
        if let NotNull(codes) = measure_code {
            builder = builder.measure_code(codes);
        }
        if let NotNull(codes) = reported_measure_code {
            builder = builder.reported_measure_code(codes);
        }
        call_json(builder.send())
    }

    /// Get a specific dataset by ID
    /// @param dataset_id The dataset ID
    /// @return List containing the dataset
    fn get_dataset_by_id(&self, dataset_id: i32) -> Robj {
        call_json(
            self.inner
                .get_datasets_by_dataset_id()
                .dataset_id(dataset_id)
                .send(),
        )
    }

    /// Get data items for a specific dataset
    /// @param dataset_id The dataset ID
    /// @return List containing data items
    fn get_dataset_data_items(&self, dataset_id: i32) -> Robj {
        call_json(
            self.inner
                .get_datasets_by_dataset_id_data_items()
                .dataset_id(dataset_id)
                .send(),
        )
    }

    // ========================================================================
    // Measures
    // ========================================================================

    /// Get all measures
    /// @param measure_category_code Optional vector of measure category codes to filter by
    /// @return List containing measures
    fn get_measures(
        &self,
        #[default = "NULL"] measure_category_code: Nullable<Vec<String>>,
    ) -> Robj {
        let mut builder = self.inner.get_measures();
        if let NotNull(codes) = measure_category_code {
            builder = builder.measure_category_code(codes);
        }
        call_json(builder.send())
    }

    /// Get a specific measure by code
    /// @param measure_code The measure code
    /// @return List containing the measure
    fn get_measure_by_code(&self, measure_code: &str) -> Robj {
        call_json(
            self.inner
                .get_measures_by_measure_code()
                .measure_code(measure_code)
                .send(),
        )
    }

    /// Get data items for a specific measure
    /// @param measure_code The measure code
    /// @return List containing data items
    fn get_measure_data_items(&self, measure_code: &str) -> Robj {
        call_json(
            self.inner
                .get_measures_by_measure_code_data_items()
                .measure_code(measure_code)
                .send(),
        )
    }

    /// Get available reporting units for a specific measure
    /// @param measure_code The measure code
    /// @return List containing available reporting units
    fn get_measure_reporting_units_available(&self, measure_code: &str) -> Robj {
        call_json(
            self.inner
                .get_measures_by_measure_code_reporting_units_available()
                .measure_code(measure_code)
                .send(),
        )
    }

    // ========================================================================
    // Measure Categories
    // ========================================================================

    /// Get all measure categories
    /// @return List containing measure categories
    fn get_measure_categories(&self) -> Robj {
        call_json(self.inner.get_measure_categories().send())
    }

    /// Get a specific measure category by code
    /// @param measure_category_code The measure category code
    /// @return List containing the measure category
    fn get_measure_category_by_code(&self, measure_category_code: &str) -> Robj {
        call_json(
            self.inner
                .get_measure_categories_by_measure_category_code()
                .measure_category_code(measure_category_code)
                .send(),
        )
    }

    /// Get measures in a specific category
    /// @param measure_category_code The measure category code
    /// @return List containing measures
    fn get_measure_category_measures(&self, measure_category_code: &str) -> Robj {
        call_json(
            self.inner
                .get_measure_categories_by_measure_category_code_measures()
                .measure_category_code(measure_category_code)
                .send(),
        )
    }

    // ========================================================================
    // Reported Measures
    // ========================================================================

    /// Get all reported measures
    /// @param reported_measure_category_code Optional vector of reported measure category codes to filter by
    /// @return List containing reported measures
    fn get_reported_measures(
        &self,
        #[default = "NULL"] reported_measure_category_code: Nullable<Vec<String>>,
    ) -> Robj {
        let mut builder = self.inner.get_reported_measures();
        if let NotNull(codes) = reported_measure_category_code {
            builder = builder.reported_measure_category_code(codes);
        }
        call_json(builder.send())
    }

    /// Get a specific reported measure by code
    /// @param reported_measure_code The reported measure code
    /// @return List containing the reported measure
    fn get_reported_measure_by_code(&self, reported_measure_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reported_measures_by_reported_measure_code()
                .reported_measure_code(reported_measure_code)
                .send(),
        )
    }

    /// Get data items for a specific reported measure
    /// @param reported_measure_code The reported measure code
    /// @return List containing data items
    fn get_reported_measure_data_items(&self, reported_measure_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reported_measures_by_reported_measure_code_data_items()
                .reported_measure_code(reported_measure_code)
                .send(),
        )
    }

    // ========================================================================
    // Reported Measure Categories
    // ========================================================================

    /// Get all reported measure categories
    /// @return List containing reported measure categories
    fn get_reported_measure_categories(&self) -> Robj {
        call_json(self.inner.get_reported_measure_categories().send())
    }

    /// Get a specific reported measure category by code
    /// @param reported_measure_category_code The reported measure category code
    /// @return List containing the reported measure category
    fn get_reported_measure_category_by_code(&self, reported_measure_category_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reported_measure_categories_by_reported_measure_category_code()
                .reported_measure_category_code(reported_measure_category_code)
                .send(),
        )
    }

    /// Get reported measures in a specific category
    /// @param reported_measure_category_code The reported measure category code
    /// @return List containing reported measures
    fn get_reported_measure_category_reported_measures(
        &self,
        reported_measure_category_code: &str,
    ) -> Robj {
        call_json(
            self.inner
                .get_reported_measure_categories_by_reported_measure_category_code_reported_measures()
                .reported_measure_category_code(reported_measure_category_code)
                .send(),
        )
    }

    // ========================================================================
    // Reporting Units
    // ========================================================================

    /// Get all reporting units
    /// @param reporting_unit_type_code Optional vector of reporting unit type codes to filter by
    /// @return List containing reporting units
    fn get_reporting_units(
        &self,
        #[default = "NULL"] reporting_unit_type_code: Nullable<Vec<String>>,
    ) -> Robj {
        let mut builder = self.inner.get_reporting_units();
        if let NotNull(codes) = reporting_unit_type_code {
            builder = builder.reporting_unit_type_code(codes);
        }
        call_json(builder.send())
    }

    /// Get a specific reporting unit by code
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing the reporting unit
    fn get_reporting_unit_by_code(&self, reporting_unit_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reporting_units_by_reporting_unit_code()
                .reporting_unit_code(reporting_unit_code)
                .send(),
        )
    }

    /// Get data items for a specific reporting unit
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing data items
    fn get_reporting_unit_data_items(&self, reporting_unit_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reporting_units_by_reporting_unit_code_data_items()
                .reporting_unit_code(reporting_unit_code)
                .send(),
        )
    }

    /// Get available measures for a specific reporting unit
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing available measures
    fn get_reporting_unit_measures_available(&self, reporting_unit_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reporting_units_by_reporting_unit_code_measures_available()
                .reporting_unit_code(reporting_unit_code)
                .send(),
        )
    }

    /// Get available bricks for a specific reporting unit
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing available bricks
    fn get_reporting_unit_bricks_available(&self, reporting_unit_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reporting_units_by_reporting_unit_code_bricks_available()
                .reporting_unit_code(reporting_unit_code)
                .send(),
        )
    }

    // ========================================================================
    // Reporting Unit Types
    // ========================================================================

    /// Get all reporting unit types
    /// @return List containing reporting unit types
    fn get_reporting_unit_types(&self) -> Robj {
        call_json(self.inner.get_reporting_unit_types().send())
    }

    /// Get a specific reporting unit type by code
    /// @param reporting_unit_type_code The reporting unit type code
    /// @return List containing the reporting unit type
    fn get_reporting_unit_type_by_code(&self, reporting_unit_type_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reporting_unit_types_by_reporting_unit_type_code()
                .reporting_unit_type_code(reporting_unit_type_code)
                .send(),
        )
    }

    /// Get available bricks for a specific reporting unit type
    /// @param reporting_unit_type_code The reporting unit type code
    /// @return List containing available bricks
    fn get_reporting_unit_type_bricks_available(&self, reporting_unit_type_code: &str) -> Robj {
        call_json(
            self.inner
                .get_reporting_unit_types_by_reporting_unit_type_code_bricks_available()
                .reporting_unit_type_code(reporting_unit_type_code)
                .send(),
        )
    }

    // ========================================================================
    // Flat Data Extracts
    // ========================================================================

    /// Get flat data extract by measure category code
    ///
    /// Returns raw numeric data values. Use for data analysis.
    ///
    /// @param measure_category_code The measure category code
    /// @param skip Number of records to skip (pagination). Default 0.
    /// @param top Number of records to return (max 1000). Default 100.
    /// @param reporting_unit_type_code Optional filter by reporting unit type codes
    /// @param measure_code Optional filter by measure codes
    /// @param reporting_unit_code Optional filter by reporting unit codes
    /// @param start_date Optional start date filter (ISO format)
    /// @param end_date Optional end date filter (ISO format)
    /// @return List containing data extract
    #[allow(clippy::too_many_arguments)]
    fn get_flat_data_extract(
        &self,
        measure_category_code: &str,
        #[default = "0L"] skip: i32,
        #[default = "100L"] top: u32,
        #[default = "NULL"] reporting_unit_type_code: Nullable<Vec<String>>,
        #[default = "NULL"] measure_code: Nullable<Vec<String>>,
        #[default = "NULL"] reporting_unit_code: Nullable<Vec<String>>,
        #[default = "NULL"] start_date: Nullable<String>,
        #[default = "NULL"] end_date: Nullable<String>,
    ) -> Robj {
        let mut builder = self
            .inner
            .get_flat_data_extract_by_measure_category_code()
            .measure_category_code(measure_category_code)
            .skip(skip)
            .top(NonZeroU32::new(top).unwrap_or(NonZeroU32::MIN));

        if let NotNull(codes) = reporting_unit_type_code {
            builder = builder.reporting_unit_type_code(codes);
        }
        if let NotNull(codes) = measure_code {
            builder = builder.measure_code(codes);
        }
        if let NotNull(codes) = reporting_unit_code {
            builder = builder.reporting_unit_code(codes);
        }
        if let NotNull(date) = start_date {
            builder = builder.start_date(date);
        }
        if let NotNull(date) = end_date {
            builder = builder.end_date(date);
        }

        call_json(builder.send())
    }

    /// Get flat formatted data extract by measure category code
    ///
    /// Returns display-ready formatted values. Use for presentation.
    ///
    /// @param measure_category_code The measure category code
    /// @param skip Number of records to skip (pagination). Default 0.
    /// @param top Number of records to return (max 1000). Default 100.
    /// @param reporting_unit_type_code Optional filter by reporting unit type codes
    /// @param measure_code Optional filter by measure codes
    /// @param reporting_unit_code Optional filter by reporting unit codes
    /// @param start_date Optional start date filter (ISO format)
    /// @param end_date Optional end date filter (ISO format)
    /// @return List containing formatted data extract
    #[allow(clippy::too_many_arguments)]
    fn get_flat_formatted_data_extract(
        &self,
        measure_category_code: &str,
        #[default = "0L"] skip: i32,
        #[default = "100L"] top: u32,
        #[default = "NULL"] reporting_unit_type_code: Nullable<Vec<String>>,
        #[default = "NULL"] measure_code: Nullable<Vec<String>>,
        #[default = "NULL"] reporting_unit_code: Nullable<Vec<String>>,
        #[default = "NULL"] start_date: Nullable<String>,
        #[default = "NULL"] end_date: Nullable<String>,
    ) -> Robj {
        let mut builder = self
            .inner
            .get_flat_formatted_data_extract_by_measure_category_code()
            .measure_category_code(measure_category_code)
            .skip(skip)
            .top(NonZeroU32::new(top).unwrap_or(NonZeroU32::MIN));

        if let NotNull(codes) = reporting_unit_type_code {
            builder = builder.reporting_unit_type_code(codes);
        }
        if let NotNull(codes) = measure_code {
            builder = builder.measure_code(codes);
        }
        if let NotNull(codes) = reporting_unit_code {
            builder = builder.reporting_unit_code(codes);
        }
        if let NotNull(date) = start_date {
            builder = builder.start_date(date);
        }
        if let NotNull(date) = end_date {
            builder = builder.end_date(date);
        }

        call_json(builder.send())
    }

    // ========================================================================
    // Downloads
    // ========================================================================

    /// Get available simple download codes
    /// @return List containing download codes
    fn get_simple_download_codes(&self) -> Robj {
        call_json(self.inner.get_simple_downloads_download_codes().send())
    }

    /// Download a simple data file (XLSX)
    /// @param download_code The download code
    /// @return Raw bytes of the XLSX file
    fn download_simple(&self, download_code: &str) -> Robj {
        call_bytes(
            self.inner
                .get_simple_downloads_by_download_code()
                .download_code(download_code)
                .send(),
        )
    }

    /// Get available measure download codes
    /// @return List containing measure download codes
    fn get_measure_download_codes(&self) -> Robj {
        call_json(
            self.inner
                .get_measure_downloads_measure_download_codes()
                .send(),
        )
    }

    /// Download a measure data file (XLSX)
    /// @param measure_download_code The measure download code
    /// @return Raw bytes of the XLSX file
    fn download_measure(&self, measure_download_code: &str) -> Robj {
        call_bytes(
            self.inner
                .get_measure_downloads_by_measure_download_code()
                .measure_download_code(measure_download_code)
                .send(),
        )
    }

    /// Download measure data across reporting units (XLSX)
    /// @param measure_download_code The measure download code
    /// @return Raw bytes of the XLSX file
    fn download_measure_across_reporting_units(&self, measure_download_code: &str) -> Robj {
        call_bytes(
            self.inner
                .get_measure_downloads_across_reporting_units_by_measure_download_code()
                .measure_download_code(measure_download_code)
                .send(),
        )
    }

    /// Get available reporting unit datasheet codes
    /// @return List containing datasheet codes
    fn get_reporting_unit_datasheet_codes(&self) -> Robj {
        call_json(
            self.inner
                .get_reporting_units_downloads_datasheet_codes()
                .send(),
        )
    }

    /// Download a reporting unit datasheet (XLSX)
    /// @param datasheet_code The datasheet code
    /// @param reporting_unit_code The reporting unit code
    /// @return Raw bytes of the XLSX file
    fn download_reporting_unit_datasheet(
        &self,
        datasheet_code: &str,
        reporting_unit_code: &str,
    ) -> Robj {
        call_bytes(
            self.inner
                .get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code()
                .datasheet_code(datasheet_code)
                .reporting_unit_code(reporting_unit_code)
                .send(),
        )
    }
}

// Macro to generate the module initialization for R
extendr_module! {
    mod aihwmyhospitals;
    fn client_is_live;
    impl MyHospitalsClient;
}

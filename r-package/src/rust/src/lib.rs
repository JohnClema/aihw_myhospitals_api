use extendr_api::prelude::*;
use futures::StreamExt;
use std::sync::Arc;

// Include the pre-generated API client code
#[allow(missing_docs)]
#[allow(dead_code)]
#[allow(unused_imports)]
mod generated;

// Re-export the client from generated module
pub use generated::Client;

/// Default base URL for the MyHospitals API.
pub const DEFAULT_BASE_URL: &str = "https://myhospitalsapi.aihw.gov.au";

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
        serde_json::Value::String(s) => s.into_robj(),
        serde_json::Value::Array(arr) => {
            let items: Vec<Robj> = arr.iter().map(to_r_value).collect();
            List::from_values(items).into_robj()
        }
        serde_json::Value::Object(map) => {
            let names: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
            let values: Vec<Robj> = map.values().map(to_r_value).collect();
            List::from_names_and_values(names, values)
                .map(|l| l.into_robj())
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

/// Helper to run async code synchronously
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(future)
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
    fn new(base_url: Option<String>) -> Self {
        let url = base_url.as_deref().unwrap_or(DEFAULT_BASE_URL);
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
        let result = block_on(self.inner.get_caveats().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific caveat by code
    /// @param caveat_code The caveat code
    /// @return List containing the caveat
    fn get_caveat_by_code(&self, caveat_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_caveats_by_caveat_code()
                .caveat_code(caveat_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Suppressions
    // ========================================================================

    /// Get all suppressions
    /// @return List containing suppressions
    fn get_suppressions(&self) -> Robj {
        let result = block_on(self.inner.get_suppressions().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific suppression by code
    /// @param suppression_code The suppression code
    /// @return List containing the suppression
    fn get_suppression_by_code(&self, suppression_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_suppressions_by_suppression_code()
                .suppression_code(suppression_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Datasets
    // ========================================================================

    /// Get all datasets
    /// @param measure_code Optional vector of measure codes to filter by
    /// @param reported_measure_code Optional vector of reported measure codes to filter by
    /// @return List containing datasets
    fn get_datasets(&self, measure_code: Nullable<Vec<String>>, reported_measure_code: Nullable<Vec<String>>) -> Robj {
        let result = block_on(async {
            let mut builder = self.inner.get_datasets();
            if let NotNull(codes) = measure_code {
                builder = builder.measure_code(codes);
            }
            if let NotNull(codes) = reported_measure_code {
                builder = builder.reported_measure_code(codes);
            }
            builder.send().await
        });
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific dataset by ID
    /// @param dataset_id The dataset ID
    /// @return List containing the dataset
    fn get_dataset_by_id(&self, dataset_id: i32) -> Robj {
        let result = block_on(
            self.inner
                .get_datasets_by_dataset_id()
                .dataset_id(dataset_id)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get data items for a specific dataset
    /// @param dataset_id The dataset ID
    /// @return List containing data items
    fn get_dataset_data_items(&self, dataset_id: i32) -> Robj {
        let result = block_on(
            self.inner
                .get_datasets_by_dataset_id_data_items()
                .dataset_id(dataset_id)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Measures
    // ========================================================================

    /// Get all measures
    /// @param measure_category_code Optional vector of measure category codes to filter by
    /// @return List containing measures
    fn get_measures(&self, measure_category_code: Nullable<Vec<String>>) -> Robj {
        let result = block_on(async {
            let mut builder = self.inner.get_measures();
            if let NotNull(codes) = measure_category_code {
                builder = builder.measure_category_code(codes);
            }
            builder.send().await
        });
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific measure by code
    /// @param measure_code The measure code
    /// @return List containing the measure
    fn get_measure_by_code(&self, measure_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_measures_by_measure_code()
                .measure_code(measure_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get data items for a specific measure
    /// @param measure_code The measure code
    /// @return List containing data items
    fn get_measure_data_items(&self, measure_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_measures_by_measure_code_data_items()
                .measure_code(measure_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get available reporting units for a specific measure
    /// @param measure_code The measure code
    /// @return List containing available reporting units
    fn get_measure_reporting_units_available(&self, measure_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_measures_by_measure_code_reporting_units_available()
                .measure_code(measure_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Measure Categories
    // ========================================================================

    /// Get all measure categories
    /// @return List containing measure categories
    fn get_measure_categories(&self) -> Robj {
        let result = block_on(self.inner.get_measure_categories().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific measure category by code
    /// @param measure_category_code The measure category code
    /// @return List containing the measure category
    fn get_measure_category_by_code(&self, measure_category_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_measure_categories_by_measure_category_code()
                .measure_category_code(measure_category_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get measures in a specific category
    /// @param measure_category_code The measure category code
    /// @return List containing measures
    fn get_measure_category_measures(&self, measure_category_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_measure_categories_by_measure_category_code_measures()
                .measure_category_code(measure_category_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Reported Measures
    // ========================================================================

    /// Get all reported measures
    /// @param reported_measure_category_code Optional vector of reported measure category codes to filter by
    /// @return List containing reported measures
    fn get_reported_measures(&self, reported_measure_category_code: Nullable<Vec<String>>) -> Robj {
        let result = block_on(async {
            let mut builder = self.inner.get_reported_measures();
            if let NotNull(codes) = reported_measure_category_code {
                builder = builder.reported_measure_category_code(codes);
            }
            builder.send().await
        });
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific reported measure by code
    /// @param reported_measure_code The reported measure code
    /// @return List containing the reported measure
    fn get_reported_measure_by_code(&self, reported_measure_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reported_measures_by_reported_measure_code()
                .reported_measure_code(reported_measure_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get data items for a specific reported measure
    /// @param reported_measure_code The reported measure code
    /// @return List containing data items
    fn get_reported_measure_data_items(&self, reported_measure_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reported_measures_by_reported_measure_code_data_items()
                .reported_measure_code(reported_measure_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Reported Measure Categories
    // ========================================================================

    /// Get all reported measure categories
    /// @return List containing reported measure categories
    fn get_reported_measure_categories(&self) -> Robj {
        let result = block_on(self.inner.get_reported_measure_categories().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific reported measure category by code
    /// @param reported_measure_category_code The reported measure category code
    /// @return List containing the reported measure category
    fn get_reported_measure_category_by_code(&self, reported_measure_category_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reported_measure_categories_by_reported_measure_category_code()
                .reported_measure_category_code(reported_measure_category_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get reported measures in a specific category
    /// @param reported_measure_category_code The reported measure category code
    /// @return List containing reported measures
    fn get_reported_measure_category_reported_measures(&self, reported_measure_category_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reported_measure_categories_by_reported_measure_category_code_reported_measures()
                .reported_measure_category_code(reported_measure_category_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Reporting Units
    // ========================================================================

    /// Get all reporting units
    /// @param reporting_unit_type_code Optional vector of reporting unit type codes to filter by
    /// @return List containing reporting units
    fn get_reporting_units(&self, reporting_unit_type_code: Nullable<Vec<String>>) -> Robj {
        let result = block_on(async {
            let mut builder = self.inner.get_reporting_units();
            if let NotNull(codes) = reporting_unit_type_code {
                builder = builder.reporting_unit_type_code(codes);
            }
            builder.send().await
        });
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific reporting unit by code
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing the reporting unit
    fn get_reporting_unit_by_code(&self, reporting_unit_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reporting_units_by_reporting_unit_code()
                .reporting_unit_code(reporting_unit_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get data items for a specific reporting unit
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing data items
    fn get_reporting_unit_data_items(&self, reporting_unit_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reporting_units_by_reporting_unit_code_data_items()
                .reporting_unit_code(reporting_unit_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get available measures for a specific reporting unit
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing available measures
    fn get_reporting_unit_measures_available(&self, reporting_unit_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reporting_units_by_reporting_unit_code_measures_available()
                .reporting_unit_code(reporting_unit_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get available bricks for a specific reporting unit
    /// @param reporting_unit_code The reporting unit code
    /// @return List containing available bricks
    fn get_reporting_unit_bricks_available(&self, reporting_unit_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reporting_units_by_reporting_unit_code_bricks_available()
                .reporting_unit_code(reporting_unit_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Reporting Unit Types
    // ========================================================================

    /// Get all reporting unit types
    /// @return List containing reporting unit types
    fn get_reporting_unit_types(&self) -> Robj {
        let result = block_on(self.inner.get_reporting_unit_types().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get a specific reporting unit type by code
    /// @param reporting_unit_type_code The reporting unit type code
    /// @return List containing the reporting unit type
    fn get_reporting_unit_type_by_code(&self, reporting_unit_type_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reporting_unit_types_by_reporting_unit_type_code()
                .reporting_unit_type_code(reporting_unit_type_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get available bricks for a specific reporting unit type
    /// @param reporting_unit_type_code The reporting unit type code
    /// @return List containing available bricks
    fn get_reporting_unit_type_bricks_available(&self, reporting_unit_type_code: &str) -> Robj {
        let result = block_on(
            self.inner
                .get_reporting_unit_types_by_reporting_unit_type_code_bricks_available()
                .reporting_unit_type_code(reporting_unit_type_code)
                .send()
        );
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
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
    fn get_flat_data_extract(
        &self,
        measure_category_code: &str,
        skip: i32,
        top: u32,
        reporting_unit_type_code: Nullable<Vec<String>>,
        measure_code: Nullable<Vec<String>>,
        reporting_unit_code: Nullable<Vec<String>>,
        start_date: Nullable<String>,
        end_date: Nullable<String>,
    ) -> Robj {
        let result = block_on(async {
            let top_nonzero = std::num::NonZeroU32::new(top.max(1))
                .expect("top must be > 0");

            let mut builder = self.inner
                .get_flat_data_extract_by_measure_category_code()
                .measure_category_code(measure_category_code)
                .skip(skip)
                .top(top_nonzero);

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

            builder.send().await
        });
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
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
    fn get_flat_formatted_data_extract(
        &self,
        measure_category_code: &str,
        skip: i32,
        top: u32,
        reporting_unit_type_code: Nullable<Vec<String>>,
        measure_code: Nullable<Vec<String>>,
        reporting_unit_code: Nullable<Vec<String>>,
        start_date: Nullable<String>,
        end_date: Nullable<String>,
    ) -> Robj {
        let result = block_on(async {
            let top_nonzero = std::num::NonZeroU32::new(top.max(1))
                .expect("top must be > 0");

            let mut builder = self.inner
                .get_flat_formatted_data_extract_by_measure_category_code()
                .measure_category_code(measure_category_code)
                .skip(skip)
                .top(top_nonzero);

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

            builder.send().await
        });
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    // ========================================================================
    // Downloads
    // ========================================================================

    /// Get available simple download codes
    /// @return List containing download codes
    fn get_simple_download_codes(&self) -> Robj {
        let result = block_on(self.inner.get_simple_downloads_download_codes().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Download a simple data file (XLSX)
    /// @param download_code The download code
    /// @return Raw bytes of the XLSX file
    fn download_simple(&self, download_code: &str) -> Robj {
        let result = block_on(async {
            let response = self.inner
                .get_simple_downloads_by_download_code()
                .download_code(download_code)
                .send()
                .await?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                bytes.extend_from_slice(&chunk);
            }
            Ok::<_, progenitor_client::Error<_>>(bytes)
        });
        match result {
            Ok(bytes) => Raw::from_bytes(&bytes).into_robj(),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get available measure download codes
    /// @return List containing measure download codes
    fn get_measure_download_codes(&self) -> Robj {
        let result = block_on(self.inner.get_measure_downloads_measure_download_codes().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Download a measure data file (XLSX)
    /// @param measure_download_code The measure download code
    /// @return Raw bytes of the XLSX file
    fn download_measure(&self, measure_download_code: &str) -> Robj {
        let result = block_on(async {
            let response = self.inner
                .get_measure_downloads_by_measure_download_code()
                .measure_download_code(measure_download_code)
                .send()
                .await?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                bytes.extend_from_slice(&chunk);
            }
            Ok::<_, progenitor_client::Error<_>>(bytes)
        });
        match result {
            Ok(bytes) => Raw::from_bytes(&bytes).into_robj(),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Download measure data across reporting units (XLSX)
    /// @param measure_download_code The measure download code
    /// @return Raw bytes of the XLSX file
    fn download_measure_across_reporting_units(&self, measure_download_code: &str) -> Robj {
        let result = block_on(async {
            let response = self.inner
                .get_measure_downloads_across_reporting_units_by_measure_download_code()
                .measure_download_code(measure_download_code)
                .send()
                .await?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                bytes.extend_from_slice(&chunk);
            }
            Ok::<_, progenitor_client::Error<_>>(bytes)
        });
        match result {
            Ok(bytes) => Raw::from_bytes(&bytes).into_robj(),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Get available reporting unit datasheet codes
    /// @return List containing datasheet codes
    fn get_reporting_unit_datasheet_codes(&self) -> Robj {
        let result = block_on(self.inner.get_reporting_units_downloads_datasheet_codes().send());
        match result {
            Ok(response) => to_r_list(&response.into_inner()),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }

    /// Download a reporting unit datasheet (XLSX)
    /// @param datasheet_code The datasheet code
    /// @param reporting_unit_code The reporting unit code
    /// @return Raw bytes of the XLSX file
    fn download_reporting_unit_datasheet(&self, datasheet_code: &str, reporting_unit_code: &str) -> Robj {
        let result = block_on(async {
            let response = self.inner
                .get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code()
                .datasheet_code(datasheet_code)
                .reporting_unit_code(reporting_unit_code)
                .send()
                .await?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                bytes.extend_from_slice(&chunk);
            }
            Ok::<_, progenitor_client::Error<_>>(bytes)
        });
        match result {
            Ok(bytes) => Raw::from_bytes(&bytes).into_robj(),
            Err(e) => {
                throw_r_error(format!("API error: {}", e));
            }
        }
    }
}

// Macro to generate the module initialization for R
extendr_module! {
    mod aihwmyhospitals;
    impl MyHospitalsClient;
}

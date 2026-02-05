//! # MyHospitals Data API Client
//!
//! A Rust client for the [MyHospitals Data API](https://myhospitalsapi.aihw.gov.au/)
//! provided by the Australian Institute of Health and Welfare (AIHW).
//!
//! ## Features
//!
//! - Auto-generated from OpenAPI specification using [Progenitor](https://github.com/oxidecomputer/progenitor)
//! - Async/await support with Tokio
//! - Builder pattern for ergonomic API calls
//! - Strongly typed requests and responses
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use aihw_myhospitals_api::{Client, new_client};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Using the convenience function
//!     let client = new_client();
//!
//!     // Or with explicit URL
//!     let client = Client::new("https://myhospitalsapi.aihw.gov.au");
//!
//!     // Fetch all datasets
//!     let response = client.get_datasets().send().await?;
//!     println!("Found {} datasets", response.result.len());
//!
//!     // Access individual fields (use * to dereference newtype wrappers)
//!     for dataset in &response.result {
//!         println!("{}: {} to {}",
//!             *dataset.data_set_name,
//!             dataset.reporting_start_date,
//!             dataset.reporting_end_date
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## API Categories
//!
//! The client provides access to all MyHospitals API endpoints:
//!
//! - **Caveats** - Data footnotes and suppressions
//! - **DataSets** - Grouped data periods
//! - **Measures** - Health metrics and indicators
//! - **MeasureCategories** - Measure classifications
//! - **ReportedMeasures** - Reported measurement forms
//! - **ReportingUnits** - Hospitals and health services
//! - **ReportingUnitTypes** - Unit classifications (Hospital, State, National)
//! - **FlatDataExtracts** - Bulk data retrieval (paginated, max 1000)
//! - **Downloads** - Excel/XLSX exports
//!
//! ## Response Structure
//!
//! All API responses are wrapped in an envelope containing:
//! - `result` - The actual data
//! - `version_information` - API version and data version metadata
//!
//! ```rust,no_run
//! # use aihw_myhospitals_api::new_client;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = new_client();
//! let response = client.get_reporting_unit_types().send().await?;
//!
//! // Access the data
//! for unit_type in &response.result {
//!     println!("{}", *unit_type.reporting_unit_type_name);
//! }
//!
//! // Access version info
//! if let Some(info) = &response.version_information {
//!     println!("API Version: {:?}", info.api_version);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Newtype Wrappers
//!
//! Many fields use newtype wrappers for type safety. Use `*` to dereference:
//!
//! ```rust,no_run
//! # use aihw_myhospitals_api::new_client;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = new_client();
//! let response = client.get_measures().send().await?;
//!
//! for measure in &response.result {
//!     // Dereference to get the underlying String
//!     println!("Code: {}", *measure.measure_code);
//!     println!("Name: {}", *measure.measure_name);
//! }
//! # Ok(())
//! # }
//! ```

// Include the generated client code
// Note: We allow missing_docs here since the code is auto-generated
#[allow(missing_docs)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

#[warn(missing_docs)]
pub use generated::*;

/// Default base URL for the MyHospitals API.
///
/// This is the production URL: `https://myhospitalsapi.aihw.gov.au`
pub const DEFAULT_BASE_URL: &str = "https://myhospitalsapi.aihw.gov.au";

/// Create a new client configured for the production MyHospitals API.
///
/// This is equivalent to `Client::new(DEFAULT_BASE_URL)`.
///
/// # Example
///
/// ```rust,no_run
/// use aihw_myhospitals_api::new_client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = new_client();
///     let response = client.get_measure_categories().send().await?;
///     println!("Found {} categories", response.result.len());
///     Ok(())
/// }
/// ```
pub fn new_client() -> Client {
    Client::new(DEFAULT_BASE_URL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = new_client();
        let _ = client;
    }

    #[test]
    fn test_custom_base_url() {
        let client = Client::new("https://custom.example.com");
        let _ = client;
    }

    #[test]
    fn test_default_base_url() {
        assert_eq!(DEFAULT_BASE_URL, "https://myhospitalsapi.aihw.gov.au");
    }
}

/// Integration tests that require network access.
///
/// Run with: `cargo test --features integration-tests`
#[cfg(all(test, feature = "integration-tests"))]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_measure_categories() {
        let client = new_client();
        let response = client.get_measure_categories().send().await.unwrap();
        assert!(!response.result.is_empty(), "Should have at least one measure category");
    }

    #[tokio::test]
    async fn test_get_reporting_unit_types() {
        let client = new_client();
        let response = client.get_reporting_unit_types().send().await.unwrap();
        assert!(!response.result.is_empty(), "Should have at least one reporting unit type");

        // Verify expected types exist
        let type_codes: Vec<&str> = response
            .result
            .iter()
            .map(|t| t.reporting_unit_type_code.as_str())
            .collect();
        assert!(type_codes.contains(&"H"), "Should have Hospital type");
        assert!(type_codes.contains(&"NAT"), "Should have National type");
    }

    #[tokio::test]
    async fn test_get_datasets() {
        let client = new_client();
        let response = client.get_datasets().send().await.unwrap();
        assert!(!response.result.is_empty(), "Should have at least one dataset");

        // Verify version information is present
        assert!(response.version_information.is_some(), "Should have version information");
    }
}

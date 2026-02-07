//! Example: Basic usage of the MyHospitals API client
//!
//! Run with: pixi run example

use aihw_myhospitals_api::new_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    println!("=== MyHospitals Data API Demo ===\n");

    // List measure categories
    println!("Fetching measure categories...");
    let response = client.get_measure_categories().send().await?;
    println!("Found {} measure categories:", response.result.len());
    for cat in response.result.iter().take(5) {
        println!(
            "  - {:?} ({:?})",
            cat.measure_category_name, cat.measure_category_code
        );
    }

    // List reporting unit types
    println!("\nFetching reporting unit types...");
    let response = client.get_reporting_unit_types().send().await?;
    println!("Found {} reporting unit types:", response.result.len());
    for ut in response.result.iter() {
        println!(
            "  - {:?} ({:?})",
            ut.reporting_unit_type_name, ut.reporting_unit_type_code
        );
    }

    // List datasets (demonstrates date parsing)
    println!("\nFetching datasets...");
    let response = client.get_datasets().send().await?;
    println!("Found {} datasets. First 3:", response.result.len());
    for ds in response.result.iter().take(3) {
        println!("  - ID {}: {:?}", ds.data_set_id, ds.data_set_name);
        println!(
            "    Period: {} to {}",
            ds.reporting_start_date, ds.reporting_end_date
        );
    }

    // Show version information
    if let Some(version_info) = &response.version_information {
        println!("\nAPI Version Info:");
        if let Some(api_version) = &version_info.api_version {
            println!("  API Version: {}", api_version);
        }
        if let Some(data_version) = &version_info.data_version {
            println!("  Data Version: {}", data_version);
        }
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}

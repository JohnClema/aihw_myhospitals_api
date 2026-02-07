//! Example: Working with datasets
//!
//! Run with: pixi run -- cargo run --example datasets

use aihw_myhospitals_api::new_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    println!("=== Datasets API ===\n");

    // Get all datasets
    println!("Fetching all datasets...");
    let response = client.get_datasets().send().await?;
    println!("Found {} datasets:\n", response.result.len());

    for dataset in response.result.iter().take(5) {
        println!("ID: {}", dataset.data_set_id);
        println!("Name: {}", *dataset.data_set_name);
        println!(
            "Period: {} to {}",
            dataset.reporting_start_date, dataset.reporting_end_date
        );
        println!();
    }

    // Get a specific dataset by ID
    if let Some(first_dataset) = response.result.first() {
        let dataset_id = first_dataset.data_set_id;
        println!("--- Fetching dataset details for ID: {} ---\n", dataset_id);

        let detail = client
            .get_datasets_by_dataset_id()
            .dataset_id(dataset_id)
            .send()
            .await?;

        println!("Dataset: {}", *detail.result.data_set_name);
        println!("Start Date: {}", detail.result.reporting_start_date);
        println!("End Date: {}", detail.result.reporting_end_date);

        // Get data items for this dataset
        // Note: This requires a reporting_unit_code parameter
        println!("\n--- Fetching data items for dataset {} ---\n", dataset_id);

        // First get a reporting unit
        let units = client.get_reporting_units().send().await?;
        if let Some(unit) = units.result.first() {
            let unit_code = &*unit.reporting_unit_code;

            let items = client
                .get_datasets_by_dataset_id_data_items()
                .dataset_id(dataset_id)
                .reporting_unit_code(vec![unit_code.to_string()])
                .send()
                .await?;

            // Result is a single DataItemModel (not a Vec)
            println!("Data item for unit {}:", unit_code);
            println!(
                "  Measure: {}, Value: {:?}",
                *items.result.measure_code, items.result.value
            );
        }
    }

    // Show version information
    if let Some(version_info) = &response.version_information {
        println!("\n--- Version Information ---");
        if let Some(api_version) = &version_info.api_version {
            println!("API Version: {}", api_version);
        }
        if let Some(data_version) = &version_info.data_version {
            println!("Data Version: {}", data_version);
        }
    }

    Ok(())
}

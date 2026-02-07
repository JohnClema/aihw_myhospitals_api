//! Example: Working with Excel/XLSX downloads
//!
//! Run with: pixi run -- cargo run --example downloads
//!
//! Note: Download endpoints return raw byte streams (XLSX files).
//! This example shows how to list available downloads.
//! Actual file downloads return ByteStream which needs special handling.

use aihw_myhospitals_api::new_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    println!("=== Downloads API ===\n");
    println!("The API provides Excel/XLSX downloads for various data views.\n");

    // Get available simple downloads
    // Note: The result is a HashMap<String, Vec<DatasheetConfigurationSummaryModel>>
    println!("--- Simple Downloads ---\n");

    let simple_downloads = client.get_simple_downloads_download_codes().send().await?;
    println!(
        "Found {} simple download categories:\n",
        simple_downloads.result.len()
    );

    for (category, configs) in simple_downloads.result.iter().take(3) {
        println!("Category: {}", category);
        for config in configs.iter().take(2) {
            println!("  - Code: {}", *config.datasheet_code);
            println!("    Type: {}", *config.datasheet_type);
            println!("    Description: {}", *config.datasheet_description);
        }
        println!();
    }

    // Get available measure downloads
    // Note: The result is a HashMap<String, Vec<DatasheetConfigurationSummaryModel>>
    println!("--- Measure Downloads ---\n");

    let measure_downloads = client
        .get_measure_downloads_measure_download_codes()
        .send()
        .await?;
    println!(
        "Found {} measure download categories:\n",
        measure_downloads.result.len()
    );

    for (category, configs) in measure_downloads.result.iter().take(3) {
        println!("Category: {}", category);
        for config in configs.iter().take(2) {
            println!("  - Code: {}", *config.datasheet_code);
            println!("    Type: {}", *config.datasheet_type);
        }
        println!();
    }

    // Get available datasheet downloads for reporting units
    println!("--- Reporting Unit Datasheet Downloads ---\n");

    let datasheet_codes = client
        .get_reporting_units_downloads_datasheet_codes()
        .send()
        .await?;
    println!(
        "Found {} datasheet categories:\n",
        datasheet_codes.result.len()
    );

    for (category, configs) in datasheet_codes.result.iter().take(3) {
        println!("Category: {}", category);
        for config in configs.iter().take(2) {
            println!("  - Code: {}", *config.datasheet_code);
            println!("    Type: {}", *config.datasheet_type);
        }
        println!();
    }

    // Example of downloading a file
    // Note: Download endpoints return ByteStream which you need to collect
    println!("--- Download Example ---\n");
    println!("To download a file, use one of these methods:");
    println!("  - get_simple_downloads_by_download_code(code)");
    println!("  - get_measure_downloads_by_measure_download_code(code)");
    println!("  - get_measure_downloads_across_reporting_units_by_measure_download_code(code)");
    println!("  - get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code(datasheet, unit)");
    println!();
    println!("Example:");
    println!("```rust");
    println!("use futures::StreamExt;");
    println!();
    println!("let response = client");
    println!("    .get_simple_downloads_by_download_code()");
    println!("    .download_code(\"some-code\")");
    println!("    .send()");
    println!("    .await?;");
    println!();
    println!("let mut bytes = Vec::new();");
    println!("let mut stream = response.into_inner();");
    println!("while let Some(chunk) = stream.next().await {{");
    println!("    bytes.extend_from_slice(&chunk?);");
    println!("}}");
    println!("std::fs::write(\"output.xlsx\", &bytes)?;");
    println!("```");

    Ok(())
}

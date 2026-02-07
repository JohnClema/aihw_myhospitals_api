//! Example: Working with flat data extracts (bulk data retrieval)
//!
//! Run with: pixi run -- cargo run --example data_extracts
//!
//! Note: The flat data extract API uses skip/top pagination (not page_number/page_size)
//! with a maximum of 1000 results per request.

use aihw_myhospitals_api::new_client;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    println!("=== Flat Data Extracts API ===\n");
    println!("These endpoints provide bulk data retrieval with pagination.\n");
    println!("NOTE: Uses skip/top pagination with max 1000 results per request.\n");

    // First, get a measure category to use
    println!("Fetching measure categories...");
    let categories = client.get_measure_categories().send().await?;

    if let Some(category) = categories.result.first() {
        let code = &*category.measure_category_code;
        println!(
            "Using category: {} ({})\n",
            *category.measure_category_name, code
        );

        // Get flat data extract (raw numeric values)
        // Parameters: skip (offset), top (limit, 1-1000)
        println!("--- Flat Data Extract (raw values) ---\n");

        let extract = client
            .get_flat_data_extract_by_measure_category_code()
            .measure_category_code(code)
            .skip(0) // Start from beginning
            .top(NonZeroU32::new(10).unwrap()) // Get 10 items
            .send()
            .await?;

        if let Some(data) = &extract.result.data {
            println!("Retrieved {} data items (showing first 5):", data.len());
            for item in data.iter().take(5) {
                println!(
                    "  - Measure: {}, Unit: {}, Value: {:?}",
                    *item.measure_code, *item.reporting_unit_code, item.value
                );
            }
        }

        // Get formatted data extract (display-ready values)
        println!("\n--- Formatted Data Extract (display values) ---\n");

        let formatted = client
            .get_flat_formatted_data_extract_by_measure_category_code()
            .measure_category_code(code)
            .skip(0)
            .top(NonZeroU32::new(10).unwrap())
            .send()
            .await?;

        if let Some(data) = &formatted.result.data {
            println!(
                "Retrieved {} formatted data items (showing first 5):",
                data.len()
            );
            for item in data.iter().take(5) {
                println!(
                    "  - Measure: {}, Unit: {}",
                    *item.measure_code, *item.reporting_unit_code
                );
                println!("    Formatted Value: {:?}", item.formatted_value);
            }
        }

        // Demonstrate pagination with skip/top
        println!("\n--- Pagination Demo ---\n");

        // Get first page
        let page1 = client
            .get_flat_data_extract_by_measure_category_code()
            .measure_category_code(code)
            .skip(0) // Start at 0
            .top(NonZeroU32::new(5).unwrap())
            .send()
            .await?;

        // Get second page
        let page2 = client
            .get_flat_data_extract_by_measure_category_code()
            .measure_category_code(code)
            .skip(5) // Skip first 5
            .top(NonZeroU32::new(5).unwrap())
            .send()
            .await?;

        if let (Some(d1), Some(d2)) = (&page1.result.data, &page2.result.data) {
            println!("Page 1 (skip=0, top=5): {} items", d1.len());
            if let Some(first) = d1.first() {
                println!("  First item measure: {}", *first.measure_code);
            }

            println!("Page 2 (skip=5, top=5): {} items", d2.len());
            if let Some(first) = d2.first() {
                println!("  First item measure: {}", *first.measure_code);
            }
        }

        // Demonstrate filtering options
        println!("\n--- Filtering Options ---\n");
        println!("The API supports filtering by:");
        println!(
            "  - reporting_unit_type_code: Filter by unit type (H=Hospital, S=State, N=National)"
        );
        println!("  - measure_code: Filter by specific measures");
        println!("  - reporting_unit_code: Filter by specific reporting units");
        println!("  - start_date / end_date: Filter by date range");

        // Example with filter - National level only
        let filtered = client
            .get_flat_data_extract_by_measure_category_code()
            .measure_category_code(code)
            .reporting_unit_type_code(vec!["N".to_string()]) // National only
            .skip(0)
            .top(NonZeroU32::new(10).unwrap())
            .send()
            .await?;

        if let Some(data) = &filtered.result.data {
            println!("\nFiltered to National level only: {} items", data.len());
            for item in data.iter().take(3) {
                println!(
                    "  - Measure: {}, Value: {:?}",
                    *item.measure_code, item.value
                );
            }
        }
    }

    Ok(())
}

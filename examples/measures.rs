//! Example: Working with measures and measure categories
//!
//! Run with: pixi run -- cargo run --example measures

use aihw_myhospitals_api::new_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    println!("=== Measure Categories API ===\n");

    // Get all measure categories
    println!("Fetching measure categories...");
    let response = client.get_measure_categories().send().await?;
    println!("Found {} categories:\n", response.result.len());

    for cat in response.result.iter().take(5) {
        println!("Code: {}", *cat.measure_category_code);
        println!("Name: {}", *cat.measure_category_name);
        println!();
    }

    // Get details of a specific category
    if let Some(first_cat) = response.result.first() {
        let code = &*first_cat.measure_category_code;
        println!("--- Category details: {} ---\n", code);

        let detail = client
            .get_measure_categories_by_measure_category_code()
            .measure_category_code(code)
            .send()
            .await?;

        println!("Name: {}", *detail.result.measure_category_name);

        // Get measures in this category
        println!("\n--- Measures in category {} ---\n", code);

        let measures = client
            .get_measure_categories_by_measure_category_code_measures()
            .measure_category_code(code)
            .send()
            .await?;

        println!("Found {} measures:", measures.result.len());
        for m in measures.result.iter().take(5) {
            println!("  - {}: {}", *m.measure_code, *m.measure_name);
        }
    }

    println!("\n=== Measures API ===\n");

    // Get all measures
    println!("Fetching all measures...");
    let response = client.get_measures().send().await?;
    println!("Found {} measures:\n", response.result.len());

    for measure in response.result.iter().take(3) {
        println!("Code: {}", *measure.measure_code);
        println!("Name: {}", *measure.measure_name);
        println!();
    }

    // Get details of a specific measure
    if let Some(first_measure) = response.result.first() {
        let code = &*first_measure.measure_code;
        println!("--- Measure details: {} ---\n", code);

        let detail = client
            .get_measures_by_measure_code()
            .measure_code(code)
            .send()
            .await?;

        println!("Name: {}", *detail.result.measure_name);
        println!("Units: {:?}", detail.result.units);
        println!("Categories: {:?}", detail.result.measure_categories.len());

        // Get data items for this measure
        println!("\n--- Data items for measure {} ---\n", code);

        let items = client
            .get_measures_by_measure_code_data_items()
            .measure_code(code)
            .send()
            .await?;

        println!("Found {} data items (showing first 5):", items.result.len());
        for item in items.result.iter().take(5) {
            println!(
                "  - Unit: {}, Value: {:?}, Dataset: {}",
                *item.reporting_unit_summary.reporting_unit_code, item.value, item.data_set_id
            );
        }

        // Get reporting units available for this measure
        println!("\n--- Reporting units for measure {} ---\n", code);

        let units = client
            .get_measures_by_measure_code_reporting_units_available()
            .measure_code(code)
            .send()
            .await?;

        println!("Available at {} reporting units (showing first 5):", units.result.len());
        for unit in units.result.iter().take(5) {
            println!("  - {}: {}", *unit.reporting_unit_code, *unit.reporting_unit_name);
        }
    }

    println!("\n=== Reported Measure Categories API ===\n");

    // Get reported measure categories
    let response = client.get_reported_measure_categories().send().await?;
    println!("Found {} reported measure categories:\n", response.result.len());

    for cat in response.result.iter().take(3) {
        println!("Code: {}", *cat.reported_measure_category_code);
        println!("Name: {}", *cat.reported_measure_category_name);
        println!();
    }

    // Get details and reported measures for first category
    if let Some(first_cat) = response.result.first() {
        let code = &*first_cat.reported_measure_category_code;
        println!("--- Reported measures in category {} ---\n", code);

        let measures = client
            .get_reported_measure_categories_by_reported_measure_category_code_reported_measures()
            .reported_measure_category_code(code)
            .send()
            .await?;

        println!("Found {} reported measures:", measures.result.len());
        for m in measures.result.iter().take(5) {
            println!("  - {}: {}", *m.reported_measure_code, *m.reported_measure_name);
        }
    }

    println!("\n=== Reported Measures API ===\n");

    // Get all reported measures
    let response = client.get_reported_measures().send().await?;
    println!("Found {} reported measures:\n", response.result.len());

    // Get details for first reported measure
    if let Some(first) = response.result.first() {
        let code = &*first.reported_measure_code;
        println!("--- Reported measure details: {} ---\n", code);

        let detail = client
            .get_reported_measures_by_reported_measure_code()
            .reported_measure_code(code)
            .send()
            .await?;

        println!("Name: {}", *detail.result.reported_measure_name);

        // Get data items
        let items = client
            .get_reported_measures_by_reported_measure_code_data_items()
            .reported_measure_code(code)
            .send()
            .await?;

        println!("Found {} data items", items.result.len());
    }

    Ok(())
}

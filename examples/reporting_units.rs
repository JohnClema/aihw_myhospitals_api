//! Example: Working with reporting units (hospitals, states, national)
//!
//! Run with: pixi run -- cargo run --example reporting_units

use aihw_myhospitals_api::new_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    println!("=== Reporting Unit Types API ===\n");

    // Get all reporting unit types
    println!("Fetching reporting unit types...");
    let response = client.get_reporting_unit_types().send().await?;
    println!("Found {} types:\n", response.result.len());

    for unit_type in &response.result {
        println!("Code: {}", *unit_type.reporting_unit_type_code);
        println!("Name: {}", *unit_type.reporting_unit_type_name);
        println!();
    }

    // Get details of first type
    if let Some(first_type) = response.result.first() {
        let code = &*first_type.reporting_unit_type_code;
        println!("--- Type details: {} ---\n", code);

        let detail = client
            .get_reporting_unit_types_by_reporting_unit_type_code()
            .reporting_unit_type_code(code)
            .send()
            .await?;

        println!("Name: {}", *detail.result.reporting_unit_type_name);

        // Get available bricks (geographic areas) for this type
        // Note: The result is a HashMap<String, Vec<String>>
        println!("\n--- Available bricks for type {} ---\n", code);

        let bricks = client
            .get_reporting_unit_types_by_reporting_unit_type_code_bricks_available()
            .reporting_unit_type_code(code)
            .send()
            .await?;

        println!("Found {} brick categories:", bricks.result.len());
        for (category, values) in bricks.result.iter().take(3) {
            println!("  Category '{}': {} values", category, values.len());
            for v in values.iter().take(3) {
                println!("    - {}", v);
            }
        }
    }

    println!("\n=== Reporting Units API ===\n");

    // Get all reporting units
    println!("Fetching reporting units...");
    let response = client.get_reporting_units().send().await?;
    println!("Found {} reporting units:\n", response.result.len());

    for unit in response.result.iter().take(5) {
        println!("Code: {}", *unit.reporting_unit_code);
        println!("Name: {}", *unit.reporting_unit_name);
        println!(
            "Type: {}",
            *unit.reporting_unit_type.reporting_unit_type_code
        );
        println!();
    }

    // Get details of a specific reporting unit (find a hospital)
    let hospital = response
        .result
        .iter()
        .find(|u| *u.reporting_unit_type.reporting_unit_type_code == "H");

    if let Some(hospital) = hospital {
        let code = &*hospital.reporting_unit_code;
        println!("--- Hospital details: {} ---\n", code);

        let detail = client
            .get_reporting_units_by_reporting_unit_code()
            .reporting_unit_code(code)
            .send()
            .await?;

        println!("Name: {}", *detail.result.reporting_unit_name);
        println!(
            "Type: {}",
            *detail.result.reporting_unit_type.reporting_unit_type_code
        );
        println!("Private: {}", detail.result.private);
        println!("Closed: {}", detail.result.closed);
        println!("Latitude: {:?}", detail.result.latitude);
        println!("Longitude: {:?}", detail.result.longitude);

        // Show meta tags
        if !detail.result.meta_tags.is_empty() {
            println!("\nMeta tags:");
            for tag in &detail.result.meta_tags {
                println!(
                    "  - {}: {}",
                    *tag.meta_tag_type.meta_tag_type_name, *tag.meta_tag_name
                );
            }
        }

        // Get data items for this hospital
        println!("\n--- Data items for {} ---\n", code);

        let items = client
            .get_reporting_units_by_reporting_unit_code_data_items()
            .reporting_unit_code(code)
            .send()
            .await?;

        println!("Found {} data items (showing first 5):", items.result.len());
        for item in items.result.iter().take(5) {
            println!(
                "  - Measure: {}, Value: {:?}",
                *item.measure_code, item.value
            );
        }

        // Get measures available for this hospital
        println!("\n--- Measures available for {} ---\n", code);

        let measures = client
            .get_reporting_units_by_reporting_unit_code_measures_available()
            .reporting_unit_code(code)
            .send()
            .await?;

        println!(
            "Found {} available measures (showing first 5):",
            measures.result.len()
        );
        for m in measures.result.iter().take(5) {
            println!("  - {}: {}", *m.measure_code, *m.measure_name);
        }

        // Get available bricks (geographic breakdown)
        // Note: The result is a Vec<String> for reporting units
        println!("\n--- Available bricks for {} ---\n", code);

        let bricks = client
            .get_reporting_units_by_reporting_unit_code_bricks_available()
            .reporting_unit_code(code)
            .send()
            .await?;

        println!("Found {} bricks:", bricks.result.len());
        for brick in bricks.result.iter().take(5) {
            println!("  - {}", brick);
        }
    }

    Ok(())
}

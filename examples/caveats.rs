//! Example: Working with caveats (footnotes and suppressions)
//!
//! Run with: pixi run -- cargo run --example caveats

use aihw_myhospitals_api::new_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    println!("=== Caveats API ===\n");

    // Get all caveats (footnotes)
    println!("Fetching all caveats...");
    let response = client.get_caveats().send().await?;
    println!("Found {} caveats:\n", response.result.len());

    for caveat in response.result.iter().take(5) {
        println!("Code: {}", *caveat.caveat_code);
        println!("Name: {}", *caveat.caveat_name);
        println!("Display Value: {:?}", caveat.caveat_display_value);
        println!("Footnote: {:?}", caveat.caveat_footnote);
        println!();
    }

    // Get a specific caveat by code
    if let Some(first_caveat) = response.result.first() {
        let code = &*first_caveat.caveat_code;
        println!("--- Fetching specific caveat: {} ---", code);
        let detail = client.get_caveats_by_caveat_code().caveat_code(code).send().await?;
        println!("Caveat name: {}", *detail.result.caveat_name);
        println!("Caveat footnote: {:?}", detail.result.caveat_footnote);
    }

    println!("\n=== Suppressions API (Deprecated - use Caveats) ===\n");

    // Get all suppressions
    println!("Fetching all suppressions...");
    let response = client.get_suppressions().send().await?;
    println!("Found {} suppressions:\n", response.result.len());

    for suppression in response.result.iter().take(5) {
        println!("Code: {}", *suppression.suppression_code);
        println!("Name: {}", *suppression.suppression_name);
        println!("Display Value: {:?}", suppression.suppression_display_value);
        println!("Footnote: {:?}", suppression.suppression_footnote);
        println!();
    }

    // Get a specific suppression by code
    if let Some(first_suppression) = response.result.first() {
        let code = &*first_suppression.suppression_code;
        println!("--- Fetching specific suppression: {} ---", code);
        let detail = client
            .get_suppressions_by_suppression_code()
            .suppression_code(code)
            .send()
            .await?;
        println!("Suppression name: {}", *detail.result.suppression_name);
        println!("Suppression footnote: {:?}", detail.result.suppression_footnote);
    }

    Ok(())
}

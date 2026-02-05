# aihw_myhospitals_api

A Rust client for the [MyHospitals Data API](https://myhospitalsapi.aihw.gov.au/) provided by the Australian Institute of Health and Welfare (AIHW).

## Features

- Auto-generated from OpenAPI specification using [Progenitor](https://github.com/oxidecomputer/progenitor)
- Async/await support with Tokio
- Builder pattern for ergonomic API calls
- Strongly typed requests and responses
- 38 API endpoints covering all MyHospitals data

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
aihw_myhospitals_api = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use aihw_myhospitals_api::{Client, new_client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using the convenience function
    let client = new_client();

    // Or with explicit URL
    let client = Client::new("https://myhospitalsapi.aihw.gov.au");

    // Fetch measure categories
    let response = client.get_measure_categories().send().await?;
    println!("Found {} categories", response.result.len());

    // Fetch datasets
    let response = client.get_datasets().send().await?;
    for dataset in &response.result {
        println!("{}: {} to {}",
            *dataset.data_set_name,
            dataset.reporting_start_date,
            dataset.reporting_end_date
        );
    }

    Ok(())
}
```

## API Coverage

The client provides access to all MyHospitals API endpoints:

| Category | Endpoints | Description |
|----------|-----------|-------------|
| **Caveats** | 2 | Data footnotes and suppressions |
| **Suppressions** | 2 | Legacy suppression data (use Caveats) |
| **DataSets** | 3 | Grouped data periods |
| **Measures** | 4 | Health metrics and indicators |
| **MeasureCategories** | 3 | Measure classifications |
| **ReportedMeasures** | 4 | Reported measurement forms |
| **ReportedMeasureCategories** | 3 | Reported measure groupings |
| **ReportingUnits** | 5 | Hospitals and health services |
| **ReportingUnitTypes** | 3 | Unit classifications (Hospital, State, National) |
| **FlatDataExtracts** | 2 | Bulk data retrieval (paginated, max 1000) |
| **Downloads** | 7 | Excel/XLSX exports |

## Examples

Run the examples to explore the API:

```bash
# Basic usage
cargo run --example basic_usage

# Caveats and suppressions
cargo run --example caveats

# Datasets and data items
cargo run --example datasets

# Measures and categories
cargo run --example measures

# Reporting units (hospitals, states)
cargo run --example reporting_units

# Bulk data extracts with pagination
cargo run --example data_extracts

# Excel download codes
cargo run --example downloads
```

## Response Structure

All API responses are wrapped in an envelope containing:
- `result` - The actual data
- `version_information` - API version and data version metadata

```rust
let response = client.get_reporting_unit_types().send().await?;

// Access the data
for unit_type in &response.result {
    println!("{}", *unit_type.reporting_unit_type_name);
}

// Access version info
if let Some(info) = &response.version_information {
    println!("API Version: {:?}", info.api_version);
}
```

## Newtype Wrappers

Many fields use newtype wrappers for type safety. Dereference with `*` to access the underlying string:

```rust
let response = client.get_measures().send().await?;
for measure in &response.result {
    // Use * to dereference the newtype wrapper
    println!("Code: {}", *measure.measure_code);
    println!("Name: {}", *measure.measure_name);
}
```

## Pagination

The flat data extract endpoints use skip/top pagination:

```rust
use std::num::NonZeroU32;

let response = client
    .get_flat_data_extract_by_measure_category_code()
    .measure_category_code("ED")
    .skip(0)                              // Start from beginning
    .top(NonZeroU32::new(100).unwrap())   // Get 100 items (max 1000)
    .send()
    .await?;
```

## File Downloads

Download endpoints return `ByteStream`. Collect to bytes:

```rust
use futures::StreamExt;

let response = client
    .get_simple_downloads_by_download_code()
    .download_code("some-code")
    .send()
    .await?;

let mut bytes = Vec::new();
let mut stream = response.into_inner();
while let Some(chunk) = stream.next().await {
    bytes.extend_from_slice(&chunk?);
}
std::fs::write("output.xlsx", &bytes)?;
```

## Development

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests (requires network)
cargo test --features integration-tests
```

### Debug Build Output

Set `DEBUG_CODEGEN=1` to see generated code paths:

```bash
DEBUG_CODEGEN=1 cargo build
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Releasing

Releases are automated via GitHub Actions. To create a new release:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with the new version
3. Commit: `git commit -am "Release v0.x.y"`
4. Tag: `git tag v0.x.y`
5. Push: `git push && git push --tags`

The release workflow will:
- Run all CI checks
- Publish to crates.io
- Create a GitHub release with changelog notes

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>).

## Data License

Data from the MyHospitals API is licensed under [CC-BY 3.0](https://creativecommons.org/licenses/by/3.0/) by the Australian Institute of Health and Welfare.

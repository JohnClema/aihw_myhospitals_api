# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- R: `MyHospitalsClient` is now exported, so `library(aihwmyhospitals)` makes it
  available as documented.
- R: optional arguments have their documented defaults (`base_url` and filters
  default to `NULL`; flat extracts default to `skip = 0L`, `top = 100L`), so
  calls like `MyHospitalsClient$new()` and `client$get_datasets()` work.
- R: a failed API call no longer crashes the R session or leaks memory. Errors
  are raised from R as conditions of class `myhospitals_error`, instead of
  from Rust with the message used as a printf format (a `%` in a URL or
  response body could segfault R).

## [0.1.0] - 2026-02-05

### Added

- Initial release
- Auto-generated API client from OpenAPI specification using Progenitor
- Full coverage of all 38 MyHospitals API endpoints:
  - Caveats and Suppressions
  - DataSets and Data Items
  - Measures and Measure Categories
  - Reported Measures and Categories
  - Reporting Units and Types
  - Flat Data Extracts (paginated)
  - Excel/XLSX Downloads
- Async/await support with Tokio
- Builder pattern for ergonomic API calls
- Strongly typed requests and responses
- Response envelope with version information
- Proper date handling (NaiveDate for date fields)
- Integration tests (behind `integration-tests` feature flag)
- Comprehensive documentation and examples

[Unreleased]: https://github.com/johnclema/aihw_myhospitals_api/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/johnclema/aihw_myhospitals_api/releases/tag/v0.1.0

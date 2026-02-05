# Contributing to aihw_myhospitals_api

Thank you for your interest in contributing!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/aihw_myhospitals_api`
3. Create a branch: `git checkout -b my-feature`

## Development

### Prerequisites

- Rust 1.75 or later
- (Optional) [pixi](https://pixi.sh) for reproducible environment

### Building

```bash
cargo build
```

### Testing

```bash
# Unit tests
cargo test

# Integration tests (requires network access to MyHospitals API)
cargo test --features integration-tests
```

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Add documentation for public items

## Pull Requests

1. Update `CHANGELOG.md` under `[Unreleased]` with your changes
2. Ensure all tests pass
3. Update documentation if needed
4. Submit your PR with a clear description

## Updating the OpenAPI Spec

If the upstream MyHospitals API changes:

1. Download the new spec from https://myhospitalsapi.aihw.gov.au/swagger/v1/swagger.json
2. Save to `spec/swagger.json`
3. Run `cargo build` to regenerate the client
4. Fix any compilation errors in `build.rs` if the spec structure changed
5. Update examples and tests as needed

## Questions?

Open an issue for any questions or concerns.

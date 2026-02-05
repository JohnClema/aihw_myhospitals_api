## R CMD check results

0 errors | 2 warnings | 2 notes

### Warnings

1. **Non-API calls to R** (BODY, CLOENV, DATAPTR, ENCLOS, FORMALS) - These
   come from the extendr framework (https://extendr.github.io/), the standard
   way to create R extensions in Rust. The extendr project is actively working
   on moving to the new R C API. This is consistent with other extendr-based
   CRAN packages (e.g., polars, gifski, string2path).

2. **Rust compilation warning** - "No rustc version reported" - We now report
   rustc version in Makevars before compilation begins.

### Notes

1. **New submission** - This is a new package.

2. **Possibly misspelled words** - AIHW (Australian Institute of Health and
   Welfare) and MyHospitals are proper names, not misspellings.

3. **Package size** (~22MB tarball, ~11MB installed) - This package includes
   vendored Rust dependencies (vendor.tar.xz) required for CRAN's offline
   compilation environment. The installed size is primarily from the compiled
   Rust binary. This is typical for Rust-based packages.

## Rust requirements

This package requires Rust (cargo >= 1.70 and rustc >= 1.70) to compile
from source. The `configure` script checks for these tools and provides
installation instructions if they are not found.

Users can install Rust from https://rustup.rs

## Test environments

* local macOS Tahoe 26.3 (aarch64-apple-darwin), R 4.5.2
* win-builder (R-devel, Windows Server 2022)
* Debian Linux (R-devel)

## Downstream dependencies

This is a new package with no reverse dependencies.

## R CMD check results

0 errors | 0 warnings | 2 notes

### Notes

1. **New submission** - This is a new package.

2. **Package size** (~22MB tarball, ~11MB installed) - This package includes
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

## Downstream dependencies

This is a new package with no reverse dependencies.

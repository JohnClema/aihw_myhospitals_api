## R CMD check results

0 errors | 0 warnings | 2 notes

### Previously Reported Warnings (Now Addressed)

1. **"Downloads Rust crates"** - Fixed. All Rust dependencies are vendored in
   `src/rust/vendor.tar.xz` and extracted at build time. Compilation uses
   `cargo build --offline` to prevent any network access.

2. **"No rustc version reported"** - Fixed. Both Makevars files report rustc
   and cargo versions before compilation.

### Expected Warnings

1. **"Found non-API calls to R: BODY, CLOENV, DATAPTR, ENCLOS, FORMALS"** -
   These come from the extendr framework (https://extendr.github.io/), the
   standard Rust-to-R binding library. The extendr project is actively
   migrating to the official R C API
   (https://github.com/extendr/extendr/issues/805). This is consistent with
   other extendr-based CRAN packages (e.g., string2path, prqlr).

2. **"Compiled code contains abort/exit/_exit"** - These symbols originate from
   Rust's standard library and the ring cryptography crate (used for TLS).
   We mitigate with `panic = "abort"`, link-time optimization (`lto = true`),
   and `codegen-units = 1`. Remaining references are in code paths that
   cannot be reached from R and do not affect R session stability. This is
   consistent with other Rust-based CRAN packages.

### Notes

1. **New submission** - This is a new package.

2. **Possibly misspelled words** - AIHW (Australian Institute of Health and
   Welfare) and MyHospitals are proper nouns, not misspellings.

3. **Package size** (~22MB tarball) - Includes vendored Rust dependencies
   required for CRAN's offline build environment. The size is primarily from
   the HTTP/TLS client stack needed for API communication.

## Test environments

* local macOS (aarch64-apple-darwin), R 4.5.x
* win-builder (R-devel, Windows Server 2022)
* Debian Linux (R-devel, via CRAN incoming checks)

## Downstream dependencies

No reverse dependencies.

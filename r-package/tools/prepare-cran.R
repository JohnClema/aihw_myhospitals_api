#!/usr/bin/env Rscript
# prepare-cran.R
# Script to prepare the aihwmyhospitals package for CRAN submission
#
# Usage:
#   Rscript tools/prepare-cran.R
#   # or from R:
#   source("tools/prepare-cran.R")

# Ensure we're in the r-package directory
if (!file.exists("DESCRIPTION")) {


  # Try to find it

  if (file.exists("r-package/DESCRIPTION")) {
    setwd("r-package")
  } else {
    stop("Please run this script from the r-package directory or its parent")
  }
}

cat("=== CRAN Preparation Script ===\n\n")

# Step 1: Check for required packages
cat("Step 1: Checking required packages...\n")

required_pkgs <- c("rextendr", "devtools", "roxygen2")
missing_pkgs <- required_pkgs[!sapply(required_pkgs, requireNamespace, quietly = TRUE)]

if (length(missing_pkgs) > 0) {


  cat("Installing missing packages:", paste(missing_pkgs, collapse = ", "), "\n")
  install.packages(missing_pkgs)
}

library(rextendr)
library(devtools)

cat("   Done.\n\n")

# Step 2: Vendor Rust dependencies
cat("Step 2: Vendoring Rust dependencies...\n")
cat("   This bundles all Rust crates for offline CRAN builds.\n")

tryCatch({
  rextendr::vendor_pkgs(path = ".")
  cat("   Created vendor.tar.xz and inst/AUTHORS\n")
}, error = function(e) {
  cat("   Warning: vendor_pkgs failed:", conditionMessage(e), "\n")
  cat("   You may need to run this manually.\n")
})

cat("   Done.\n\n")

# Step 3: Generate documentation
cat("Step 3: Generating documentation...\n")

tryCatch({
  devtools::document()
  cat("   Generated man/ files and NAMESPACE\n")
}, error = function(e) {

  cat("   Warning: document() failed:", conditionMessage(e), "\n")
})

cat("   Done.\n\n")

# Step 4: Build the package
cat("Step 4: Building package...\n")

pkg_name <- read.dcf("DESCRIPTION")[1, "Package"]
pkg_version <- read.dcf("DESCRIPTION")[1, "Version"]
tarball <- paste0(pkg_name, "_", pkg_version, ".tar.gz")

# Move up one directory to build
original_wd <- getwd()
setwd("..")

tryCatch({
  system2("R", args = c("CMD", "build", "r-package"), stdout = TRUE, stderr = TRUE)
  cat("   Built:", tarball, "\n")
}, error = function(e) {
  cat("   Warning: R CMD build failed:", conditionMessage(e), "\n")
})

cat("   Done.\n\n")

# Step 5: Run CRAN check
cat("Step 5: Running R CMD check --as-cran...\n")
cat("   This may take several minutes...\n\n")

if (file.exists(tarball)) {
  check_result <- system2(
    "R",
    args = c("CMD", "check", "--as-cran", tarball),
    stdout = TRUE,
    stderr = TRUE
  )

  # Print check output
  cat(check_result, sep = "\n")

  # Look for errors/warnings
  has_errors <- any(grepl("ERROR", check_result))
  has_warnings <- any(grepl("WARNING", check_result))
  has_notes <- any(grepl("NOTE", check_result))

  cat("\n")
  cat("=== Check Summary ===\n")
  if (has_errors) cat("   ERRORS found - must fix before submission\n")
  if (has_warnings) cat("   WARNINGS found - should fix before submission\n")
  if (has_notes) cat("   NOTES found - review before submission\n")
  if (!has_errors && !has_warnings && !has_notes) {
    cat("   All clear! Ready for CRAN submission.\n")
  }
} else {
  cat("   Skipped: tarball not found\n")
}

setwd(original_wd)

cat("\n=== Preparation Complete ===\n")
cat("\nNext steps:\n")
cat("1. Review any errors/warnings from R CMD check\n")
cat("2. Update cran-comments.md with check results\n
")
cat("3. Submit to CRAN:\n")
cat("   devtools::submit_cran()\n")
cat("   # or upload manually at https://cran.r-project.org/submit.html\n")

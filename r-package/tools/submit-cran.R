#!/usr/bin/env Rscript
# submit-cran.R
# Script to submit the aihwmyhospitals package to CRAN
#
# Usage:
#   Rscript tools/submit-cran.R
#   # or via pixi:
#   pixi run submit-cran

# Ensure we're in the r-package directory
if (!file.exists("DESCRIPTION")) {
  if (file.exists("r-package/DESCRIPTION")) {
    setwd("r-package")
  } else {
    stop("Please run this script from the r-package directory or its parent")
  }
}

cat("=== CRAN Submission Script ===\n\n")

# Check if devtools is available
if (!requireNamespace("devtools", quietly = TRUE)) {
  stop("devtools is required. Install with: install.packages('devtools')")
}

library(devtools)

# Read package info
pkg_name <- read.dcf("DESCRIPTION")[1, "Package"]
pkg_version <- read.dcf("DESCRIPTION")[1, "Version"]

cat("Package:", pkg_name, "\n")
cat("Version:", pkg_version, "\n\n")

# Check for required files
cat("Checking required files...\n")
required_files <- c(

"DESCRIPTION",
  "NAMESPACE",
  "LICENSE",
  "configure",
  "cleanup",
  "cran-comments.md",
  "src/rust/vendor.tar.xz"
)

missing <- required_files[!file.exists(required_files)]
if (length(missing) > 0) {
  cat("\nMissing required files:\n")
  cat(paste(" -", missing, collapse = "\n"), "\n")
  cat("\nPlease run 'pixi run prepare-cran' first.\n")
  stop("Missing required files for CRAN submission")
}
cat("   All required files present.\n\n")

# Check cran-comments.md
cat("Contents of cran-comments.md:\n")
cat("---\n")
cat(readLines("cran-comments.md"), sep = "\n")
cat("\n---\n\n")

# Confirm submission
cat("Ready to submit to CRAN.\n\n")
cat("This will:\n")
cat("  1. Build the package tarball\n")
cat("  2. Upload to CRAN's submission portal\n")
cat("  3. Send confirmation email to maintainer\n\n")

# Interactive confirmation
if (interactive()) {
  response <- readline("Do you want to proceed? (yes/no): ")
  if (tolower(response) != "yes") {
    cat("Submission cancelled.\n")
    quit(status = 0)
  }
} else {
  # Non-interactive mode - check for --yes flag
  args <- commandArgs(trailingOnly = TRUE)
  if (!("--yes" %in% args)) {
    cat("Non-interactive mode. Use --yes flag to confirm submission.\n")
    cat("Example: Rscript tools/submit-cran.R --yes\n")
    quit(status = 1)
  }
}

# Submit to CRAN
cat("\nSubmitting to CRAN...\n")
tryCatch({
  devtools::submit_cran(pkg = ".")
  cat("\n=== Submission Complete ===\n")
  cat("\nNext steps:\n")
  cat("1. Check your email for CRAN confirmation\n")
  cat("2. Click the confirmation link in the email\n")
  cat("3. Wait for CRAN review (typically 1-5 days)\n")
  cat("4. Address any feedback from CRAN maintainers\n")
}, error = function(e) {
  cat("\nSubmission failed:\n")
  cat(conditionMessage(e), "\n")
  cat("\nYou can also submit manually at:\n")
  cat("  https://cran.r-project.org/submit.html\n")
  quit(status = 1)
})

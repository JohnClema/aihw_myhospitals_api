# Tests for aihwmyhospitals client
# These tests require network access to the AIHW MyHospitals API

test_that("MyHospitalsClient is exported", {
  expect_true("MyHospitalsClient" %in% getNamespaceExports("aihwmyhospitals"))
})

test_that("client can be created", {
  client <- MyHospitalsClient$new(NULL)
  expect_true(!is.null(client))
})

test_that("client can be created with no arguments", {
  client <- MyHospitalsClient$new()
  expect_s3_class(client, "MyHospitalsClient")
})

test_that("optional arguments have the documented defaults", {
  client <- MyHospitalsClient$new()

  # A NULL default, as opposed to no default (the empty symbol).
  has_null_default <- function(f, arg) {
    arg %in% names(formals(f)) && is.null(formals(f)[[arg]])
  }

  expect_true(has_null_default(MyHospitalsClient$new, "base_url"))
  expect_true(has_null_default(client$get_datasets, "measure_code"))
  expect_true(has_null_default(client$get_datasets, "reported_measure_code"))
  expect_true(has_null_default(client$get_measures, "measure_category_code"))
  expect_true(has_null_default(client$get_reported_measures, "reported_measure_category_code"))
  expect_true(has_null_default(client$get_reporting_units, "reporting_unit_type_code"))

  for (method in c("get_flat_data_extract", "get_flat_formatted_data_extract")) {
    f <- client[[method]]
    expect_identical(formals(f)$skip, 0L)
    expect_identical(formals(f)$top, 100L)
    for (arg in c("reporting_unit_type_code", "measure_code", "reporting_unit_code",
                  "start_date", "end_date")) {
      expect_true(has_null_default(f, arg), info = paste(method, arg))
    }
  }
})

test_that("client can be created with custom URL", {
  client <- MyHospitalsClient$new("https://myhospitalsapi.aihw.gov.au")
  expect_true(!is.null(client))
})

# Integration tests (require network access)
# Skip if not running integration tests
skip_if_not_installed <- function() {
  skip_on_cran()
  skip_if_offline()
}

test_that("get_measure_categories returns data", {
  skip_if_not_installed()

  client <- MyHospitalsClient$new(NULL)
  result <- client$get_measure_categories()

  expect_type(result, "list")
  expect_true("result" %in% names(result))
})

test_that("get_caveats returns data", {
  skip_if_not_installed()

  client <- MyHospitalsClient$new(NULL)
  result <- client$get_caveats()

  expect_type(result, "list")
  expect_true("result" %in% names(result))
})

test_that("get_suppressions returns data", {
  skip_if_not_installed()

  client <- MyHospitalsClient$new(NULL)
  result <- client$get_suppressions()

  expect_type(result, "list")
  expect_true("result" %in% names(result))
})

test_that("get_datasets returns data", {
  skip_if_not_installed()

  client <- MyHospitalsClient$new(NULL)
  result <- client$get_datasets(NULL, NULL)

  expect_type(result, "list")
  expect_true("result" %in% names(result))
})

test_that("get_reporting_unit_types returns data", {
  skip_if_not_installed()

  client <- MyHospitalsClient$new(NULL)
  result <- client$get_reporting_unit_types()

  expect_type(result, "list")
  expect_true("result" %in% names(result))
})

test_that("get_measures returns data", {
  skip_if_not_installed()

  client <- MyHospitalsClient$new(NULL)
  result <- client$get_measures(NULL)

  expect_type(result, "list")
  expect_true("result" %in% names(result))
})

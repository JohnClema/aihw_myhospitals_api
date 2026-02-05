# Tests for aihwmyhospitals client
# These tests require network access to the AIHW MyHospitals API

test_that("client can be created", {
  client <- MyHospitalsClient$new(NULL)
  expect_true(!is.null(client))
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

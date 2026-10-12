# Failed calls must raise ordinary, catchable R errors. These tests need no
# network: port 1 on localhost refuses connections, so every call fails fast.

unreachable_client <- function() MyHospitalsClient$new("http://127.0.0.1:1")

test_that("failed calls raise a classed R error", {
  client <- unreachable_client()

  err <- expect_error(client$get_caveats(), class = "myhospitals_error")
  expect_match(conditionMessage(err), "^API error: ")
  expect_s3_class(err, "error")

  caught <- tryCatch(client$get_caveats(), myhospitals_error = function(e) "caught")
  expect_identical(caught, "caught")
})

test_that("printf directives in a request don't crash R", {
  # The percent-encoded request URL is part of the error message, which used
  # to be passed to Rf_error() as a printf format: "ED stays" (sent as
  # "ED%20stays") and "%s" segfaulted, and "%p" printed pointers.
  client <- unreachable_client()

  for (code in c("x%p%p%p", "ED stays", "%s%s%s%n", "Café", "a/b")) {
    err <- expect_error(
      client$get_measure_by_code(code),
      class = "myhospitals_error",
      info = code
    )
    expect_match(conditionMessage(err), "^API error: ", info = code)
  }

  err <- expect_error(client$get_measure_by_code("x%p%p%p"), class = "myhospitals_error")
  expect_match(conditionMessage(err), "x%25p%25p%25p", fixed = TRUE)
})

test_that("the error call is the method call", {
  client <- unreachable_client()
  err <- expect_error(client$get_caveats(), class = "myhospitals_error")
  expect_match(deparse(conditionCall(err)), "get_caveats")
})

test_that("download methods raise the same error", {
  client <- unreachable_client()
  expect_error(client$download_simple("x"), class = "myhospitals_error")
})

test_that("errors don't break the client for later calls", {
  client <- unreachable_client()
  for (i in 1:3) {
    expect_error(client$get_caveats(), class = "myhospitals_error")
  }
  expect_s3_class(client, "MyHospitalsClient")
})

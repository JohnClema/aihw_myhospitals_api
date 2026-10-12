# Failed calls must raise ordinary, catchable R errors. These tests need no
# network (see unreachable_client() in helper-client.R).

test_that("failed calls raise a classed R error", {
  client <- unreachable_client()

  expect_error(client$get_caveats(), "^API error: ", class = "myhospitals_error")

  caught <- tryCatch(client$get_caveats(), myhospitals_error = function(e) "caught")
  expect_identical(caught, "caught")
})

test_that("the message includes the underlying cause", {
  client <- unreachable_client()
  # e.g. "error sending request: client error (Connect): tcp connect error:
  # Connection refused (os error 61)", not just "error sending request".
  expect_error(
    client$get_caveats(),
    "error sending request: ",
    fixed = TRUE,
    class = "myhospitals_error"
  )
})

test_that("printf directives in a request don't crash R", {
  # The percent-encoded request URL is part of the error message, which used
  # to be passed to Rf_error() as a printf format: "ED stays" (sent as
  # "ED%20stays") and "%s" segfaulted, and "%p" printed pointers.
  client <- unreachable_client()

  for (code in c("x%p%p%p", "ED stays", "%s%s%s%n", "Caf\u00e9", "a/b")) {
    expect_error(
      client$get_measure_by_code(code),
      "^API error: ",
      class = "myhospitals_error",
      info = code
    )
  }

  expect_error(
    client$get_measure_by_code("x%p%p%p"),
    "x%25p%25p%25p",
    fixed = TRUE,
    class = "myhospitals_error"
  )
})

test_that("a % in an argument's class can't reach a printf format", {
  # For some argument errors extendr includes the argument's class in the
  # message it passes to Rf_error() as a printf format. This used to segfault.
  client <- unreachable_client()

  expect_error(client$get_flat_data_extract("X", top = structure(-1L, class = "%s%s%s%s%s%s")))

  err <- expect_error(client$get_dataset_by_id(structure(1.5, class = "x%p%p%p")))
  expect_false(grepl("0x", conditionMessage(err), fixed = TRUE))
})

test_that("errors report the user's call", {
  client <- unreachable_client()

  err <- expect_error(client$get_caveats(), class = "myhospitals_error")
  expect_identical(conditionCall(err), quote(client$get_caveats()))

  # Argument errors are raised by extendr during the call into Rust.
  err <- expect_error(client$get_measure_by_code(c("a", "b")))
  expect_identical(conditionCall(err), quote(client$get_measure_by_code(c("a", "b"))))
})

test_that("a client restored from disk raises a classed error", {
  # Serializing an external pointer drops its address, as saveRDS() does.
  restored <- unserialize(serialize(MyHospitalsClient$new(), NULL))

  err <- expect_error(
    restored$get_caveats(),
    "no longer valid",
    class = "myhospitals_error"
  )
  expect_identical(conditionCall(err), quote(restored$get_caveats()))
})

test_that("download methods raise the same error", {
  client <- unreachable_client()
  expect_error(client$download_simple("x"), class = "myhospitals_error")
})

test_that("a trailing slash in the base URL doesn't double the path separator", {
  client <- unreachable_client("http://127.0.0.1:1/")
  expect_error(
    client$get_caveats(),
    "(url: http://127.0.0.1:1/api/v1/caveats)",
    fixed = TRUE,
    class = "myhospitals_error"
  )
})

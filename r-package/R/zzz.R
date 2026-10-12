# Raise API errors from R rather than from Rust.
#
# The Rust methods return a failed call as a condition of class
# `myhospitals_error` instead of raising it: raising an R error from Rust
# longjmps over Rust frames, skipping their destructors, and extendr's
# `throw_r_error()` passes the message to `Rf_error()` as a printf format.
#
# This file must be sourced after `extendr-wrappers.R`, which defines the
# `MyHospitalsClient` environment. Package files are collated alphabetically,
# hence the name.

#' Signal an error condition returned by a Rust method, else return the value
#' @noRd
.raise_if_error <- function(x) {
  if (inherits(x, "myhospitals_error")) {
    x$call <- sys.call(-1)
    stop(x)
  }
  x
}

local({
  for (name in ls(MyHospitalsClient)) {
    method <- MyHospitalsClient[[name]]
    body(method) <- bquote(.raise_if_error(.(body(method))))
    MyHospitalsClient[[name]] <- method
  }
})

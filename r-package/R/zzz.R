# Raise errors from R rather than from Rust.
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

#' Stop if `self` no longer points at a Rust client
#'
#' A client restored from disk (`saveRDS()`, a saved workspace) or copied to
#' another R process holds a NULL pointer. Calling into Rust with it would make
#' extendr raise an error from inside Rust, leaking the call's arguments.
#' @noRd
.check_client <- function(self) {
  if (!client_is_live(self)) {
    stop(structure(
      class = c("myhospitals_error", "error", "condition"),
      list(
        message = paste(
          "This client is no longer valid: it was saved and restored, or",
          "copied to another R process. Create a new one with",
          "MyHospitalsClient$new()."
        ),
        call = sys.call(-1)
      )
    ))
  }
}

local({
  for (name in ls(MyHospitalsClient)) {
    method <- MyHospitalsClient[[name]]
    call <- body(method)
    # Only rewrite plain .Call() bodies. extendr wraps methods that return `()`
    # in invisible(); those can't return an error condition.
    if (!identical(call[[1]], quote(.Call))) next

    # extendr reports some argument errors through Rf_error() with the
    # argument's class in the message, and Rf_error() treats the message as a
    # printf format, so a class containing "%" could crash R. Drop classes
    # first. (call[[2]] is the native routine.)
    for (i in seq_along(call)[-(1:2)]) {
      if (!identical(call[[i]], quote(self))) {
        call[[i]] <- bquote(unclass(.(call[[i]])))
      }
    }

    # Run the .Call() in the method's own frame, so that errors raised during
    # it (such as an argument of the wrong type) report the user's call.
    body(method) <- if ("self" %in% all.names(call)) {
      bquote({
        .check_client(self)
        result <- .(call)
        .raise_if_error(result)
      })
    } else {
      bquote({
        result <- .(call)
        .raise_if_error(result)
      })
    }
    MyHospitalsClient[[name]] <- method
  }
})

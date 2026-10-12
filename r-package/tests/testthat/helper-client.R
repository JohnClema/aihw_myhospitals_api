# A client whose calls fail fast without touching the network: port 1 on
# localhost refuses connections. reqwest reads the proxy settings when the
# client is built, so keep localhost off any configured HTTP proxy then.
unreachable_client <- function(url = "http://127.0.0.1:1") {
  old <- Sys.getenv(c("NO_PROXY", "no_proxy"), unset = NA)
  Sys.setenv(NO_PROXY = "127.0.0.1", no_proxy = "127.0.0.1")
  on.exit({
    Sys.unsetenv(names(old)[is.na(old)])
    if (any(!is.na(old))) do.call(Sys.setenv, as.list(old[!is.na(old)]))
  })
  MyHospitalsClient$new(url)
}

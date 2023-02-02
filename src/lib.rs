extern crate libc;

use curl::Error;
use curl_sys::{CURLcode, CURL, CURLE_OK};
use libc::{c_char, c_int};
use std::ffi::CString;

extern "C" {
    pub fn curl_easy_impersonate(
        data: *mut CURL,
        target: *const c_char,
        default_headers: c_int,
    ) -> CURLcode;
}

/// Sets the appropriate options & headers to impersonate the `target`
/// The built-in list of HTTP headers will not be set if `default_headers` is `false`
/// `curl` is a raw curl handle, usually obtained via `easy.raw()`
pub fn impersonate(curl: *mut CURL, target: &str, default_headers: bool) -> Result<(), Error> {
    let target = CString::new(target)?;

    let rc = unsafe { curl_easy_impersonate(curl, target.as_ptr(), default_headers as c_int) };

    if rc == CURLE_OK {
        return Ok(());
    }

    Err(Error::new(rc))
}

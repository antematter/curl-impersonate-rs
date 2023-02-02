use curl::easy::Easy;
use curl_impersonate;
use std::io::{stdout, Write};

fn main() {
    let mut easy = Easy::new();

    curl_impersonate::impersonate(easy.raw(), "chrome99_android", true);

    easy.url("http://httpbin.org/get").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();

    easy.perform().unwrap();
    println!("Code {}", easy.response_code().unwrap());
}

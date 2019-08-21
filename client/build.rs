// extern crate bindgen;

fn main() {
    println!(r"cargo:rustc-link-lib=../altv-capi/altv-capi-client");
}

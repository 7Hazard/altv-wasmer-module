
fn main()
{
    // write build configs for rustflags
    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => {
            println!("cargo:rustc-flags=-l libucrtd -l libvcruntimed -l libcmtd -l libcpmtd");
        },
        "release" => {
            println!("cargo:rustc-flags=-l libucrt -l libvcruntime -l libcmt -l libcpmt");
        },
        _ => panic!("bad build profile"),
    };
}

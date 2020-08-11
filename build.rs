
fn main()
{
    println!("cargo:rustc-env=RUSTFLAGS=target-feature=+crt-static");
}

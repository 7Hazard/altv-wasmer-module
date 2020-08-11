extern crate reqwest;
extern crate bindgen;

use error_chain::error_chain;
use std::path::Path;
use std::fs;
use std::io::prelude::*;
use std::fs::File;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()>
{
    dl_capi("client", "win32").await?;
    bind("client", "win32");

    Ok(())
}

async fn dl_capi(kind: &str, platform: &str) -> Result<()>
{
    let profile = std::env::var("PROFILE").unwrap();
    let capiname = match profile.as_str() {
        "debug" => format!("altv-capi-{}-static-mtd-{}", kind, platform),
        "release" => format!("altv-capi-{}-static-{}", kind, platform),
        _ => panic!("bad build profile"),
    };
    let capipathstr = format!("./{}", capiname);
    let capipath = Path::new(&capipathstr[..]);
    if capipath.exists()
    {
        return Ok(())
    }

    let url = format!("http://github.com/7Hazard/altv-capi/releases/latest/download/{}.zip", capiname);
    let response = reqwest::get(&url[..]).await?;

    let zipstr = format!("./{}.zip", capiname);
    let zip_path = Path::new(&zipstr[..]);

    {
        let content =  response.bytes().await?;
        File::create(&zip_path).expect("couldn't create zip file").write_all(&content[..])?;
    }

    // unzip
    let unzipper = unzip::Unzipper::new(File::open(zip_path).expect("could not open zip"), Path::new("./"));
    unzipper.unzip().expect("could not unzip");

    fs::remove_file(zip_path).expect("could not remove file");

    Ok(())
}

fn bind(kind: &str, platform: &str)
{
    let profile = std::env::var("PROFILE").unwrap();
    let capiname = match profile.as_str() {
        "debug" => format!("altv-capi-{}-static-mtd", kind),
        "release" => format!("altv-capi-{}-static", kind),
        _ => panic!("bad build profile"),
    };
    let capinameplat = format!("{}-{}", capiname, platform);
    let relpath = format!("./{}/lib", capinameplat);
    let path = Path::new(&relpath[..]).canonicalize().unwrap();
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static={}", capiname);

    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .layout_tests(false)

        // The input header we would like to generate
        // bindings for.
        .header(format!("./{}/include/altv-capi-{}.h", capinameplat, kind))

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))

        // Finish the builder and generate the bindings.
        .generate()

        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(format!("src/altv_{}.rs", kind))
        .expect("Couldn't write bindings!");
}

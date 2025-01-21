mod auto_version;

use std::thread;
use std::time::Duration;
use crate::auto_version::{copy_with_version, get_version, update_and_get_version};
use interoptopus::util::NamespaceMappings;
use interoptopus::Interop;
use interoptopus_backend_csharp::overloads::DotNet;
use interoptopus_backend_csharp::{Config, Generator};
use rust_bindings::ffi_inventory;

// By adding the interop generation logic into a `build.rs` that depends on
// our `rust_bindings` we ensure that upon `cargo build` both the `.dll`
// gets built, and the `.cs` files.
//
// Instead, if you used to unit test trick in the other examples, you will have
// to run both `cargo build` to produce the `.dll` and `cargo test`
// to produce the bindings (since `cargo test` does not imply `cargo build`).
const DLL_FILE: &str = "rust_bindings";
const SO_FILE: &str = "librust_bindings";
// TODO: Release version
const DLL_SOURCE: &str = "../target/release/"; // Won't work when build from global workspace
const SO_SOURCE: &str = "../target/aarch64-linux-android/release/"; // Won't work when build from global workspace
const DLL_DEST: &str = "C:/Users/luish/Rust/Assets/Plugins/";
const SO_DEST: &str = "C:/Users/luish/Rust/Assets/Plugins/Android/arm64-v8a";
const OUT_DIR: &str = "C:/Users/luish/Rust/Assets/InteropScripts";

fn main() {
    println!("cargo:warning=PROFILE={}", std::env::var("PROFILE").unwrap());

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android_build();    // Assuming we previously build normally for windows
        return;
    }

    let build_script_active = std::env::var("CARGO_FEATURE_BUILD_SCRIPT").is_ok();

    if !build_script_active {
        // Print all arguments
        println!("Skipping build script tasks...");
        return;
    }

    println!("Performing build script tasks...");

    let lib_name = match update_and_get_version() {
        Ok(version) => {
            println!("Current version: {}", version);
            copy_with_version(DLL_SOURCE, DLL_DEST, DLL_FILE, &version, None).unwrap_or_else(|e| {
                eprintln!("Failed to copy files: {}", e);
                "Failed to copy files".to_string()
            })
        }
        Err(e) => {
            eprintln!("Failed to update version: {}", e);
            "Failed to update version".to_string()
        }
    };

    println!("Creating Interop.cs in {} for {}", OUT_DIR, &lib_name);

    let inventory = ffi_inventory();
    let overload = DotNet::new().build();
    // let config = ConfigBuilder::default().build().unwrap();
    let config = Config {
        class: "Interop".to_string(),
        dll_name: lib_name, // TODO: so file handling
        namespace_mappings: NamespaceMappings::new("InteropScripts"),
        ..Config::default()
    };

    Generator::new(config, inventory)
        .add_overload_writer(overload)
        // You might also want to consider writing to `OUT_DIR` instead, since
        // writing to any other place from a `build.rs` is discouraged (we do
        // it here to simplify our example).
        .write_file(OUT_DIR.to_owned() + "/Interop.cs")
        .unwrap();
}

fn android_build() {
    match get_version() {
        Ok(version) => {
            println!("Current version: {}", version);
            copy_with_version(SO_SOURCE, SO_DEST, DLL_FILE, &version, Option::from(SO_FILE)).unwrap_or_else(|e| {
                eprintln!("Failed to copy files: {}", e);
                "Failed to copy files".to_string()
            })
        }
        Err(e) => {
            eprintln!("Failed to update version: {}", e);
            "Failed to update version".to_string()
        }

    };
}

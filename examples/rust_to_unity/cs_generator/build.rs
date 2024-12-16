mod auto_version;
use crate::auto_version::{copy_with_version, update_and_get_version};
use interoptopus::Interop;
use interoptopus_backend_csharp::overloads::DotNet;
use interoptopus_backend_csharp::{ConfigBuilder, Generator};
use rust_bindings::ffi_inventory;

// By adding the interop generation logic into a `build.rs` that depends on
// our `rust_bindings` we ensure that upon `cargo build` both the `.dll`
// gets built, and the `.cs` files.
//
// Instead, if you used to unit test trick in the other examples, you will have
// to run both `cargo build` to produce the `.dll` and `cargo test`
// to produce the bindings (since `cargo test` does not imply `cargo build`).
const DLL_FILE: &str = "rust_bindings";
// TODO: Release version
const DLL_SOURCE: &str = "../target/debug/"; // Won't work when build from global workspace
const DLL_DEST: &str = "C:/Users/luish/Rust/Assets/Plugins/";
const OUT_DIR: &str = "C:/Users/luish/Rust/Assets/InteropScripts";

fn main() {
    match update_and_get_version() {
        Ok(version) => {
            println!("Current version: {}", version);
            if let Err(e) = copy_with_version(DLL_SOURCE, DLL_DEST, DLL_FILE, &version) {
                eprintln!("Failed to copy files: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to update version: {}", e),
    }

    println!("Creating Interop.cs in {}", OUT_DIR);

    let inventory = ffi_inventory();
    let overload = DotNet::new().build();
    let config = ConfigBuilder::default().build().unwrap();

    Generator::new(config, inventory)
        .add_overload_writer(overload)
        // You might also want to consider writing to `OUT_DIR` instead, since
        // writing to any other place from a `build.rs` is discouraged (we do
        // it here to simplify our example).
        .write_file(OUT_DIR.to_owned() + "/Interop.cs")
        .unwrap();
}

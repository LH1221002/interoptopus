use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_function, function, pattern, Inventory, InventoryBuilder};

pub mod engine;
pub mod error;
mod all_examples3;
mod basic3;
mod custom_forces3;
mod elasticity3;
mod faucet3;
mod harness_basic3;
mod heightfield3;
mod helper;
mod surface_tension3;

// As in `engine`, we create matching functions that are better suited for an FFI boundary.
#[ffi_function]
pub fn start_server(server_name: CStrPointer) {
    let Ok(name) = server_name.as_str() else {
        return;
    };

    rust_library::start_server(name.to_string());
}

pub fn ffi_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(start_server))
        .register(pattern!(engine::GameEngine))
        .validate()
        .inventory()
}

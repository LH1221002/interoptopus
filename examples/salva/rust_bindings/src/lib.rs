use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_function, function, pattern, Inventory, InventoryBuilder};

pub mod engine;
pub mod error;
pub mod math;
pub mod fluid;
pub mod fluids_pipeline;

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
        .register(pattern!(fluid::Fluid))
        .register(pattern!(fluids_pipeline::FluidsPipeline))
        .validate()
        .inventory()
}

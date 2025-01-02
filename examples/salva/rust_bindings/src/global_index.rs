use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::fluid::Fluid;

// Assuming `Fluid` is already defined in your codebase
pub static GLOBAL_FLUIDS: Lazy<Mutex<Vec<Fluid>>> = Lazy::new(|| Mutex::new(Vec::new()));

// Modularized FFI interface for the browser rendering engine
// This file now only declares and re-exports the logical FFI modules

pub mod html_parsing;
pub use html_parsing::*;
pub mod layout_api;
pub use layout_api::*;
pub mod draw_commands;
pub use draw_commands::*;
pub mod dom_api;
pub use dom_api::*;
pub mod memory_management;
pub use memory_management::*;
pub mod js_api;
pub use js_api::*; 
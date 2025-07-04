// DOM node manipulation FFI functions for the browser engine
// Extracted from functions.rs for modularization

use crate::dom::node::{DOMNode, DOMArena, NodeType, FFILayoutBox};
use std::ffi::{c_char, CString};
use std::ptr;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::dom::node::NODE_ID_COUNTER;
use super::{safe_c_string_to_rust, safe_rust_string_to_c};

static ARENA: Lazy<Mutex<DOMArena>> = Lazy::new(|| Mutex::new(DOMArena::new()));

// --- All dom_get_*, dom_set_*, dom_insert_*, dom_remove_*, dom_class_list_*, dom_add_event_listener, dom_remove_event_listener, dom_dispatch_event, and helpers go here ---
// (Move the full function bodies from functions.rs)

// ... (functions will be moved here in the next step) ... 
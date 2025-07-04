// Memory management FFI functions for the browser engine
// Extracted from functions.rs for modularization

use crate::dom::node::FFILayoutBox;
use super::{LayoutBoxArray, DrawCommandArray};
use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn free_layout_box_array(box_array_ptr: *mut LayoutBoxArray) {
    if !box_array_ptr.is_null() {
        unsafe {
            let box_array = Box::from_raw(box_array_ptr);
            for box_ptr in box_array.boxes {
                if !box_ptr.is_null() {
                    let _ = Box::from_raw(box_ptr);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn free_ffi_layout_box(box_ptr: *mut FFILayoutBox) {
    if !box_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(box_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

// ... (functions will be moved here in the next step) ... 
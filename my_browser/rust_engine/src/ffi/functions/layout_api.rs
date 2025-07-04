// Layout box FFI functions for the browser engine
// Extracted from functions.rs for modularization

use crate::dom::node::{FFILayoutBox, LayoutBox};
use crate::ffi::LayoutBoxArray;
use std::ffi::c_char;
use std::ptr;

#[no_mangle]
pub extern "C" fn get_layout_box_batch(
    box_array_ptr: *mut LayoutBoxArray,
    start: i32,
    count: i32,
    out_ptr: *mut *mut FFILayoutBox,
) -> i32 {
    get_layout_box_batch_enhanced(box_array_ptr, start, count, out_ptr)
}

#[no_mangle]
pub extern "C" fn get_layout_box_batch_enhanced(
    box_array_ptr: *mut LayoutBoxArray,
    start: i32,
    count: i32,
    out_ptr: *mut *mut FFILayoutBox,
) -> i32 {
    println!("[FFI] get_layout_box_batch_enhanced: start={}, count={}", start, count);
    let result = std::panic::catch_unwind(|| {
        if box_array_ptr.is_null() || out_ptr.is_null() || start < 0 || count <= 0 {
            println!("[FFI] Invalid arguments");
            return 0;
        }
        let box_array = unsafe { &*box_array_ptr };
        let len = box_array.boxes.len() as i32;
        let end = (start + count).min(len);
        let actual_count = end - start;
        for i in 0..actual_count {
            unsafe {
                *out_ptr.offset(i as isize) = box_array.boxes[(start + i) as usize];
            }
        }
        println!("[FFI] Returning {} boxes", actual_count);
        actual_count
    });
    match result {
        Ok(n) => n,
        Err(_) => {
            eprintln!("[FFI] get_layout_box_batch_enhanced: panic caught!");
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn get_layout_box_count(box_array_ptr: *mut LayoutBoxArray) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if box_array_ptr.is_null() {
            return 0;
        }
        let box_array = unsafe { &*box_array_ptr };
        box_array.total_count
    });
    match result {
        Ok(count) => count,
        Err(_) => 0
    }
}

#[no_mangle]
pub extern "C" fn get_layout_box(box_array_ptr: *mut LayoutBoxArray, index: i32) -> *mut FFILayoutBox {
    let result = std::panic::catch_unwind(|| {
        if box_array_ptr.is_null() || index < 0 {
            return ptr::null_mut();
        }
        let box_array = unsafe { &*box_array_ptr };
        if index as usize >= box_array.boxes.len() {
            return ptr::null_mut();
        }
        box_array.boxes[index as usize]
    });
    match result {
        Ok(ptr) => ptr,
        Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn get_layout_box_x(box_ptr: *mut FFILayoutBox) -> f32 {
    let result = std::panic::catch_unwind(|| {
        if box_ptr.is_null() { return 0.0; }
        let layout_box = unsafe { &*box_ptr };
        layout_box.x
    });
    match result {
        Ok(val) => val,
        Err(_) => 0.0
    }
}

#[no_mangle]
pub extern "C" fn get_layout_box_y(box_ptr: *mut FFILayoutBox) -> f32 {
    let result = std::panic::catch_unwind(|| {
        if box_ptr.is_null() { return 0.0; }
        let layout_box = unsafe { &*box_ptr };
        layout_box.y
    });
    match result {
        Ok(val) => val,
        Err(_) => 0.0
    }
}

#[no_mangle]
pub extern "C" fn get_layout_box_width(box_ptr: *mut FFILayoutBox) -> f32 {
    let result = std::panic::catch_unwind(|| {
        if box_ptr.is_null() { return 0.0; }
        let layout_box = unsafe { &*box_ptr };
        layout_box.width
    });
    match result {
        Ok(val) => val,
        Err(_) => 0.0
    }
}

#[no_mangle]
pub extern "C" fn get_layout_box_height(box_ptr: *mut FFILayoutBox) -> f32 {
    let result = std::panic::catch_unwind(|| {
        if box_ptr.is_null() { return 0.0; }
        let layout_box = unsafe { &*box_ptr };
        layout_box.height
    });
    match result {
        Ok(val) => val,
        Err(_) => 0.0
    }
} 
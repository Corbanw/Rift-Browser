// JavaScript execution FFI functions for the browser engine
// Extracted from functions.rs for modularization

use std::ffi::c_char;
use crate::VeloxEngine;
use crate::ffi::{FFIPerformanceTracker, safe_c_string_to_rust};

#[no_mangle]
pub extern "C" fn execute_javascript(script_ptr: *const c_char, script_name_ptr: *const c_char) -> i32 {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] execute_javascript called");
    let input_start = std::time::Instant::now();
    let script_content = match safe_c_string_to_rust(script_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] Script content conversion failed: {}", e);
            return -1;
        }
    };
    let script_name = match safe_c_string_to_rust(script_name_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] Script name conversion failed: {}", e);
            return -1;
        }
    };
    tracker.record_stage("input_conversion", input_start.elapsed());
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut engine = VeloxEngine::new(800.0, 600.0);
        let js_start = std::time::Instant::now();
        let execution_result = engine.execute_script(&script_content, &script_name);
        let _js_duration = js_start.elapsed();
        match execution_result {
            Ok(_) => {
                println!("[FFI] JavaScript executed successfully: {}", script_name);
                0
            }
            Err(e) => {
                eprintln!("[FFI] JavaScript execution failed: {}", e);
                -1
            }
        }
    }));
    match result {
        Ok(result_code) => result_code,
        Err(_) => {
            eprintln!("[FFI] execute_javascript: panic caught!");
            -1
        }
    }
} 
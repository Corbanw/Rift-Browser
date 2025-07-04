// Draw command FFI functions for the browser engine
// Extracted from functions.rs for modularization

use crate::paint::painter::Painter;
use crate::paint::display_list::DrawCommand;
use super::{DrawCommandArray, LayoutBoxArray, FFIPerformanceTracker, safe_rust_string_to_c};
use crate::dom::node::LayoutBox;
use crate::parser::html::HTMLParser;
use crate::parser::css::parse_css;
use crate::layout::layout::LayoutEngine;
use crate::compositor::compositor::Compositor;
use std::ffi::{c_char, CString};
use std::ptr;

#[no_mangle]
pub extern "C" fn parse_html_to_draw_commands(input_ptr: *const c_char) -> *mut DrawCommandArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html_to_draw_commands called");
    let input_start = std::time::Instant::now();
    let input_string = match super::safe_c_string_to_rust(input_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] Input conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    tracker.record_stage("input_conversion", input_start.elapsed());
    let result = std::panic::catch_unwind(|| {
        let parse_start = std::time::Instant::now();
        let mut parser = HTMLParser::new(input_string);
        let dom = parser.parse();
        let parse_duration = parse_start.elapsed();
        let css_start = std::time::Instant::now();
        let stylesheet = parser.get_stylesheet();
        let css_duration = css_start.elapsed();
        let layout_start = std::time::Instant::now();
        let mut layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
        let arena = super::ARENA.lock().unwrap();
        let layout_boxes = layout_engine.layout(&dom, &*arena);
        let layout_duration = layout_start.elapsed();
        let draw_start = std::time::Instant::now();
        let draw_commands = layout_boxes_to_draw_commands(&layout_boxes);
        let draw_duration = draw_start.elapsed();
        println!("[FFI] Generated {} draw commands", draw_commands.len());
        let conversion_start = std::time::Instant::now();
        let draw_array = DrawCommandArray::new(draw_commands);
        let conversion_duration = conversion_start.elapsed();
        (draw_array, parse_duration, css_duration, layout_duration, draw_duration, conversion_duration)
    });
    match result {
        Ok((draw_array, parse_duration, css_duration, layout_duration, draw_duration, conversion_duration)) => {
            tracker.record_stage("html_parsing", parse_duration);
            tracker.record_stage("css_parsing", css_duration);
            tracker.record_stage("layout", layout_duration);
            tracker.record_stage("draw_conversion", draw_duration);
            tracker.record_stage("ffi_conversion", conversion_duration);
            tracker.log_performance();
            Box::into_raw(Box::new(draw_array))
        }
        Err(_) => {
            eprintln!("[FFI] parse_html_to_draw_commands: panic caught!");
            ptr::null_mut()
        }
    }
}

fn layout_boxes_to_draw_commands(layout_boxes: &[LayoutBox]) -> Vec<DrawCommand> {
    let mut commands = Vec::new();
    for layout_box in layout_boxes {
        let rect_command = DrawCommand {
            command_type: 0,
            x: layout_box.x,
            y: layout_box.y,
            width: layout_box.width,
            height: layout_box.height,
            color: safe_rust_string_to_c(""),
            text: ptr::null_mut(),
            font_size: 0.0,
            font_weight: 0.0,
        };
        commands.push(rect_command);
        if !layout_box.text_content.is_empty() {
            let text_command = DrawCommand {
                command_type: 1,
                x: layout_box.x + 2.0,
                y: layout_box.y + layout_box.font_size + 2.0,
                width: layout_box.width - 4.0,
                height: layout_box.font_size,
                color: safe_rust_string_to_c(""),
                text: safe_rust_string_to_c(&layout_box.text_content),
                font_size: layout_box.font_size,
                font_weight: layout_box.font_weight,
            };
            commands.push(text_command);
        }
    }
    commands
}

#[no_mangle]
pub extern "C" fn get_draw_command_count(cmd_array_ptr: *mut DrawCommandArray) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if cmd_array_ptr.is_null() {
            return 0;
        }
        let cmd_array = unsafe { &*cmd_array_ptr };
        cmd_array.total_count
    });
    match result {
        Ok(count) => count,
        Err(_) => 0
    }
}

#[no_mangle]
pub extern "C" fn get_draw_command(cmd_array_ptr: *mut DrawCommandArray, index: i32) -> *mut DrawCommand {
    let result = std::panic::catch_unwind(|| {
        if cmd_array_ptr.is_null() || index < 0 {
            return ptr::null_mut();
        }
        let cmd_array = unsafe { &*cmd_array_ptr };
        if index >= cmd_array.total_count {
            return ptr::null_mut();
        }
        cmd_array.commands[index as usize]
    });
    match result {
        Ok(ptr) => ptr,
        Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn free_draw_command_array(cmd_array_ptr: *mut DrawCommandArray) {
    if !cmd_array_ptr.is_null() {
        unsafe {
            let cmd_array = Box::from_raw(cmd_array_ptr);
            for cmd_ptr in cmd_array.commands {
                if !cmd_ptr.is_null() {
                    let cmd = Box::from_raw(cmd_ptr);
                    if !cmd.color.is_null() {
                        let _ = CString::from_raw(cmd.color);
                    }
                    if !cmd.text.is_null() {
                        let _ = CString::from_raw(cmd.text);
                    }
                }
            }
        }
    }
} 
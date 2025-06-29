// Main FFI functions for browser rendering engine
// Provides C-compatible interface with enhanced batching and draw commands

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use tokio::runtime::Runtime;
use reqwest::Client as AsyncClient;
use futures::StreamExt;

use crate::dom::node::{DOMNode, LayoutBox, FFILayoutBox, NodeType, StyleMap, BoxValues};
use crate::parser::html::{HTMLParser, StreamingHTMLParser};
use crate::parser::css::{parse_css, Stylesheet};
use crate::layout::layout::LayoutEngine;
use crate::paint::painter::Painter;
use crate::compositor::compositor::Compositor;

use super::{LayoutBoxArray, DrawCommand, DrawCommandArray, FFIPerformanceTracker, 
            safe_c_string_to_rust, safe_rust_string_to_c, apply_stylesheet_to_dom, 
            process_html_streaming};

// Main HTML parsing function with performance tracking
#[no_mangle]
pub extern "C" fn parse_html(input_ptr: *const c_char) -> *mut LayoutBoxArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html called");
    
    // Input conversion
    let input_start = std::time::Instant::now();
    let input_string = match safe_c_string_to_rust(input_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] Input conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    tracker.record_stage("input_conversion", input_start.elapsed());
    
    let result = std::panic::catch_unwind(|| {
        if input_string.len() > 500_000 {
            println!("[PERF] WARNING: Large input detected ({}bytes)", input_string.len());
        }
        
        // HTML parsing
        let parse_start = std::time::Instant::now();
        let mut parser = HTMLParser::new(input_string);
        let dom = parser.parse();
        let parse_duration = parse_start.elapsed();
        
        println!("[FFI] DOM parsed with {} nodes", dom.children.len());
        
        // CSS extraction and parsing
        let css_start = std::time::Instant::now();
        let stylesheet = parser.get_stylesheet();
        let css_duration = css_start.elapsed();
        
        // Layout generation
        let layout_start = std::time::Instant::now();
        let mut layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
        let layout_boxes = layout_engine.layout(&dom);
        let layout_duration = layout_start.elapsed();
        
        // Paint and compositor pipeline
        let paint_start = std::time::Instant::now();
        let display_list = Painter::from_layout_boxes(&layout_boxes);
        let compositor = Compositor::new();
        let _composited_list = compositor.composite(display_list);
        let paint_duration = paint_start.elapsed();
        
        println!("[FFI] Generated {} layout boxes", layout_boxes.len());
        
        // FFI conversion
        let conversion_start = std::time::Instant::now();
        let layout_array = LayoutBoxArray::new(layout_boxes);
        let conversion_duration = conversion_start.elapsed();
        
        // Return timing data along with the result
        (layout_array, parse_duration, css_duration, layout_duration, paint_duration, conversion_duration)
    });
    
    match result {
        Ok((layout_array, parse_duration, css_duration, layout_duration, paint_duration, conversion_duration)) => {
            tracker.record_stage("html_parsing", parse_duration);
            tracker.record_stage("css_parsing", css_duration);
            tracker.record_stage("layout", layout_duration);
            tracker.record_stage("paint_compositor", paint_duration);
            tracker.record_stage("ffi_conversion", conversion_duration);
            tracker.log_performance();
            Box::into_raw(Box::new(layout_array))
        }
        Err(_) => {
            eprintln!("[FFI] parse_html: panic caught!");
            ptr::null_mut()
        }
    }
}

// HTML parsing function that accepts both HTML and CSS as separate parameters
#[no_mangle]
pub extern "C" fn parse_html_with_css(html_ptr: *const c_char, css_ptr: *const c_char) -> *mut LayoutBoxArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html_with_css called");
    
    // Input conversion
    let input_start = std::time::Instant::now();
    let html_string = match safe_c_string_to_rust(html_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] HTML input conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    
    let css_string = match safe_c_string_to_rust(css_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] CSS input conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    tracker.record_stage("input_conversion", input_start.elapsed());
    
    let result = std::panic::catch_unwind(|| {
        if html_string.len() > 500_000 {
            println!("[PERF] WARNING: Large HTML input detected ({}bytes)", html_string.len());
        }
        
        // HTML parsing
        let parse_start = std::time::Instant::now();
        let mut parser = HTMLParser::new(html_string);
        let dom = parser.parse();
        let parse_duration = parse_start.elapsed();
        
        println!("[FFI] DOM parsed with {} nodes", dom.children.len());
        
        // CSS parsing
        let css_start = std::time::Instant::now();
        let mut stylesheet = parser.get_stylesheet(); // Get inline styles from HTML
        
        // Parse additional CSS if provided
        if !css_string.is_empty() {
            let additional_css = parse_css(&css_string);
            // Merge the additional CSS rules with the inline styles
            stylesheet.rules.extend(additional_css.rules);
        }
        let css_duration = css_start.elapsed();
        
        // Layout generation
        let layout_start = std::time::Instant::now();
        let mut layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
        let layout_boxes = layout_engine.layout(&dom);
        let layout_duration = layout_start.elapsed();
        
        // Paint and compositor pipeline
        let paint_start = std::time::Instant::now();
        let display_list = Painter::from_layout_boxes(&layout_boxes);
        let compositor = Compositor::new();
        let _composited_list = compositor.composite(display_list);
        let paint_duration = paint_start.elapsed();
        
        println!("[FFI] Generated {} layout boxes", layout_boxes.len());
        
        // FFI conversion
        let conversion_start = std::time::Instant::now();
        let layout_array = LayoutBoxArray::new(layout_boxes);
        let conversion_duration = conversion_start.elapsed();
        
        // Return timing data along with the result
        (layout_array, parse_duration, css_duration, layout_duration, paint_duration, conversion_duration)
    });
    
    match result {
        Ok((layout_array, parse_duration, css_duration, layout_duration, paint_duration, conversion_duration)) => {
            tracker.record_stage("html_parsing", parse_duration);
            tracker.record_stage("css_parsing", css_duration);
            tracker.record_stage("layout", layout_duration);
            tracker.record_stage("paint_compositor", paint_duration);
            tracker.record_stage("ffi_conversion", conversion_duration);
            tracker.log_performance();
            Box::into_raw(Box::new(layout_array))
        }
        Err(_) => {
            eprintln!("[FFI] parse_html_with_css: panic caught!");
            ptr::null_mut()
        }
    }
}

// Minimal legacy FFI function for get_layout_box_batch, forwards to enhanced version
#[no_mangle]
pub extern "C" fn get_layout_box_batch(
    box_array_ptr: *mut LayoutBoxArray,
    start: i32,
    count: i32,
    out_ptr: *mut *mut FFILayoutBox,
) -> i32 {
    get_layout_box_batch_enhanced(box_array_ptr, start, count, out_ptr)
}

// Enhanced batch extraction
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

// New function to get draw commands instead of layout boxes
#[no_mangle]
pub extern "C" fn parse_html_to_draw_commands(input_ptr: *const c_char) -> *mut DrawCommandArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html_to_draw_commands called");
    
    // Input conversion
    let input_start = std::time::Instant::now();
    let input_string = match safe_c_string_to_rust(input_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] Input conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    tracker.record_stage("input_conversion", input_start.elapsed());
    
    let result = std::panic::catch_unwind(|| {
        // HTML parsing
        let parse_start = std::time::Instant::now();
        let mut parser = HTMLParser::new(input_string);
        let dom = parser.parse();
        let parse_duration = parse_start.elapsed();
        
        // CSS extraction and parsing
        let css_start = std::time::Instant::now();
        let stylesheet = parser.get_stylesheet();
        let css_duration = css_start.elapsed();
        
        // Layout generation
        let layout_start = std::time::Instant::now();
        let mut layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
        let layout_boxes = layout_engine.layout(&dom);
        let layout_duration = layout_start.elapsed();
        
        // Convert layout boxes to draw commands
        let draw_start = std::time::Instant::now();
        let draw_commands = layout_boxes_to_draw_commands(&layout_boxes);
        let draw_duration = draw_start.elapsed();
        
        println!("[FFI] Generated {} draw commands", draw_commands.len());
        
        // FFI conversion
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

// Convert layout boxes to draw commands
fn layout_boxes_to_draw_commands(layout_boxes: &[LayoutBox]) -> Vec<DrawCommand> {
    let mut commands = Vec::new();
    
    for layout_box in layout_boxes {
        // Create rectangle command for the box
        let rect_command = DrawCommand {
            command_type: 0, // Rectangle
            x: layout_box.x,
            y: layout_box.y,
            width: layout_box.width,
            height: layout_box.height,
            color: safe_rust_string_to_c(&layout_box.background_color),
            text: ptr::null_mut(),
            font_size: 0.0,
            font_weight: 0.0,
        };
        commands.push(rect_command);
        
        // Create text command if there's text content
        if !layout_box.text_content.is_empty() {
            let text_command = DrawCommand {
                command_type: 1, // Text
                x: layout_box.x + 2.0, // Small padding
                y: layout_box.y + layout_box.font_size + 2.0,
                width: layout_box.width - 4.0,
                height: layout_box.font_size,
                color: safe_rust_string_to_c(&layout_box.color),
                text: safe_rust_string_to_c(&layout_box.text_content),
                font_size: layout_box.font_size,
                font_weight: layout_box.font_weight,
            };
            commands.push(text_command);
        }
    }
    
    commands
}

// Minimal legacy FFI function for parse_url_via_rust, forwards to enhanced version
#[no_mangle]
pub extern "C" fn parse_url_via_rust(url_ptr: *const std::os::raw::c_char) -> *mut LayoutBoxArray {
    parse_url_via_rust_enhanced(url_ptr)
}

// Enhanced URL parsing with streaming
#[no_mangle]
pub extern "C" fn parse_url_via_rust_enhanced(url_ptr: *const c_char) -> *mut LayoutBoxArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_url_via_rust_enhanced called");
    
    // URL conversion
    let url_start = std::time::Instant::now();
    let url = match safe_c_string_to_rust(url_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] URL conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    tracker.record_stage("url_conversion", url_start.elapsed());
    
    println!("[FFI] Processing URL: {}", url);
    
    // Create runtime for async operations
    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("[FFI] Failed to create tokio runtime: {}", e);
            return ptr::null_mut();
        }
    };
    
    let result = std::panic::catch_unwind(|| {
        // Run async processing
        let async_result = rt.block_on(async {
            let stream_start = std::time::Instant::now();
            match process_html_streaming(&url).await {
                Ok((tokens, external_stylesheets)) => {
                    let stream_duration = stream_start.elapsed();
                    println!("[FFI] Streaming HTML processing successful: {} tokens", tokens.len());
                    
                    // Build DOM from tokens
                    let dom_start = std::time::Instant::now();
                    let mut dom_builder = HTMLParser::new(String::new());
                    let mut root = DOMNode::new(NodeType::Document);
                    dom_builder.build_dom_from_tokens(&tokens, &mut root);
                    let dom_duration = dom_start.elapsed();
                    
                    // Fetch external CSS concurrently
                    let css_start = std::time::Instant::now();
                    let mut all_css = String::new();
                    if !external_stylesheets.is_empty() {
                        println!("[FFI] Fetching {} external stylesheets", external_stylesheets.len());
                        
                        let mut css_futures = Vec::new();
                        for stylesheet_url in external_stylesheets {
                            let client = AsyncClient::new();
                            let future = async move {
                                match client.get(&stylesheet_url).send().await {
                                    Ok(resp) => match resp.text().await {
                                        Ok(css) => Some(css),
                                        Err(e) => {
                                            eprintln!("[FFI] Failed to read CSS from {}: {}", stylesheet_url, e);
                                            None
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("[FFI] Failed to fetch CSS from {}: {}", stylesheet_url, e);
                                        None
                                    }
                                }
                            };
                            css_futures.push(future);
                        }
                        
                        // Wait for all CSS fetches to complete
                        let css_results = futures::future::join_all(css_futures).await;
                        for css in css_results {
                            if let Some(css_content) = css {
                                all_css.push_str(&css_content);
                                all_css.push('\n');
                            }
                        }
                    }
                    let css_duration = css_start.elapsed();
                    
                    // Parse CSS and apply to DOM
                    let style_start = std::time::Instant::now();
                    let stylesheet = parse_css(&all_css);
                    apply_stylesheet_to_dom(&mut root, &stylesheet);
                    let style_duration = style_start.elapsed();
                    
                    println!("[FFI] Parsed CSS with {} rules", stylesheet.rules.len());
                    
                    // Generate layout boxes
                    let layout_start = std::time::Instant::now();
                    let layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
                    let layout_boxes = layout_engine.layout(&root);
                    let layout_duration = layout_start.elapsed();
                    
                    println!("[FFI] Generated {} layout boxes", layout_boxes.len());
                    
                    // Convert to FFI format
                    let conversion_start = std::time::Instant::now();
                    let layout_array = LayoutBoxArray::new(layout_boxes);
                    let conversion_duration = conversion_start.elapsed();
                    
                    Ok((layout_array, stream_duration, dom_duration, css_duration, style_duration, layout_duration, conversion_duration))
                }
                Err(e) => {
                    eprintln!("[FFI] Streaming HTML processing failed: {}", e);
                    Err(e)
                }
            }
        });
        
        async_result
    });
    
    match result {
        Ok(Ok((layout_array, stream_duration, dom_duration, css_duration, style_duration, layout_duration, conversion_duration))) => {
            tracker.record_stage("streaming_parse", stream_duration);
            tracker.record_stage("dom_build", dom_duration);
            tracker.record_stage("css_fetch", css_duration);
            tracker.record_stage("style_apply", style_duration);
            tracker.record_stage("layout", layout_duration);
            tracker.record_stage("ffi_conversion", conversion_duration);
            tracker.log_performance();
            Box::into_raw(Box::new(layout_array))
        }
        Ok(Err(_)) => {
            eprintln!("[FFI] parse_url_via_rust_enhanced: processing failed");
            ptr::null_mut()
        }
        Err(_) => {
            eprintln!("[FFI] parse_url_via_rust_enhanced: panic caught!");
            ptr::null_mut()
        }
    }
}

// Legacy functions for backward compatibility
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

// Individual property getters for layout boxes
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

// Memory management functions
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
pub extern "C" fn free_draw_command_array(cmd_array_ptr: *mut DrawCommandArray) {
    if !cmd_array_ptr.is_null() {
        unsafe {
            let cmd_array = Box::from_raw(cmd_array_ptr);
            for cmd_ptr in cmd_array.commands {
                if !cmd_ptr.is_null() {
                    let _ = Box::from_raw(cmd_ptr);
                }
            }
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

// Test function
#[no_mangle]
pub extern "C" fn test_export() -> f32 {
    println!("[FFI] test_export called");
    42.0
} 
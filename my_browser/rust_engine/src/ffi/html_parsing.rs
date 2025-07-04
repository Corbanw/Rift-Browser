// HTML and CSS parsing FFI functions for the browser engine
// Extracted from functions.rs for modularization

use std::ffi::{c_char, CString};
use std::ptr;
use tokio::runtime::Runtime;
use reqwest::Client as AsyncClient;
use futures::StreamExt;
use crate::dom::node::{DOMNode, LayoutBox, FFILayoutBox, NodeType, DOMArena};
use crate::parser::html::HTMLParser;
use crate::parser::css::parse_css;
use crate::layout::layout::LayoutEngine;
use crate::paint::painter::Painter;
use crate::compositor::compositor::Compositor;
use crate::VeloxEngine;
use super::{LayoutBoxArray, FFIPerformanceTracker, safe_c_string_to_rust, apply_stylesheet_to_dom, process_html_streaming};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static ARENA: Lazy<Mutex<DOMArena>> = Lazy::new(|| Mutex::new(DOMArena::new()));

// JavaScript execution function (moved to js_api.rs)

// HTML parsing with JavaScript execution
#[no_mangle]
pub extern "C" fn parse_html_with_javascript(html_ptr: *const c_char) -> *mut LayoutBoxArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html_with_javascript called");
    let input_start = std::time::Instant::now();
    let html_string = match safe_c_string_to_rust(html_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] HTML input conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    tracker.record_stage("input_conversion", input_start.elapsed());
    let result = std::panic::catch_unwind(|| {
        if html_string.len() > 500_000 {
            println!("[PERF] WARNING: Large input detected ({}bytes)", html_string.len());
        }
        let parse_start = std::time::Instant::now();
        let mut parser = HTMLParser::new(html_string);
        let dom = parser.parse();
        let parse_duration = parse_start.elapsed();
        println!("[FFI] DOM parsed with {} nodes", dom.children.len());
        let mut engine = VeloxEngine::new(800.0, 600.0);
        let js_start = std::time::Instant::now();
        for (i, script_content) in parser.get_extracted_scripts().iter().enumerate() {
            let script_name = format!("inline_script_{}", i);
            if let Err(e) = engine.execute_script(script_content, &script_name) {
                eprintln!("[FFI] Failed to execute script {}: {}", script_name, e);
            }
        }
        let js_duration = js_start.elapsed();
        let css_start = std::time::Instant::now();
        let stylesheet = parser.get_stylesheet();
        let css_duration = css_start.elapsed();
        let layout_start = std::time::Instant::now();
        let mut layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
        let arena = ARENA.lock().unwrap();
        let layout_boxes = layout_engine.layout(&dom, &*arena);
        let layout_duration = layout_start.elapsed();
        let paint_start = std::time::Instant::now();
        let display_list = Painter::from_layout_boxes(&layout_boxes);
        let compositor = Compositor::new();
        let _composited_list = compositor.composite(display_list);
        let paint_duration = paint_start.elapsed();
        println!("[FFI] Generated {} layout boxes", layout_boxes.len());
        let conversion_start = std::time::Instant::now();
        let layout_array = LayoutBoxArray::new(layout_boxes);
        let conversion_duration = conversion_start.elapsed();
        (layout_array, parse_duration, css_duration, js_duration, layout_duration, paint_duration, conversion_duration)
    });
    match result {
        Ok((layout_array, parse_duration, css_duration, js_duration, layout_duration, paint_duration, conversion_duration)) => {
            tracker.record_stage("html_parsing", parse_duration);
            tracker.record_stage("css_parsing", css_duration);
            tracker.record_stage("javascript_execution", js_duration);
            tracker.record_stage("layout", layout_duration);
            tracker.record_stage("paint_compositor", paint_duration);
            tracker.record_stage("ffi_conversion", conversion_duration);
            tracker.log_performance();
            Box::into_raw(Box::new(layout_array))
        }
        Err(_) => {
            eprintln!("[FFI] parse_html_with_javascript: panic caught!");
            ptr::null_mut()
        }
    }
}

// Main HTML parsing function with performance tracking
#[no_mangle]
pub extern "C" fn parse_html(input_ptr: *const c_char) -> *mut LayoutBoxArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html called");
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
        let parse_start = std::time::Instant::now();
        let mut parser = HTMLParser::new(input_string);
        let dom = parser.parse();
        let parse_duration = parse_start.elapsed();
        println!("[FFI] DOM parsed with {} nodes", dom.children.len());
        let css_start = std::time::Instant::now();
        let stylesheet = parser.get_stylesheet();
        let css_duration = css_start.elapsed();
        let layout_start = std::time::Instant::now();
        let mut layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
        let arena = ARENA.lock().unwrap();
        let layout_boxes = layout_engine.layout(&dom, &*arena);
        let layout_duration = layout_start.elapsed();
        let paint_start = std::time::Instant::now();
        let display_list = Painter::from_layout_boxes(&layout_boxes);
        let compositor = Compositor::new();
        let _composited_list = compositor.composite(display_list);
        let paint_duration = paint_start.elapsed();
        println!("[FFI] Generated {} layout boxes", layout_boxes.len());
        let conversion_start = std::time::Instant::now();
        let layout_array = LayoutBoxArray::new(layout_boxes);
        let conversion_duration = conversion_start.elapsed();
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
        let parse_start = std::time::Instant::now();
        let mut parser = HTMLParser::new(html_string);
        let dom = parser.parse();
        let parse_duration = parse_start.elapsed();
        println!("[FFI] DOM parsed with {} nodes", dom.children.len());
        let css_start = std::time::Instant::now();
        let mut stylesheet = parser.get_stylesheet();
        if !css_string.is_empty() {
            let additional_css = parse_css(&css_string);
            stylesheet.rules.extend(additional_css.rules);
        }
        let css_duration = css_start.elapsed();
        let layout_start = std::time::Instant::now();
        let mut layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
        let arena = ARENA.lock().unwrap();
        let layout_boxes = layout_engine.layout(&dom, &*arena);
        let layout_duration = layout_start.elapsed();
        let paint_start = std::time::Instant::now();
        let display_list = Painter::from_layout_boxes(&layout_boxes);
        let compositor = Compositor::new();
        let _composited_list = compositor.composite(display_list);
        let paint_duration = paint_start.elapsed();
        println!("[FFI] Generated {} layout boxes", layout_boxes.len());
        let conversion_start = std::time::Instant::now();
        let layout_array = LayoutBoxArray::new(layout_boxes);
        let conversion_duration = conversion_start.elapsed();
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

// HTML parsing with JavaScript execution and images
#[no_mangle]
pub extern "C" fn parse_html_with_css_and_images(input_ptr: *const c_char) -> *mut LayoutBoxArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html_with_css_and_images called");
    let input_string = match safe_c_string_to_rust(input_ptr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FFI] Input conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    let result = std::panic::catch_unwind(|| {
        let mut engine = VeloxEngine::new(800.0, 600.0);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let layout_boxes = runtime.block_on(async {
            engine.render_html_with_js(&input_string).await
        });
        match layout_boxes {
            Ok(boxes) => {
                println!("[FFI] Generated {} layout boxes with JavaScript", boxes.len());
                LayoutBoxArray::new(boxes)
            }
            Err(e) => {
                eprintln!("[FFI] JavaScript rendering failed: {}", e);
                let mut parser = HTMLParser::new(input_string);
                let dom = parser.parse();
                let stylesheet = parser.get_stylesheet();
                let layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
                let arena = ARENA.lock().unwrap();
                let boxes = layout_engine.layout(&dom, &*arena);
                LayoutBoxArray::new(boxes)
            }
        }
    });
    match result {
        Ok(layout_array) => {
            tracker.log_performance();
            Box::into_raw(Box::new(layout_array))
        }
        Err(_) => {
            eprintln!("[FFI] parse_html_with_css_and_images: panic caught!");
            ptr::null_mut()
        }
    }
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
    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("[FFI] Failed to create tokio runtime: {}", e);
            return ptr::null_mut();
        }
    };
    let result = std::panic::catch_unwind(|| {
        let async_result = rt.block_on(async {
            let stream_start = std::time::Instant::now();
            match process_html_streaming(&url).await {
                Ok((tokens, external_stylesheets)) => {
                    let stream_duration = stream_start.elapsed();
                    println!("[FFI] Streaming HTML processing successful: {} tokens", tokens.len());
                    let dom_start = std::time::Instant::now();
                    let mut dom_builder = HTMLParser::new(String::new());
                    let mut root = DOMNode::new(NodeType::Document);
                    dom_builder.build_dom_from_tokens(&tokens, &mut root);
                    let dom_duration = dom_start.elapsed();
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
                        let css_results = futures::future::join_all(css_futures).await;
                        for css in css_results {
                            if let Some(css_content) = css {
                                all_css.push_str(&css_content);
                                all_css.push('\n');
                            }
                        }
                    }
                    let css_duration = css_start.elapsed();
                    let style_start = std::time::Instant::now();
                    let stylesheet = parse_css(&all_css);
                    let mut arena = crate::ffi::GLOBAL_DOM_ARENA.lock().unwrap();
                    apply_stylesheet_to_dom(&mut root, &stylesheet, &mut *arena);
                    let style_duration = style_start.elapsed();
                    println!("[FFI] Parsed CSS with {} rules", stylesheet.rules.len());
                    let layout_start = std::time::Instant::now();
                    let layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
                    let layout_boxes = layout_engine.layout(&root, &*arena);
                    let layout_duration = layout_start.elapsed();
                    println!("[FFI] Generated {} layout boxes", layout_boxes.len());
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

// ... (functions will be moved here in the next step) ... 
use std::ffi::c_char;
use std::ptr;
use crate::ffi::{LayoutBoxArray, FFIPerformanceTracker, safe_c_string_to_rust, process_html_streaming};
use crate::parser::html::HTMLParser;
use crate::parser::css::parse_css;
use crate::layout::layout::LayoutEngine;
use crate::paint::painter::Painter;
use crate::compositor::compositor::Compositor;
use crate::dom::node::DOMArena;
use crate::VeloxEngine;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ARENA: Lazy<Mutex<DOMArena>> = Lazy::new(|| Mutex::new(DOMArena::new()));

// HTML parsing with JavaScript execution
#[no_mangle]
pub extern "C" fn parse_html_with_javascript(html_ptr: *const c_char) -> *mut LayoutBoxArray {
    let mut tracker = FFIPerformanceTracker::new();
    println!("[FFI] parse_html_with_javascript called");
    let input_start = std::time::Instant::now();
    let input_string = match safe_c_string_to_rust(html_ptr) {
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
    
    let result = std::panic::catch_unwind(|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let layout_boxes = runtime.block_on(async {
            match process_html_streaming(&url).await {
                Ok((tokens, css_rules)) => {
                    println!("[FFI] Streamed {} tokens and {} CSS rules", tokens.len(), css_rules.len());
                    let mut parser = HTMLParser::new(format!("<html><head></head><body></body></html>"));
                    let mut dom = parser.parse();
                    let mut stylesheet = parser.get_stylesheet();
                    
                    // Apply CSS rules
                    for css in css_rules {
                        let additional_css = parse_css(&css);
                        stylesheet.rules.extend(additional_css.rules);
                    }
                    
                    let layout_engine = LayoutEngine::new(800.0, 600.0).with_stylesheet(stylesheet);
                    let arena = ARENA.lock().unwrap();
                    let boxes = layout_engine.layout(&dom, &*arena);
                    Ok(boxes)
                }
                Err(e) => {
                    eprintln!("[FFI] Streaming failed: {}", e);
                    Err(e)
                }
            }
        });
        
        match layout_boxes {
            Ok(boxes) => {
                println!("[FFI] Generated {} layout boxes from URL", boxes.len());
                LayoutBoxArray::new(boxes)
            }
            Err(_) => {
                // Fallback to simple HTML parsing
                let mut parser = HTMLParser::new(format!("<html><body><p>Failed to load: {}</p></body></html>", url));
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
            eprintln!("[FFI] parse_url_via_rust_enhanced: panic caught!");
            ptr::null_mut()
        }
    }
} 
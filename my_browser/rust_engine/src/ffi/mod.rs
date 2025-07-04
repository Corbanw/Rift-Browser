// FFI bridge modules for browser rendering engine
// Provides C-compatible interface for layout boxes and draw commands

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use tokio::runtime::Runtime;
use reqwest::Client as AsyncClient;
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use crate::dom::node::{DOMNode, LayoutBox, FFILayoutBox, NodeType, StyleMap, BoxValues, DOMArena};
use crate::parser::html::{HTMLParser, StreamingHTMLParser};
use crate::parser::css::{parse_css, Stylesheet};
use crate::layout::layout::LayoutEngine;
use crate::paint::painter::Painter;
use crate::compositor::compositor::Compositor;

// Include the functions module
pub mod functions;

// Enhanced FFI structures for better batching and performance
#[repr(C)]
pub struct LayoutBoxArray {
    pub boxes: Vec<*mut FFILayoutBox>,
    pub total_count: i32,
    pub batch_size: i32,
}

#[repr(C)]
pub struct DrawCommand {
    pub command_type: i32, // 0=rect, 1=text, 2=line, 3=image
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: *mut c_char,
    pub text: *mut c_char,
    pub font_size: f32,
    pub font_weight: f32,
}

#[repr(C)]
pub struct DrawCommandArray {
    pub commands: Vec<*mut DrawCommand>,
    pub total_count: i32,
    pub batch_size: i32,
}

// Performance tracking for FFI calls
#[derive(Debug)]
pub struct FFIPerformanceTracker {
    pub start_time: std::time::Instant,
    pub stage_times: std::collections::HashMap<String, std::time::Duration>,
}

// Implement required traits for FFI safety
impl std::panic::UnwindSafe for FFIPerformanceTracker {}
impl std::panic::RefUnwindSafe for FFIPerformanceTracker {}

impl FFIPerformanceTracker {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            stage_times: std::collections::HashMap::new(),
        }
    }

    pub fn record_stage(&mut self, stage: &str, duration: std::time::Duration) {
        self.stage_times.insert(stage.to_string(), duration);
    }

    pub fn log_performance(&self) {
        let total = self.start_time.elapsed();
        println!("[PERF] FFI Total: {}ms", total.as_millis());
        for (stage, duration) in &self.stage_times {
            println!("[PERF] FFI {}: {}ms", stage, duration.as_millis());
        }
    }
}

impl LayoutBoxArray {
    pub fn new(boxes: Vec<LayoutBox>) -> Self {
        let ffi_boxes: Vec<*mut FFILayoutBox> = boxes.into_iter()
            .map(|b| Box::into_raw(Box::new(b.to_ffi())))
            .collect();
        let total_count = ffi_boxes.len() as i32;
        LayoutBoxArray { 
            boxes: ffi_boxes, 
            total_count,
            batch_size: 100, // Default batch size
        }
    }

    pub fn with_batch_size(mut self, batch_size: i32) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn get_batch(&self, start: i32, count: i32) -> Vec<*mut FFILayoutBox> {
        let start = start as usize;
        let end = (start + count as usize).min(self.boxes.len());
        self.boxes[start..end].to_vec()
    }
}

impl DrawCommandArray {
    pub fn new(commands: Vec<DrawCommand>) -> Self {
        let ffi_commands: Vec<*mut DrawCommand> = commands.into_iter()
            .map(|c| Box::into_raw(Box::new(c)))
            .collect();
        let total_count = ffi_commands.len() as i32;
        DrawCommandArray { 
            commands: ffi_commands, 
            total_count,
            batch_size: 50, // Default batch size for draw commands
        }
    }

    pub fn with_batch_size(mut self, batch_size: i32) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn get_batch(&self, start: i32, count: i32) -> Vec<*mut DrawCommand> {
        let start = start as usize;
        let end = (start + count as usize).min(self.commands.len());
        self.commands[start..end].to_vec()
    }
}

// Helper functions for FFI operations
pub fn safe_c_string_to_rust(c_ptr: *const c_char) -> Result<String, String> {
    if c_ptr.is_null() {
        return Err("Null pointer provided".to_string());
    }
    
    let cstr = unsafe { CStr::from_ptr(c_ptr) };
    match cstr.to_str() {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(format!("Invalid UTF-8: {}", e)),
    }
}

pub fn safe_rust_string_to_c(s: &str) -> *mut c_char {
    match CString::new(s) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

// Enhanced selector matching for CSS
pub fn matches_selector(node: &DOMNode, selector: &str) -> bool {
    match &node.node_type {
        NodeType::Element(tag_name) => {
            if selector == tag_name {
                return true;
            }
            
            if selector.starts_with('.') {
                let class_name = &selector[1..];
                if let Some(classes) = node.attributes.get("class") {
                    return classes.split_whitespace().any(|c| c == class_name);
                }
            }
            
            if selector.starts_with('#') {
                let id_name = &selector[1..];
                if let Some(id) = node.attributes.get("id") {
                    return id == id_name;
                }
            }
            
            false
        }
        _ => false,
    }
}

// Apply CSS stylesheet to DOM
pub fn apply_stylesheet_to_dom(dom: &mut DOMNode, stylesheet: &Stylesheet, arena: &mut DOMArena) {
    fn recurse(node: &mut DOMNode, stylesheet: &Stylesheet, arena: &mut DOMArena) {
        if let NodeType::Element(_) = &node.node_type {
            let mut style_map = std::collections::HashMap::new();
            let tag = match &node.node_type {
                NodeType::Element(t) => t.as_str(),
                _ => "",
            };
            let class_attr = node.attributes.get("class").cloned().unwrap_or_default();
            let id_attr = node.attributes.get("id").cloned().unwrap_or_default();
            
            for rule in &stylesheet.rules {
                let sel = rule.selector.trim();
                if matches_selector(node, sel) {
                    println!("[CSS MATCH] selector='{}' -> <{} class='{}' id='{}'>", sel, tag, class_attr, id_attr);
                    for (k, v) in &rule.declarations {
                        style_map.insert(k.clone(), v.clone());
                    }
                }
            }
            // Convert HashMap to StyleMap
            let mut style_map_obj = StyleMap::default();
            for (k, v) in &style_map {
                style_map_obj.set_property(k, v);
            }
            node.styles = style_map_obj;
            if !style_map.is_empty() {
                println!("[STYLE] <{} class='{}' id='{}'> styles: {:?}", tag, class_attr, id_attr, style_map);
            }
        }
        for child_id in &node.children {
            if let Some(child_node) = arena.get_node(child_id) {
                let mut child = child_node.lock().unwrap();
                recurse(&mut child, stylesheet, arena);
            }
        }
    }
    recurse(dom, stylesheet, arena);
}

// Async HTML processing with streaming
pub async fn process_html_streaming(url: &str) -> Result<(Vec<crate::parser::html::Token>, Vec<String>), Box<dyn std::error::Error>> {
    let client = AsyncClient::new();
    let response = client.get(url).send().await?;
    let mut stream = response.bytes_stream();
    let mut parser = StreamingHTMLParser::new();
    let mut all_tokens = Vec::new();
    
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        if let Ok(chunk_str) = String::from_utf8(bytes.to_vec()) {
            // Process chunk and collect tokens
            let new_tokens = parser.process_chunk(&chunk_str);
            all_tokens.extend(new_tokens);
        }
    }
    
    // Also get any remaining tokens from the parser
    all_tokens.extend(parser.get_tokens().to_vec());
    
    println!("[STREAMING] Total tokens collected: {}", all_tokens.len());
    Ok((all_tokens, parser.get_extracted_css().to_vec()))
} 

pub use self::functions::{
    dom_get_parent_node,
    dom_get_child_nodes,
    dom_get_first_child,
    dom_get_last_child,
    dom_get_next_sibling,
    dom_get_previous_sibling,
    dom_insert_before,
    dom_replace_child,
    dom_clone_node,
    dom_remove_node,
    dom_contains_node,
}; 

lazy_static! {
    pub static ref GLOBAL_DOM_ARENA: Arc<Mutex<DOMArena>> = Arc::new(Mutex::new(DOMArena::new()));
}

pub fn get_global_arena() -> std::sync::MutexGuard<'static, DOMArena> {
    GLOBAL_DOM_ARENA.lock().unwrap()
} 
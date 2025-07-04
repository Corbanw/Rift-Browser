// Main library file - modular browser rendering engine
// This orchestrates the pipeline: HTML/CSS parsing -> DOM -> Style -> Layout -> Paint -> Compositor -> FFI

// Core modules
pub mod parser;
pub mod dom;
pub mod style;
pub mod layout;
pub mod paint;
pub mod compositor;
pub mod ffi;
pub mod javascript;

// Re-export commonly used types for convenience
pub use dom::node::{DOMNode, LayoutBox, FFILayoutBox, NodeType, StyleMap, BoxValues};
pub use parser::html::{HTMLParser, StreamingHTMLParser};
pub use parser::css::{parse_css, Stylesheet};
pub use layout::layout::LayoutEngine;
pub use paint::painter::Painter;
pub use compositor::compositor::Compositor;
pub use javascript::{JavaScriptRuntime, ScriptManager};

// Re-export FFI types and functions
pub use ffi::{LayoutBoxArray, DrawCommand, DrawCommandArray, FFIPerformanceTracker};
pub use ffi::functions::*;

// Main entry point for the Velox browser rendering engine
pub struct VeloxEngine {
    pub layout_engine: LayoutEngine,
    pub painter: Painter,
    pub compositor: Compositor,
    pub script_manager: Option<ScriptManager>,
}

impl VeloxEngine {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            layout_engine: LayoutEngine::new(width, height),
            painter: Painter::new(),
            compositor: Compositor::new(),
            script_manager: None,
        }
    }

    pub fn with_stylesheet(mut self, stylesheet: Stylesheet) -> Self {
        self.layout_engine = self.layout_engine.with_stylesheet(stylesheet);
        self
    }

    /// Initialize JavaScript runtime with DOM tree
    pub fn init_javascript(&mut self, dom: &DOMNode) -> Result<(), Box<dyn std::error::Error>> {
        let mut script_manager = ScriptManager::new(ffi::GLOBAL_DOM_ARENA.clone(), dom.id.clone())?;
        script_manager.initialize()?;
        self.script_manager = Some(script_manager);
        Ok(())
    }

    /// Execute JavaScript code
    pub fn execute_script(&mut self, script_content: &str, script_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(script_manager) = &mut self.script_manager {
            script_manager.execute_script(script_content, script_name)?;
        }
        Ok(())
    }

    /// Execute external JavaScript from URL
    pub async fn execute_external_script(&mut self, script_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(script_manager) = &mut self.script_manager {
            script_manager.execute_external_script(script_url).await?;
        }
        Ok(())
    }

    /// Run JavaScript event loop
    pub fn run_js_event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(script_manager) = &mut self.script_manager {
            script_manager.run_event_loop()?;
        }
        Ok(())
    }

    pub fn render_html(&self, html: &str) -> Vec<LayoutBox> {
        // Parse HTML
        let mut parser = HTMLParser::new(html.to_string());
        let dom = parser.parse();
        let stylesheet = parser.get_stylesheet();

        // Apply styles
        let mut styled_dom = dom.clone();
        {
            let mut arena = ffi::GLOBAL_DOM_ARENA.lock().unwrap();
            ffi::apply_stylesheet_to_dom(&mut styled_dom, &stylesheet, &mut *arena);
        }
        // Layout
        let layout_engine = self.layout_engine.clone().with_stylesheet(stylesheet);
        layout_engine.layout(&styled_dom, &ffi::GLOBAL_DOM_ARENA.lock().unwrap())
    }

    /// Render HTML with JavaScript execution
    pub async fn render_html_with_js(&mut self, html: &str) -> Result<Vec<LayoutBox>, Box<dyn std::error::Error>> {
        // Parse HTML
        let mut parser = HTMLParser::new(html.to_string());
        let dom = parser.parse();
        let stylesheet = parser.get_stylesheet();

        // Initialize JavaScript runtime if not already done
        if self.script_manager.is_none() {
            self.init_javascript(&dom)?;
        }

        // Execute inline scripts
        for (i, script_content) in parser.get_extracted_scripts().iter().enumerate() {
            let script_name = format!("inline_script_{}", i);
            self.execute_script(script_content, &script_name)?;
        }

        // Execute external scripts
        for script_url in parser.get_script_src_urls() {
            self.execute_external_script(script_url).await?;
        }

        // Apply styles
        let mut styled_dom = dom.clone();
        {
            let mut arena = ffi::GLOBAL_DOM_ARENA.lock().unwrap();
            ffi::apply_stylesheet_to_dom(&mut styled_dom, &stylesheet, &mut *arena);
        }
        // Layout
        let layout_engine = self.layout_engine.clone().with_stylesheet(stylesheet);
        let layout_boxes = layout_engine.layout(&styled_dom, &ffi::GLOBAL_DOM_ARENA.lock().unwrap());

        // Run JavaScript event loop for any pending operations
        self.run_js_event_loop()?;

        Ok(layout_boxes)
    }

    pub fn render_url(&self, url: &str) -> Result<Vec<LayoutBox>, Box<dyn std::error::Error>> {
        // This would use the async streaming parser in a real implementation
        // For now, return an error indicating this needs to be implemented
        Err("URL rendering not yet implemented".into())
    }
}

// Default implementation for common use cases
impl Default for VeloxEngine {
    fn default() -> Self {
        Self::new(800.0, 600.0)
    }
}

// Export the main engine for external use
pub fn create_velox_engine(width: f32, height: f32) -> VeloxEngine {
    VeloxEngine::new(width, height)
}

// Convenience function for quick HTML rendering
pub fn render_html_quick(html: &str) -> Vec<LayoutBox> {
    let engine = VeloxEngine::default();
    engine.render_html(html)
}

// Test function to verify the engine is working
pub fn test_engine() -> bool {
    let test_html = r#"
        <html>
            <head><style>body { background: red; }</style></head>
            <body>
                <h1>Test</h1>
                <p>Hello World</p>
            </body>
        </html>
    "#;
    
    let boxes = render_html_quick(test_html);
    !boxes.is_empty()
}

// Performance monitoring utilities
pub struct PerformanceMonitor {
    start_time: std::time::Instant,
    stages: std::collections::HashMap<String, std::time::Duration>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            stages: std::collections::HashMap::new(),
        }
    }

    pub fn record_stage(&mut self, name: &str, duration: std::time::Duration) {
        self.stages.insert(name.to_string(), duration);
    }

    pub fn log_summary(&self) {
        let total = self.start_time.elapsed();
        println!("[PERF] Total rendering time: {}ms", total.as_millis());
        for (stage, duration) in &self.stages {
            let percentage = (duration.as_millis() as f64 / total.as_millis() as f64) * 100.0;
            println!("[PERF] {}: {}ms ({:.1}%)", stage, duration.as_millis(), percentage);
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
} 

pub use ffi::{
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
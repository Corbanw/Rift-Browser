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

// Re-export commonly used types for convenience
pub use dom::node::{DOMNode, LayoutBox, FFILayoutBox, NodeType, StyleMap, BoxValues};
pub use parser::html::{HTMLParser, StreamingHTMLParser};
pub use parser::css::{parse_css, Stylesheet};
pub use layout::layout::LayoutEngine;
pub use paint::painter::Painter;
pub use compositor::compositor::Compositor;

// Re-export FFI types and functions
pub use ffi::{LayoutBoxArray, DrawCommand, DrawCommandArray, FFIPerformanceTracker};
pub use ffi::functions::*;

// Main entry point for the browser rendering engine
pub struct BrowserEngine {
    pub layout_engine: LayoutEngine,
    pub painter: Painter,
    pub compositor: Compositor,
}

impl BrowserEngine {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            layout_engine: LayoutEngine::new(width, height),
            painter: Painter::new(),
            compositor: Compositor::new(),
        }
    }

    pub fn with_stylesheet(mut self, stylesheet: Stylesheet) -> Self {
        self.layout_engine = self.layout_engine.with_stylesheet(stylesheet);
        self
    }

    pub fn render_html(&self, html: &str) -> Vec<LayoutBox> {
        // Parse HTML
        let mut parser = HTMLParser::new(html.to_string());
        let dom = parser.parse();
        let stylesheet = parser.get_stylesheet();

        // Apply styles
        let mut styled_dom = dom.clone();
        ffi::apply_stylesheet_to_dom(&mut styled_dom, &stylesheet);

        // Layout
        let layout_engine = self.layout_engine.clone().with_stylesheet(stylesheet);
        layout_engine.layout(&styled_dom)
    }

    pub fn render_url(&self, url: &str) -> Result<Vec<LayoutBox>, Box<dyn std::error::Error>> {
        // This would use the async streaming parser in a real implementation
        // For now, return an error indicating this needs to be implemented
        Err("URL rendering not yet implemented".into())
    }
}

// Default implementation for common use cases
impl Default for BrowserEngine {
    fn default() -> Self {
        Self::new(800.0, 600.0)
    }
}

// Export the main engine for external use
pub fn create_browser_engine(width: f32, height: f32) -> BrowserEngine {
    BrowserEngine::new(width, height)
}

// Convenience function for quick HTML rendering
pub fn render_html_quick(html: &str) -> Vec<LayoutBox> {
    let engine = BrowserEngine::default();
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
use crate::dom::node::{DOMNode, LayoutBox, NodeType, StyleMap, BoxValues};
use crate::parser::css::{parse_inline_styles, Stylesheet};
use std::time::Instant;
use crate::paint::display_list::{DrawCommand, DisplayList};
use crate::paint::painter::Painter;
use crate::compositor::compositor::Compositor;
use crate::ffi::matches_selector;
use crate::dom::node::DOMArena;

#[derive(Debug, Clone)]
pub struct LayoutStats {
    pub total_nodes_processed: usize,
    pub layout_boxes_created: usize,
    pub parallel_batches: usize,
    pub layout_time_ms: u64,
    pub memory_usage_mb: f64,
    pub nodes_per_second: f64,
    pub boxes_per_second: f64,
}

impl Default for LayoutStats {
    fn default() -> Self {
        Self {
            total_nodes_processed: 0,
            layout_boxes_created: 0,
            parallel_batches: 0,
            layout_time_ms: 0,
            memory_usage_mb: 0.0,
            nodes_per_second: 0.0,
            boxes_per_second: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayoutEngine {
    viewport_width: f32,
    viewport_height: f32,
    pub stylesheet: Option<Stylesheet>,
    pub layout_stats: LayoutStats,
}

impl LayoutEngine {
    // Advanced limits for complex layouts
    pub const MAX_LAYOUT_BOXES: usize = 100_000; // 100K boxes max
    pub const MAX_DOM_NODES: usize = 200_000; // 200K nodes max
    pub const MAX_LAYOUT_TIME_MS: u64 = 60_000; // 60 seconds max
    pub const MAX_PARALLEL_THREADS: usize = 16; // Max parallel threads
    pub const CHUNK_SIZE: usize = 1000; // Process in 1K node chunks
    pub const PROGRESS_INTERVAL: usize = 5000; // Log progress every 5K nodes
    pub const MEMORY_CHECK_INTERVAL: usize = 10000; // Check memory every 10K nodes

    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        println!("Rust: Layout Engine initialized with viewport: {}x{}", viewport_width, viewport_height);
        Self {
            viewport_width,
            viewport_height,
            stylesheet: None,
            layout_stats: LayoutStats::default(),
        }
    }

    pub fn with_stylesheet(mut self, stylesheet: Stylesheet) -> Self {
        self.stylesheet = Some(stylesheet);
        self
    }

    /// Find the <body> node in the DOM tree, or return the given node if not found
    fn find_body_node_id(&self, node: &DOMNode, arena: &DOMArena) -> Option<String> {
        match &node.node_type {
            NodeType::Element(tag) if tag.eq_ignore_ascii_case("body") => {
                Some(node.id.clone())
            },
            _ => {
                for child_id in &node.children {
                    if let Some(child_node) = arena.get_node(child_id) {
                        let child = child_node.lock().unwrap();
                        if let Some(found_id) = self.find_body_node_id(&child, arena) {
                            return Some(found_id);
                        }
                    }
                }
                None
            }
        }
    }

    /// Basic block/inline layout algorithm
    pub fn layout(&self, dom: &DOMNode, arena: &DOMArena) -> Vec<LayoutBox> {
        println!("[LAYOUT] Starting basic block/inline layout");
        let layout_root_id = self.find_body_node_id(dom, arena).unwrap_or_else(|| dom.id.clone());
        let layout_root = match arena.get_node(&layout_root_id) {
            Some(node) => node,
            None => {
                eprintln!("[LAYOUT] Error: Layout root not found for id {}. Returning empty layout.", layout_root_id);
                return Vec::new();
            }
        };
        let layout_root = layout_root.lock().unwrap();
        println!("[LAYOUT] Using {:?} as layout root", layout_root.node_type);
        
        let mut boxes = Vec::new();
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        let mut line_height = 0.0;
        let mut in_inline_context = false;
        
        self.layout_node(&layout_root, arena, &mut boxes, &mut current_x, &mut current_y, &mut line_height, &mut in_inline_context, 0);
        
        println!("[LAYOUT] Basic layout completed: {} boxes created", boxes.len());
        boxes
    }
    
    fn layout_node(&self, node: &DOMNode, arena: &DOMArena, boxes: &mut Vec<LayoutBox>, current_x: &mut f32, current_y: &mut f32, line_height: &mut f32, in_inline_context: &mut bool, depth: usize) {
        let styles = self.get_node_styles(node);
        let display = styles.display.to_lowercase();
        
        match &node.node_type {
            NodeType::Element(tag_name) => {
                let is_block = display == "block" || tag_name == "div" || tag_name == "p" || tag_name == "h1" || tag_name == "h2" || tag_name == "h3" || tag_name == "h4" || tag_name == "h5" || tag_name == "h6" || tag_name == "section" || tag_name == "article" || tag_name == "header" || tag_name == "footer" || tag_name == "nav" || tag_name == "main" || tag_name == "aside";
                let is_inline = display == "inline" || tag_name == "span" || tag_name == "a" || tag_name == "strong" || tag_name == "em" || tag_name == "b" || tag_name == "i" || tag_name == "u" || tag_name == "code" || tag_name == "small";
                
                if is_block {
                    // Block element: start new line
                    if *in_inline_context {
                        *current_x = 0.0;
                        *current_y += *line_height;
                        *line_height = 0.0;
                        *in_inline_context = false;
                    }
                    
                    let (width, height) = self.calculate_block_dimensions(&styles, tag_name);
                    let margin = parse_box_value(&styles.margin);
                    let padding = parse_box_value(&styles.padding);
                    
                    // Apply margin
                    *current_x += margin.left;
                    *current_y += margin.top;
                    
                    let box_layout = LayoutBox {
                        x: *current_x,
                        y: *current_y,
                        width: width + padding.left + padding.right,
                        height: height + padding.top + padding.bottom,
                        node_type: tag_name.clone(),
                        text_content: self.extract_text_content(node, arena),
                        background_color: styles.background_color.clone(),
                        color: styles.color.clone(),
                        font_size: styles.font_size.parse().unwrap_or(16.0),
                        font_family: styles.font_family.clone(),
                        border_color: styles.border_color.clone(),
                        border_width: parse_box_value(&styles.border_width),
                        margin: margin.clone(),
                        padding: padding.clone(),
                        font_weight: styles.font_weight.parse().unwrap_or(400.0),
                        text_align: styles.text_align.clone(),
                        flex_direction: styles.flex_direction.clone(),
                        flex_wrap: styles.flex_wrap.clone(),
                        justify_content: styles.justify_content.clone(),
                        align_items: styles.align_items.clone(),
                        flex_grow: styles.flex_grow.parse().unwrap_or(0.0),
                        flex_shrink: styles.flex_shrink.parse().unwrap_or(1.0),
                        flex_basis: styles.flex_basis.clone(),
                        order: styles.order.parse().unwrap_or(0),
                        grid_column: styles.grid_column.clone(),
                        grid_row: styles.grid_row.clone(),
                        line_height: styles.line_height.parse().unwrap_or(1.2),
                        word_wrap: styles.word_wrap.clone(),
                        white_space: styles.white_space.clone(),
                        text_overflow: styles.text_overflow.clone(),
                        color_scheme: styles.color_scheme.clone()
                    };
                    
                    boxes.push(box_layout);
                    
                    // Move to next line
                    *current_x = 0.0;
                    *current_y += height + padding.top + padding.bottom + margin.bottom;
                    *line_height = 0.0;
                    
                    // Layout children
                    for child_id in &node.children {
                        if let Some(child_node) = arena.get_node(child_id) {
                            let child = child_node.lock().unwrap();
                            self.layout_node(&child, arena, boxes, current_x, current_y, line_height, in_inline_context, depth + 1);
                        }
                    }
                    
                } else if is_inline {
                    // Inline element: continue on same line
                    *in_inline_context = true;
                    
                    let text_content = self.extract_text_content(node, arena);
                    let font_size = styles.font_size.parse().unwrap_or(16.0);
                    let estimated_width = text_content.len() as f32 * font_size * 0.6; // Rough estimate
                    let estimated_height = font_size * 1.2;
                    
                    let margin = parse_box_value(&styles.margin);
                    let padding = parse_box_value(&styles.padding);
                    
                    // Check if we need to wrap to next line
                    if *current_x + estimated_width + margin.left + margin.right + padding.left + padding.right > self.viewport_width * 0.9 {
                        *current_x = 0.0;
                        *current_y += *line_height;
                        *line_height = 0.0;
                    }
                    
                    *current_x += margin.left;
                    
                    let box_layout = LayoutBox {
                        x: *current_x,
                        y: *current_y,
                        width: estimated_width + padding.left + padding.right,
                        height: estimated_height + padding.top + padding.bottom,
                        node_type: tag_name.clone(),
                        text_content: text_content.clone(),
                        background_color: styles.background_color.clone(),
                        color: styles.color.clone(),
                        font_size: font_size,
                        font_family: styles.font_family.clone(),
                        border_color: styles.border_color.clone(),
                        border_width: parse_box_value(&styles.border_width),
                        margin: margin.clone(),
                        padding: padding.clone(),
                        font_weight: styles.font_weight.parse().unwrap_or(400.0),
                        text_align: styles.text_align.clone(),
                        flex_direction: styles.flex_direction.clone(),
                        flex_wrap: styles.flex_wrap.clone(),
                        justify_content: styles.justify_content.clone(),
                        align_items: styles.align_items.clone(),
                        flex_grow: styles.flex_grow.parse().unwrap_or(0.0),
                        flex_shrink: styles.flex_shrink.parse().unwrap_or(1.0),
                        flex_basis: styles.flex_basis.clone(),
                        order: styles.order.parse().unwrap_or(0),
                        grid_column: styles.grid_column.clone(),
                        grid_row: styles.grid_row.clone(),
                        line_height: styles.line_height.parse().unwrap_or(1.2),
                        word_wrap: styles.word_wrap.clone(),
                        white_space: styles.white_space.clone(),
                        text_overflow: styles.text_overflow.clone(),
                        color_scheme: styles.color_scheme.clone()
                    };
                    
                    boxes.push(box_layout);
                    
                    *current_x += estimated_width + padding.left + padding.right + margin.right;
                    *line_height = (*line_height).max(estimated_height + padding.top + padding.bottom);
                    
                    // Layout children
                    for child_id in &node.children {
                        if let Some(child_node) = arena.get_node(child_id) {
                            let child = child_node.lock().unwrap();
                            self.layout_node(&child, arena, boxes, current_x, current_y, line_height, in_inline_context, depth + 1);
                        }
                    }
                    
                } else {
                    // Default to block behavior for unknown elements
                    for child_id in &node.children {
                        if let Some(child_node) = arena.get_node(child_id) {
                            let child = child_node.lock().unwrap();
                            self.layout_node(&child, arena, boxes, current_x, current_y, line_height, in_inline_context, depth + 1);
                        }
                    }
                }
            },
            NodeType::Text => {
                // Text node: create inline text box
                let text_content = node.text_content.trim();
                if !text_content.is_empty() {
                    let font_size = 16.0; // Default font size
                    let estimated_width = text_content.len() as f32 * font_size * 0.6;
                    let estimated_height = font_size * 1.2;
                    
                    // Check if we need to wrap to next line
                    if *current_x + estimated_width > self.viewport_width * 0.9 {
                        *current_x = 0.0;
                        *current_y += *line_height;
                        *line_height = 0.0;
                        *in_inline_context = false;
                    }
                    
                    let box_layout = LayoutBox {
                        x: *current_x,
                        y: *current_y,
                        width: estimated_width,
                        height: estimated_height,
                        node_type: "text".to_string(),
                        text_content: text_content.to_string(),
                        background_color: "transparent".to_string(),
                        color: "#000000".to_string(),
                        font_size: font_size,
                        font_family: "Arial".to_string(),
                        border_color: "transparent".to_string(),
                        border_width: BoxValues::default(),
                        margin: BoxValues::default(),
                        padding: BoxValues::default(),
                        font_weight: 400.0,
                        text_align: "left".to_string(),
                        flex_direction: "row".to_string(),
                        flex_wrap: "nowrap".to_string(),
                        justify_content: "flex-start".to_string(),
                        align_items: "stretch".to_string(),
                        flex_grow: 0.0,
                        flex_shrink: 1.0,
                        flex_basis: "auto".to_string(),
                        order: 0,
                        grid_column: "auto".to_string(),
                        grid_row: "auto".to_string(),
                        line_height: 1.2,
                        word_wrap: "normal".to_string(),
                        white_space: "normal".to_string(),
                        text_overflow: "clip".to_string(),
                        color_scheme: "light".to_string()
                    };
                    
                    boxes.push(box_layout);
                    
                    *current_x += estimated_width;
                    *line_height = (*line_height).max(estimated_height);
                    *in_inline_context = true;
                }
            },
            _ => {
                // Other node types: just process children
                for child_id in &node.children {
                    if let Some(child_node) = arena.get_node(child_id) {
                        let child = child_node.lock().unwrap();
                        self.layout_node(&child, arena, boxes, current_x, current_y, line_height, in_inline_context, depth + 1);
                    }
                }
            }
        }
    }
    
    fn calculate_block_dimensions(&self, styles: &StyleMap, tag_name: &str) -> (f32, f32) {
        let width = self.parse_length(&styles.width, self.viewport_width * 0.9);
        let height = self.parse_length(&styles.height, if tag_name == "p" { 20.0 } else { 100.0 });
        
        // Apply viewport constraints
        let max_width = self.viewport_width * 0.9;
        let max_height = self.viewport_height * 0.9;
        
        (width.min(max_width), height.min(max_height))
    }

    fn print_dom_tree(&self, node: &DOMNode, depth: usize, arena: &DOMArena) {
        let indent = "  ".repeat(depth);
        match &node.node_type {
            NodeType::Element(tag_name) => {
                println!("{}<{}> ({} children)", indent, tag_name, node.children.len());
                if depth < 3 { // Limit depth for large trees
                    for child_id in &node.children {
                        if let Some(child_node) = arena.get_node(child_id) {
                            let child = child_node.lock().unwrap();
                            self.print_dom_tree(&child, depth + 1, arena);
                        }
                    }
                } else if !node.children.is_empty() {
                    println!("{}... ({} more children)", indent, node.children.len());
                }
            }
            NodeType::Text => {
                let text = node.text_content.trim();
                if !text.is_empty() && text.len() < 100 {
                    println!("{}Text: '{}'", indent, text);
                } else if !text.is_empty() {
                    println!("{}Text: '{}...' ({} chars)", indent, &text[..50], text.len());
                }
            }
            NodeType::Document => {
                println!("{}Document ({} children)", indent, node.children.len());
                if depth < 3 {
                    for child_id in &node.children {
                        if let Some(child_node) = arena.get_node(child_id) {
                            let child = child_node.lock().unwrap();
                            self.print_dom_tree(&child, depth + 1, arena);
                        }
                    }
                }
            }
        }
    }

    fn should_skip_element(&self, tag_name: &str) -> bool {
        let skip_tags = [
            "script", "style", "meta", "link", "title", "head", 
            "noscript", "template", "svg", "math", "canvas",
            "iframe", "object", "embed", "applet", "param",
            "source", "track", "area", "map", "picture", "audio", "video"
        ];
        skip_tags.contains(&tag_name.to_lowercase().as_str())
    }

    fn is_layout_important(&self, tag_name: &str) -> bool {
        let important_tags = [
            "body", "div", "span", "p", "h1", "h2", "h3", "h4", "h5", "h6",
            "a", "img", "input", "button", "form", "table", "tr", "td", "th",
            "ul", "ol", "li", "nav", "header", "footer", "main", "section",
            "article", "aside", "figure", "figcaption", "blockquote", "pre",
            "code", "strong", "em", "b", "i", "u", "br", "hr", "center",
            "fieldset", "legend", "label", "select", "textarea", "option"
        ];
        important_tags.contains(&tag_name.to_lowercase().as_str())
    }

    fn should_process_node(&self, node: &DOMNode, depth: usize) -> bool {
        match &node.node_type {
            NodeType::Element(tag_name) => {
                // Skip non-important elements at deep levels
                if depth > 15 && !self.is_layout_important(tag_name) {
                    return false;
                }
                // Always process important elements
                if self.is_layout_important(tag_name) {
                    return true;
                }
                // Skip elements that don't contribute to layout
                !self.should_skip_element(tag_name)
            }
            NodeType::Text => {
                let text = node.text_content.trim();
                !text.is_empty() && text.len() > 1
            }
            NodeType::Document => true,
        }
    }

    fn layout_node_advanced(&self, node: &DOMNode, x: f32, y: f32, boxes: &mut Vec<LayoutBox>, depth: usize, node_count: &mut usize, arena: &DOMArena) -> (Vec<LayoutBox>, (f32, f32)) {
        use std::collections::{HashSet, VecDeque};
        
        let mut queue = VecDeque::with_capacity(1000);
        let mut processed_nodes = HashSet::new();
        let mut local_boxes = Vec::new();
        
        if self.should_process_node(node, depth) {
            if depth <= 3 {
                match &node.node_type {
                    NodeType::Element(tag) => println!("[ENQUEUE] <{}> at depth {}", tag, depth),
                    NodeType::Text => println!("[ENQUEUE] <text> at depth {}", depth),
                    NodeType::Document => println!("[ENQUEUE] <document> at depth {}", depth),
                }
            }
            queue.push_back((node, x, y, depth));
        }

        let mut current_x = x;
        let mut current_y = y;
        let mut max_height: f32 = 0.0;
        let mut iterations = 0;
        let mut batch_count = 0;
        let mut consecutive_no_progress = 0;
        let start_time = Instant::now();
        let mut last_progress_time = start_time;
        let mut last_queue_size = queue.len();
        let mut last_boxes_count = local_boxes.len();
        
        println!("[LAYOUT] [ADVANCED] Starting layout with initial queue size: {}", queue.len());
        
        while let Some((current_node, node_x, node_y, node_depth)) = queue.pop_front() {
            iterations += 1;
            *node_count += 1;
            
            // Progress logging
            if node_depth <= 3 {
                match &current_node.node_type {
                    NodeType::Element(tag) => println!("[PROCESS] <{}> at depth {} (queue: {})", tag, node_depth, queue.len()),
                    NodeType::Text => println!("[PROCESS] <text> at depth {} (queue: {})", node_depth, queue.len()),
                    NodeType::Document => println!("[PROCESS] <document> at depth {} (queue: {})", node_depth, queue.len()),
                }
            }
            
            // Memory and performance checks
            if iterations % Self::MEMORY_CHECK_INTERVAL == 0 {
                let elapsed = start_time.elapsed();
                if elapsed.as_secs() > Self::MAX_LAYOUT_TIME_MS / 1000 {
                    println!("[LAYOUT] [ADVANCED] TIMEOUT: Layout taking too long ({} iterations), stopping", iterations);
                            break;
                }
                
                if local_boxes.len() > Self::MAX_LAYOUT_BOXES {
                    println!("[LAYOUT] [ADVANCED] Box limit reached ({} boxes), stopping", local_boxes.len());
                    break;
                }
                
                if *node_count > Self::MAX_DOM_NODES {
                    println!("[LAYOUT] [ADVANCED] Node limit reached ({} nodes), stopping", *node_count);
                    break;
                }
            }
            
            let node_id = format!("{:p}_{}", current_node as *const _, node_depth);
            if processed_nodes.contains(&node_id) {
                consecutive_no_progress += 1;
                if consecutive_no_progress > 100 {
                    println!("[LAYOUT] [ADVANCED] Too many consecutive no-progress iterations, stopping");
                    break;
                }
                continue;
            }
            processed_nodes.insert(node_id);
            consecutive_no_progress = 0;
            
            // Progress timeout check
            if start_time.elapsed().as_secs() > 10 && last_progress_time.elapsed().as_secs() > 10 {
                println!("[LAYOUT] [ADVANCED] PROGRESS TIMEOUT: No progress for 10 seconds, stopping layout");
                        break;
                    }
            
            if local_boxes.len() != last_boxes_count || queue.len() != last_queue_size {
                last_progress_time = Instant::now();
                last_queue_size = queue.len();
                last_boxes_count = local_boxes.len();
            }
            
            if iterations % Self::PROGRESS_INTERVAL == 0 {
                batch_count += 1;
                let elapsed = start_time.elapsed().as_millis();
                println!("[LAYOUT] [ADVANCED] Batch {}: {} iterations, {} nodes, queue: {}, boxes: {} in {}ms", 
                    batch_count, iterations, *node_count, queue.len(), local_boxes.len(), elapsed);
                last_progress_time = Instant::now();
            }
            
            if iterations > 50_000 {
                println!("[LAYOUT] [ADVANCED] WARNING: Excessive iterations ({}), stopping", iterations);
                break;
            }
            
            if node_depth > 200 {
                println!("[LAYOUT] [ADVANCED] Layout depth limit reached ({}), skipping", node_depth);
                continue;
            }
            
            let mut local_current_x = node_x;
            let mut local_current_y = node_y;
            let mut local_max_height: f32 = 0.0;
            
            match &current_node.node_type {
                NodeType::Element(tag_name) => {
                    if self.should_skip_element(tag_name) {
                        if self.is_layout_important(tag_name) {
                            println!("[SKIP] Skipping important element <{}> at depth {}", tag_name, node_depth);
                        }
                        continue;
                    }
                    
                    let styles = self.get_node_styles(current_node);
                    if styles.display == "none" {
                        if self.is_layout_important(tag_name) {
                            println!("[SKIP] Skipping display:none <{}> at depth {}", tag_name, node_depth);
                        }
                        continue;
                    }
                    
                    let margin = parse_box_value(&styles.margin);
                    let padding = parse_box_value(&styles.padding);
                    let border_width = parse_box_value(&styles.border_width);
                    let border_color = styles.border_color.clone();
                    
                    if self.is_layout_important(tag_name) {
                        println!("[LAYOUT] [ADVANCED] Processing important element: <{}> at depth {}", tag_name, node_depth);
                    }
                    
                    let (width, height) = self.calculate_dimensions(&styles, tag_name);
                    let box_layout = LayoutBox {
                        x: local_current_x + margin.left,
                        y: local_current_y + margin.top,
                        width,
                        height,
                        node_type: tag_name.clone(),
                        text_content: self.extract_text_content(current_node, arena),
                        background_color: styles.background_color.clone(),
                        color: styles.color.clone(),
                        font_size: styles.font_size.parse().unwrap_or(16.0),
                        font_family: styles.font_family.clone(),
                        border_color: border_color.clone(),
                        border_width: border_width.clone(),
                        margin: margin.clone(),
                        padding: padding.clone(),
                        font_weight: styles.font_weight.parse().unwrap_or(400.0),
                        text_align: styles.text_align.clone(),
                        flex_direction: styles.flex_direction.clone(),
                        flex_wrap: styles.flex_wrap.clone(),
                        justify_content: styles.justify_content.clone(),
                        align_items: styles.align_items.clone(),
                        flex_grow: styles.flex_grow.parse().unwrap_or(0.0),
                        flex_shrink: styles.flex_shrink.parse().unwrap_or(1.0),
                        flex_basis: styles.flex_basis.clone(),
                        order: styles.order.parse().unwrap_or(0),
                        grid_column: styles.grid_column.clone(),
                        grid_row: styles.grid_row.clone(),
                        line_height: styles.line_height.parse().unwrap_or(1.2),
                        word_wrap: styles.word_wrap.clone(),
                        white_space: styles.white_space.clone(),
                        text_overflow: styles.text_overflow.clone(),
                        color_scheme: styles.color_scheme.clone(),
                    };
                    
                    if self.is_layout_important(tag_name) || !self.extract_text_content(current_node, arena).is_empty() {
                        local_boxes.push(box_layout);
                    }
                    
                    // Advanced child processing with parallel optimization
                    let child_results: Vec<Vec<LayoutBox>> = current_node.children.iter()
                        .filter_map(|child_id| {
                            if let Some(child_node) = arena.get_node(child_id) {
                                let child = child_node.lock().unwrap();
                                if self.should_process_node(&child, node_depth + 1) {
                                    if node_depth + 1 <= 3 {
                                        match &child.node_type {
                                            NodeType::Element(tag) => println!("[ENQUEUE] <{}> at depth {} (parallel child)", tag, node_depth + 1),
                                            NodeType::Text => println!("[ENQUEUE] <text> at depth {} (parallel child)", node_depth + 1),
                                            NodeType::Document => println!("[ENQUEUE] <document> at depth {} (parallel child)", node_depth + 1),
                                        }
                                    }
                                    let mut local_boxes = Vec::new();
                                    let mut local_node_count = 0;
                                    Some(self.layout_node_advanced(&child, local_current_x, local_current_y, &mut local_boxes, node_depth + 1, &mut local_node_count, arena).0)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    for mut child_boxes in child_results {
                        local_boxes.append(&mut child_boxes);
                    }
                    
                    local_current_x += width + margin.left + margin.right + border_width.left + border_width.right + padding.left + padding.right;
                    local_max_height = local_max_height.max(height + margin.top + margin.bottom + border_width.top + border_width.bottom + padding.top + padding.bottom);
                }
                NodeType::Text => {
                    let text = current_node.text_content.trim();
                    if !text.is_empty() && text.len() > 1 {
                        let styles = self.get_node_styles(current_node);
                        let (width, height) = self.calculate_dimensions(&styles, "text");
                        let box_layout = LayoutBox {
                            x: local_current_x,
                            y: local_current_y,
                            width,
                            height,
                            node_type: "text".to_string(),
                            text_content: text.to_string(),
                            background_color: styles.background_color.clone(),
                            color: styles.color.clone(),
                            font_size: styles.font_size.parse().unwrap_or(16.0),
                            font_family: styles.font_family.clone(),
                            border_color: "".to_string(),
                            border_width: BoxValues::default(),
                            margin: BoxValues::default(),
                            padding: BoxValues::default(),
                            font_weight: styles.font_weight.parse().unwrap_or(400.0),
                            text_align: styles.text_align.clone(),
                            flex_direction: "".to_string(),
                            flex_wrap: "".to_string(),
                            justify_content: "".to_string(),
                            align_items: "".to_string(),
                            flex_grow: 0.0,
                            flex_shrink: 1.0,
                            flex_basis: "".to_string(),
                            order: 0,
                            grid_column: "".to_string(),
                            grid_row: "".to_string(),
                            line_height: styles.line_height.parse().unwrap_or(1.2),
                            word_wrap: styles.word_wrap.clone(),
                            white_space: styles.white_space.clone(),
                            text_overflow: styles.text_overflow.clone(),
                            color_scheme: styles.color_scheme.clone(),
                        };
                        local_boxes.push(box_layout);
                        local_current_x += width;
                        local_max_height = local_max_height.max(height);
                    }
                }
                NodeType::Document => {
                    println!("[LAYOUT] [ADVANCED] Document node: processing {} children", current_node.children.len());
                    let child_results: Vec<Vec<LayoutBox>> = current_node.children.iter()
                        .filter_map(|child_id| {
                            if let Some(child_node) = arena.get_node(child_id) {
                                let child = child_node.lock().unwrap();
                                if self.should_process_node(&child, node_depth + 1) {
                                    if node_depth + 1 <= 3 {
                                        match &child.node_type {
                                            NodeType::Element(tag) => println!("[ENQUEUE] <{}> at depth {} (parallel doc child)", tag, node_depth + 1),
                                            NodeType::Text => println!("[ENQUEUE] <text> at depth {} (parallel doc child)", node_depth + 1),
                                            NodeType::Document => println!("[ENQUEUE] <document> at depth {} (parallel doc child)", node_depth + 1),
                                        }
                                    }
                                    let mut local_boxes = Vec::new();
                                    let mut local_node_count = 0;
                                    Some(self.layout_node_advanced(&child, local_current_x, local_current_y, &mut local_boxes, node_depth + 1, &mut local_node_count, arena).0)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    for mut child_boxes in child_results {
                        local_boxes.append(&mut child_boxes);
                    }
                }
            }
            
            current_x = local_current_x;
            current_y = local_current_y;
            max_height = max_height.max(local_max_height);
        }
        
        println!("[LAYOUT] [ADVANCED] Layout completed: {} iterations, {} boxes created, {} nodes processed", 
            iterations, local_boxes.len(), *node_count);
        
        (local_boxes.clone(), (current_x, current_y + max_height))
    }

    fn extract_text_content(&self, node: &DOMNode, arena: &DOMArena) -> String {
        let mut text = String::new();
        match &node.node_type {
            NodeType::Text => {
                text.push_str(&node.text_content);
            }
            NodeType::Element(_) => {
                for child in &node.children {
                    if let Some(child_node) = arena.get_node(child) {
                        let child = child_node.lock().unwrap();
                        match &child.node_type {
                            NodeType::Text => {
                                text.push_str(&child.text_content);
                                text.push(' ');
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        text.trim().to_string()
    }

    fn get_node_styles(&self, node: &DOMNode) -> StyleMap {
        let mut styles = StyleMap::default();
        
        // Apply inline styles
        if let Some(style_attr) = node.attributes.get("style") {
            let inline_styles = parse_inline_styles(style_attr);
            styles.merge(&inline_styles);
        }

        // Apply external stylesheet if available
        if let Some(ref stylesheet) = self.stylesheet {
            self.apply_stylesheet_to_node(node, stylesheet, &mut styles);
        }
        
        styles
    }

    fn apply_stylesheet_to_node(&self, node: &DOMNode, stylesheet: &Stylesheet, styles: &mut StyleMap) {
        if let NodeType::Element(_tag_name) = &node.node_type {
            for rule in &stylesheet.rules {
                if matches_selector(node, &rule.selector) {
                    for (property, value) in &rule.declarations {
                        self.apply_css_property(styles, property, value);
                    }
                }
            }
        }
    }

    fn apply_css_property(&self, styles: &mut StyleMap, property: &str, value: &str) {
        // This is a simplified version - the full implementation is in css_parser.rs
        match property.to_lowercase().as_str() {
            "display" => styles.display = value.to_string(),
            "width" => styles.width = value.to_string(),
            "height" => styles.height = value.to_string(),
            "background-color" => styles.background_color = value.to_string(),
            "color" => styles.color = value.to_string(),
            "font-size" => styles.font_size = value.to_string(),
            "font-family" => styles.font_family = value.to_string(),
            "border-width" => styles.border_width = value.to_string(),
            "border-color" => styles.border_color = value.to_string(),
            "padding" => styles.padding = value.to_string(),
            "margin" => styles.margin = value.to_string(),
            "font-weight" => styles.font_weight = value.to_string(),
            "text-align" => styles.text_align = value.to_string(),
            _ => {}
        }
    }

    fn calculate_dimensions(&self, styles: &StyleMap, tag_name: &str) -> (f32, f32) {
        let width = self.parse_length(&styles.width, if tag_name == "text" { 100.0 } else { 200.0 });
        let height = self.parse_length(&styles.height, if tag_name == "text" { 20.0 } else { 100.0 });
        
        // Apply viewport constraints
        let max_width = self.viewport_width * 0.9;
        let max_height = self.viewport_height * 0.9;
        
        (width.min(max_width), height.min(max_height))
    }

    fn parse_length(&self, value: &str, default: f32) -> f32 {
        if value.is_empty() {
            return default;
        }
        
        if value.ends_with("px") {
            value[..value.len() - 2].parse().unwrap_or(default)
        } else if value.ends_with("%") {
            let percent = value[..value.len() - 1].parse().unwrap_or(0.0);
            if value.contains("width") {
                self.viewport_width * percent / 100.0
            } else {
                self.viewport_height * percent / 100.0
            }
        } else {
            value.parse().unwrap_or(default)
        }
    }
}

fn parse_box_value(value: &str) -> BoxValues {
    let parts: Vec<&str> = value.split_whitespace().collect();
    match parts.len() {
        1 => {
            let val = parts[0].parse().unwrap_or(0.0);
            BoxValues { top: val, right: val, bottom: val, left: val }
        }
        2 => {
            let top_bottom = parts[0].parse().unwrap_or(0.0);
            let left_right = parts[1].parse().unwrap_or(0.0);
            BoxValues { top: top_bottom, right: left_right, bottom: top_bottom, left: left_right }
        }
        4 => {
            BoxValues {
                top: parts[0].parse().unwrap_or(0.0),
                right: parts[1].parse().unwrap_or(0.0),
                bottom: parts[2].parse().unwrap_or(0.0),
                left: parts[3].parse().unwrap_or(0.0),
            }
        }
        _ => BoxValues::default(),
    }
} 
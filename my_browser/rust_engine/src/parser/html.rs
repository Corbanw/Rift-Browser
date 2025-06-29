use crate::dom::node::{DOMNode, NodeType};
use crate::parser::css::{parse_css, Stylesheet};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    OpenTag,
    CloseTag,
    Text,
    SelfClosingTag,
}

// New: Streaming parser state
#[derive(Debug, Clone)]
pub enum ParserState {
    Initial,
    InTag,           // Inside <tag
    InCloseTag,      // Inside </tag
    InComment,       // Inside <!-- comment -->
    InDoctype,       // Inside <!doctype
    InText,          // In text content
    InScript,        // Inside <script>...</script>
    InStyle,         // Inside <style>...</style>
}

// New: Streaming HTML parser that can process chunks
pub struct StreamingHTMLParser {
    buffer: String,                    // Accumulated HTML data
    state: ParserState,                // Current parsing state
    tokens: Vec<Token>,                // Completed tokens
    partial_token: Option<String>,     // Incomplete token being built
    inside_script_or_style: bool,      // Whether we're inside script/style
    script_or_style_tag: String,       // Current script/style tag name
    external_stylesheets: Vec<String>, // External CSS URLs found
    extracted_css: Vec<String>,        // Inline CSS blocks
    parsing_stats: ParsingStats,
}

impl StreamingHTMLParser {
    pub fn new() -> Self {
        println!("[STREAMING] Initializing streaming HTML parser");
        Self {
            buffer: String::new(),
            state: ParserState::Initial,
            tokens: Vec::new(),
            partial_token: None,
            inside_script_or_style: false,
            script_or_style_tag: String::new(),
            external_stylesheets: Vec::new(),
            extracted_css: Vec::new(),
            parsing_stats: ParsingStats::default(),
        }
    }

    /// Process a new chunk of HTML data
    pub fn process_chunk(&mut self, chunk: &str) -> Vec<Token> {
        println!("[STREAMING] Processing chunk of {} characters", chunk.len());
        
        // Add chunk to buffer
        self.buffer.push_str(chunk);
        self.parsing_stats.total_chars += chunk.len();
        
        // Process the buffer and extract complete tokens
        let new_tokens = self.process_buffer();
        
        // Return any new tokens that were completed
        new_tokens
    }

    /// Feed a chunk of bytes to the parser (alias for process_chunk)
    pub fn feed_chunk(&mut self, chunk: &[u8]) {
        if let Ok(chunk_str) = String::from_utf8(chunk.to_vec()) {
            self.process_chunk(&chunk_str);
        }
    }

    /// Process the current buffer and extract complete tokens
    fn process_buffer(&mut self) -> Vec<Token> {
        let mut new_tokens = Vec::new();
        let mut processed_pos = 0;
        
        while processed_pos < self.buffer.len() {
            match self.state {
                ParserState::Initial => {
                    if let Some(lt_pos) = self.buffer[processed_pos..].find('<') {
                        // Found a '<', transition to tag parsing
                        if lt_pos > 0 {
                            // There's text before the tag
                            let text = self.buffer[processed_pos..processed_pos + lt_pos].to_string();
                            if !text.trim().is_empty() {
                                let token = Token {
                                    token_type: TokenType::Text,
                                    value: text,
                                    attributes: HashMap::new(),
                                };
                                new_tokens.push(token);
                                self.parsing_stats.tokens_created += 1;
                            }
                        }
                        processed_pos += lt_pos;
                        self.state = ParserState::InTag;
                    } else {
                        // No '<' found, all remaining is text
                        let text = self.buffer[processed_pos..].to_string();
                        if !text.trim().is_empty() {
                            let token = Token {
                                token_type: TokenType::Text,
                                value: text,
                                attributes: HashMap::new(),
                            };
                            new_tokens.push(token);
                            self.parsing_stats.tokens_created += 1;
                        }
                        break;
                    }
                }
                
                ParserState::InTag => {
                    // Look for the closing '>' of the tag
                    if let Some(gt_pos) = self.buffer[processed_pos..].find('>') {
                        let tag_content = self.buffer[processed_pos..processed_pos + gt_pos + 1].to_string();
                        let token = self.parse_tag(&tag_content);
                        if let Some(token) = token {
                            new_tokens.push(token.clone());
                            self.parsing_stats.tokens_created += 1;
                            
                            // Handle script/style tags
                            if let TokenType::OpenTag = token.token_type {
                                if token.value == "script" || token.value == "style" {
                                    self.inside_script_or_style = true;
                                    self.script_or_style_tag = token.value.clone();
                                    self.state = ParserState::InText; // Will be overridden if needed
                                }
                            }
                        }
                        processed_pos += gt_pos + 1;
                        self.state = ParserState::Initial;
                    } else {
                        // Incomplete tag, keep the partial content
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                        break;
                    }
                }
                
                ParserState::InText => {
                    if self.inside_script_or_style {
                        // Look for closing script/style tag
                        let close_tag = format!("</{}>", self.script_or_style_tag);
                        if let Some(close_pos) = self.buffer[processed_pos..].find(&close_tag) {
                            let text = self.buffer[processed_pos..processed_pos + close_pos].to_string();
                            if !text.trim().is_empty() {
                                let token = Token {
                                    token_type: TokenType::Text,
                                    value: text,
                                    attributes: HashMap::new(),
                                };
                                new_tokens.push(token);
                                self.parsing_stats.tokens_created += 1;
                            }
                            processed_pos += close_pos;
                            self.inside_script_or_style = false;
                            self.script_or_style_tag.clear();
                            self.state = ParserState::InTag; // Process the closing tag
                        } else {
                            // Incomplete script/style content
                            break;
                        }
                    } else {
                        // Regular text, look for next '<'
                        if let Some(lt_pos) = self.buffer[processed_pos..].find('<') {
                            let text = self.buffer[processed_pos..processed_pos + lt_pos].to_string();
                            if !text.trim().is_empty() {
                                let token = Token {
                                    token_type: TokenType::Text,
                                    value: text,
                                    attributes: HashMap::new(),
                                };
                                new_tokens.push(token);
                                self.parsing_stats.tokens_created += 1;
                            }
                            processed_pos += lt_pos;
                            self.state = ParserState::InTag;
                        } else {
                            // No more '<' found, all remaining is text
                            let text = self.buffer[processed_pos..].to_string();
                            if !text.trim().is_empty() {
                                let token = Token {
                                    token_type: TokenType::Text,
                                    value: text,
                                    attributes: HashMap::new(),
                                };
                                new_tokens.push(token);
                                self.parsing_stats.tokens_created += 1;
                            }
                            break;
                        }
                    }
                }
                
                _ => {
                    // Handle other states (comments, doctype, etc.)
                    processed_pos = self.buffer.len();
                    self.state = ParserState::Initial;
                }
            }
        }
        
        // Remove processed content from buffer
        if processed_pos > 0 {
            self.buffer.drain(0..processed_pos);
        }
        
        new_tokens
    }

    /// Parse a complete tag and return a token
    fn parse_tag(&mut self, tag_content: &str) -> Option<Token> {
        let content = tag_content.trim_matches('<').trim_matches('>');
        
        // Handle comments
        if content.starts_with("!--") {
            return None; // Skip comments
        }
        
        // Handle doctype
        if content.to_lowercase().starts_with("!doctype") {
            return None; // Skip doctype
        }
        
        // Handle close tags
        if content.starts_with('/') {
            let tag_name = content[1..].trim().to_lowercase();
            return Some(Token {
                token_type: TokenType::CloseTag,
                value: tag_name,
                attributes: HashMap::new(),
            });
        }
        
        // Handle self-closing tags
        let is_self_closing = content.ends_with('/');
        let content = if is_self_closing { &content[..content.len()-1] } else { content };
        
        // Parse tag name and attributes
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }
        
        let tag_name = parts[0].to_lowercase();
        let attributes = self.parse_attributes(content);
        
        // Handle external stylesheets
        if tag_name == "link" {
            if let Some(rel) = attributes.get("rel") {
                if rel.to_lowercase() == "stylesheet" {
                    if let Some(href) = attributes.get("href") {
                        println!("[STREAMING] Found external stylesheet: {}", href);
                        self.external_stylesheets.push(href.clone());
                    }
                }
            }
        }
        
        Some(Token {
            token_type: if is_self_closing { TokenType::SelfClosingTag } else { TokenType::OpenTag },
            value: tag_name,
            attributes,
        })
    }

    /// Parse attributes from tag content
    fn parse_attributes(&self, tag_content: &str) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        let mut parts = tag_content.split_whitespace();
        
        // Skip the tag name
        if let Some(_) = parts.next() {
            for part in parts {
                if part.contains('=') {
                    let mut key_value = part.splitn(2, '=');
                    if let (Some(key), Some(value)) = (key_value.next(), key_value.next()) {
                        let key = key.trim().to_string();
                        let value = value.trim_matches('"').trim_matches('\'').to_string();
                        attributes.insert(key, value);
                    }
                }
            }
        }
        
        attributes
    }

    /// Get all tokens processed so far
    pub fn get_tokens(&self) -> &[Token] {
        &self.tokens
    }

    /// Get external stylesheets found
    pub fn get_external_stylesheets(&self) -> &[String] {
        &self.external_stylesheets
    }

    /// Get parsing statistics
    pub fn get_stats(&self) -> &ParsingStats {
        &self.parsing_stats
    }

    /// Check if parsing is complete (no partial tokens)
    pub fn is_complete(&self) -> bool {
        self.partial_token.is_none() && self.buffer.is_empty()
    }

    /// Get all extracted CSS blocks
    pub fn get_extracted_css(&self) -> &[String] {
        &self.extracted_css
    }
}

pub struct HTMLParser {
    input: String,
    position: usize,
    pub extracted_css: Vec<String>, // Store extracted CSS for later processing
    pub external_stylesheets: Vec<String>, // Store external CSS hrefs
    pub parsing_stats: ParsingStats,
}

#[derive(Debug, Clone)]
pub struct ParsingStats {
    pub total_chars: usize,
    pub tokens_created: usize,
    pub dom_nodes_created: usize,
    pub css_blocks_extracted: usize,
    pub parsing_time_ms: u64,
    pub memory_usage_mb: f64,
}

impl Default for ParsingStats {
    fn default() -> Self {
        Self {
            total_chars: 0,
            tokens_created: 0,
            dom_nodes_created: 0,
            css_blocks_extracted: 0,
            parsing_time_ms: 0,
            memory_usage_mb: 0.0,
        }
    }
}

impl HTMLParser {
    // Enhanced limits for large documents
    pub const MAX_DOCUMENT_SIZE: usize = 10_000_000; // 10MB max
    pub const MAX_TOKENS: usize = 500_000; // 500K tokens max
    pub const MAX_DOM_NODES: usize = 100_000; // 100K nodes max
    pub const CHUNK_SIZE: usize = 50_000; // Process in 50KB chunks
    pub const PROGRESS_INTERVAL: usize = 10_000; // Log progress every 10K tokens

    pub fn new(input: String) -> Self {
        let total_chars = input.len();
        println!("Rust: HTML Parser initialized for {} characters", total_chars);
        
        Self {
            input,
            position: 0,
            extracted_css: Vec::new(),
            external_stylesheets: Vec::new(),
            parsing_stats: ParsingStats {
                total_chars,
                ..Default::default()
            },
        }
    }

    pub fn parse(&mut self) -> DOMNode {
        let start_time = Instant::now();
        
        if self.input.trim().is_empty() {
            println!("Rust: HTML input is empty or whitespace!");
            return DOMNode::new(NodeType::Document);
        }
        
        // Validate document size
        if self.input.len() > Self::MAX_DOCUMENT_SIZE {
            println!("Rust: WARNING: Document too large ({} chars, max: {}), truncating", 
                self.input.len(), Self::MAX_DOCUMENT_SIZE);
            self.input.truncate(Self::MAX_DOCUMENT_SIZE);
        }
        
        println!("Rust: First 200 chars of input: {:?}", self.input.chars().take(200).collect::<String>());
        println!("Rust: Processing HTML document of {} characters", self.input.len());
        
        let mut root = DOMNode::new(NodeType::Document);
        
        // Stream-based tokenization for large documents
        let tokens = self.tokenize_streaming();
        self.parsing_stats.tokens_created = tokens.len();
        println!("Rust: Tokenized {} tokens in {}ms", tokens.len(), start_time.elapsed().as_millis());
        
        // Build DOM with memory management
        self.build_dom_optimized(&tokens, &mut root);
        self.parsing_stats.dom_nodes_created = self.count_nodes(&root);
        
        let elapsed = start_time.elapsed();
        self.parsing_stats.parsing_time_ms = elapsed.as_millis() as u64;
        
        println!("Rust: DOM built with {} nodes in {}ms", 
            self.parsing_stats.dom_nodes_created, elapsed.as_millis());
        println!("Rust: Extracted {} CSS blocks", self.extracted_css.len());
        println!("Rust: Total parsing time: {}ms", elapsed.as_millis());
        
        root
    }

    /// Get all extracted CSS as a single stylesheet
    pub fn get_stylesheet(&self) -> Stylesheet {
        let mut all_css = String::new();
        for css in &self.extracted_css {
            all_css.push_str(css);
            all_css.push('\n');
        }
        parse_css(&all_css)
    }

    /// Build DOM from a list of tokens (for streaming parser)
    pub fn build_dom_from_tokens(&mut self, tokens: &[Token], root: &mut DOMNode) {
        println!("[DOM] Building DOM from {} tokens", tokens.len());
        self.build_dom_optimized(tokens, root);
        self.parsing_stats.dom_nodes_created = self.count_nodes(root);
        println!("[DOM] Built DOM with {} nodes", self.parsing_stats.dom_nodes_created);
    }

    /// Robust state-machine-based tokenization for real-world HTML
    fn tokenize_streaming(&mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(10000);
        let mut token_count = 0;
        let input = self.input.as_bytes();
        let mut pos = 0;
        let len = input.len();
        let mut inside_script_or_style = false;
        let mut script_or_style_tag = String::new();
        let mut token_type_counts = [0usize; 4]; // Open, Close, Text, SelfClosing

        while pos < len {
            if input[pos] == b'<' {
                // Handle comments
                if pos + 4 <= len && &input[pos..pos+4] == b"<!--" {
                    let mut end = pos + 4;
                    while end + 3 <= len && &input[end..end+3] != b"-->" {
                        end += 1;
                    }
                    if end + 3 > len {
                        println!("[WARN] Unclosed comment detected at position {}", pos);
                    }
                    pos = if end + 3 <= len { end + 3 } else { len };
                    continue;
                }
                // Handle doctype
                if pos + 9 <= len && &input[pos..pos+9].to_ascii_lowercase() == b"<!doctype" {
                    while pos < len && input[pos] != b'>' { pos += 1; }
                    pos += 1;
                    continue;
                }
                // Handle close tag
                if pos + 2 < len && input[pos+1] == b'/' {
                    let start = pos + 2;
                    let mut end = start;
                    while end < len && input[end] != b'>' { end += 1; }
                    let tag = String::from_utf8_lossy(&input[start..end]).trim().to_string();
                    let token = Token {
                        token_type: TokenType::CloseTag,
                        value: tag.clone(),
                        attributes: HashMap::new(),
                    };
                    println!("[TOKEN] CloseTag: <{}> at pos {}", tag, pos);
                    token_type_counts[1] += 1;
                    tokens.push(token);
                    pos = if end < len { end + 1 } else { len };
                    if inside_script_or_style && tag == script_or_style_tag {
                        inside_script_or_style = false;
                        script_or_style_tag.clear();
                    }
                    continue;
                }
                // Handle open tag
                let start = pos + 1;
                let mut end = start;
                let mut is_self_closing = false;
                while end < len && input[end] != b'>' { end += 1; }
                if end > start && input[end-1] == b'/' { is_self_closing = true; }
                let tag_content = String::from_utf8_lossy(&input[start..end]).trim().to_string();
                let tag_name = tag_content.split_whitespace().next().unwrap_or("").to_lowercase();
                let attributes = self.parse_attributes(&tag_content);
                // Special handling for script/style
                if tag_name == "script" || tag_name == "style" {
                    inside_script_or_style = true;
                    script_or_style_tag = tag_name.clone();
                }
                let token = Token {
                    token_type: if is_self_closing { TokenType::SelfClosingTag } else { TokenType::OpenTag },
                    value: tag_name.clone(),
                    attributes,
                };
                if is_self_closing {
                    println!("[TOKEN] SelfClosingTag: <{} /> at pos {}", tag_name, pos);
                    token_type_counts[3] += 1;
                } else {
                    println!("[TOKEN] OpenTag: <{}> at pos {}", tag_name, pos);
                    token_type_counts[0] += 1;
                }
                tokens.push(token);
                pos = if end < len { end + 1 } else { len };
                continue;
            }
            // Handle text or script/style content
            let start = pos;
            if inside_script_or_style {
                // Read until closing tag
                let close_tag = format!("</{}>", script_or_style_tag);
                let mut end = pos;
                while end + close_tag.len() <= len && &self.input[end..end+close_tag.len()] != close_tag {
                    end += 1;
                    if end >= len { break; }
                }
                let text = self.input[start..end].to_string();
                let token = Token {
                    token_type: TokenType::Text,
                    value: text.clone(),
                    attributes: HashMap::new(),
                };
                println!("[TOKEN] Script/Style Text: {} chars at pos {}", text.len(), pos);
                token_type_counts[2] += 1;
                tokens.push(token);
                pos = end;
                continue;
            } else {
                // Read until next '<' using string slice operations
                let remaining_text = &self.input[pos..];
                if let Some(lt_pos) = remaining_text.find('<') {
                    let end = pos + lt_pos;
                    let text = self.input[start..end].to_string();
                    if !text.trim().is_empty() {
                        let token = Token {
                            token_type: TokenType::Text,
                            value: text.clone(),
                            attributes: HashMap::new(),
                        };
                        println!("[TOKEN] Text: '{}...' ({} chars) at pos {}", &text.chars().take(40).collect::<String>(), text.len(), pos);
                        token_type_counts[2] += 1;
                        tokens.push(token);
                    }
                    pos = end;
                } else {
                    // No more '<' found, read to end
                    let text = self.input[start..].to_string();
                    if !text.trim().is_empty() {
                        let token = Token {
                            token_type: TokenType::Text,
                            value: text.clone(),
                            attributes: HashMap::new(),
                        };
                        println!("[TOKEN] Text: '{}...' ({} chars) at pos {}", &text.chars().take(40).collect::<String>(), text.len(), pos);
                        token_type_counts[2] += 1;
                        tokens.push(token);
                    }
                    pos = len;
                }
            }
        }
        println!("[SUMMARY] Tokenization complete: OpenTag={}, CloseTag={}, Text={}, SelfClosingTag={}, Total={}",
            token_type_counts[0], token_type_counts[1], token_type_counts[2], token_type_counts[3], tokens.len());
        tokens
    }

    /// Robust stack-based DOM builder with logging
    fn build_dom_optimized(&mut self, tokens: &[Token], root: &mut DOMNode) {
        let mut stack: Vec<*mut DOMNode> = vec![root as *mut DOMNode];
        let mut open_tags: Vec<String> = vec!["__document__".to_string()];
        let mut max_depth = 0;
        let mut node_count = 0;
        for token in tokens {
            let current_parent = stack.last().unwrap();
            match &token.token_type {
                TokenType::OpenTag => {
                    let mut node = DOMNode::new(NodeType::Element(token.value.clone()));
                    node.attributes = token.attributes.clone();
                    // Detect <link rel="stylesheet" href="...">
                    if node.node_type == NodeType::Element("link".to_string()) {
                        let rel = node.attributes.get("rel").map(|s| s.to_ascii_lowercase()).unwrap_or_default();
                        if rel == "stylesheet" {
                            if let Some(href) = node.attributes.get("href") {
                                println!("[CSS] Found external stylesheet: {}", href);
                                self.external_stylesheets.push(href.clone());
                            }
                        }
                    }
                    unsafe {
                        (*(*current_parent)).children.push(node);
                        let node_ptr = (*(*current_parent)).children.last_mut().unwrap() as *mut DOMNode;
                        stack.push(node_ptr);
                    }
                    open_tags.push(token.value.clone());
                    node_count += 1;
                    max_depth = max_depth.max(stack.len());
                }
                TokenType::CloseTag => {
                    // Pop until matching open tag is found
                    let close_tag = token.value.to_lowercase();
                    let mut found = false;
                    while stack.len() > 1 && !open_tags.is_empty() {
                        let tag = open_tags.last().unwrap().to_lowercase();
                        if tag == close_tag {
                            // If closing a <style> tag, extract its text content for CSS
                            if tag == "style" {
                                println!("[CSS] Attempting to extract CSS from <style> tag");
                                unsafe {
                                    let style_node = *stack.last().unwrap();
                                    for child in &(*style_node).children {
                                        if let NodeType::Text = child.node_type {
                                            if !child.text_content.trim().is_empty() {
                                                println!("[CSS] Extracted CSS from <style>: {} chars", child.text_content.len());
                                                self.extracted_css.push(child.text_content.clone());
                                            }
                                        }
                                    }
                                }
                                println!("[CSS] Extraction complete for <style> tag");
                            }
                            stack.pop();
                            open_tags.pop();
                            found = true;
                            break;
                        } else {
                            stack.pop();
                            open_tags.pop();
                        }
                    }
                    if !found {
                        println!("[WARN] Unmatched close tag: </{}>", close_tag);
                    }
                }
                TokenType::Text => {
                    if !token.value.trim().is_empty() {
                        let mut text_node = DOMNode::new(NodeType::Text);
                        text_node.text_content = token.value.clone();
                        unsafe {
                            (*(*current_parent)).children.push(text_node);
                        }
                        node_count += 1;
                    }
                }
                TokenType::SelfClosingTag => {
                    let mut node = DOMNode::new(NodeType::Element(token.value.clone()));
                    node.attributes = token.attributes.clone();
                    unsafe {
                        (*(*current_parent)).children.push(node);
                    }
                    node_count += 1;
                }
            }
        }
        println!("[SUMMARY] DOM building complete: {} nodes, max depth {}", node_count, max_depth);
        self.print_dom_tree(root, 0);
    }

    /// Print the DOM tree for debugging
    fn print_dom_tree(&self, node: &DOMNode, depth: usize) {
        let indent = "  ".repeat(depth);
        match &node.node_type {
            NodeType::Element(tag) => {
                println!("{}<{}> ({} children)", indent, tag, node.children.len());
                for child in &node.children {
                    self.print_dom_tree(child, depth + 1);
                }
            }
            NodeType::Text => {
                let text = node.text_content.trim();
                if !text.is_empty() {
                    println!("{}Text: '{}...' ({} chars)", indent, &text.chars().take(40).collect::<String>(), text.len());
                }
            }
            NodeType::Document => {
                println!("{}Document ({} children)", indent, node.children.len());
                for child in &node.children {
                    self.print_dom_tree(child, depth + 1);
                }
            }
        }
    }

    fn count_nodes(&self, node: &DOMNode) -> usize {
        let mut count = 1;
        for child in &node.children {
            count += self.count_nodes(child);
        }
        count
    }

    fn parse_attributes(&self, tag_content: &str) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        let mut parts = tag_content.split_whitespace();
        
        if let Some(_tag_name) = parts.next() {
            let remaining = parts.collect::<Vec<&str>>().join(" ");
            
            let mut current_key = String::new();
            let mut current_value = String::new();
            let mut in_quotes = false;
            let mut quote_char = '\0';
            let mut expecting_value = false;
            
            for ch in remaining.chars() {
                match ch {
                    '=' if !in_quotes => {
                        expecting_value = true;
                    }
                    '"' | '\'' if !in_quotes => {
                        in_quotes = true;
                        quote_char = ch;
                    }
                    '"' | '\'' if in_quotes && ch == quote_char => {
                        in_quotes = false;
                        if !current_key.is_empty() {
                            attributes.insert(current_key.clone(), current_value.clone());
                            current_key.clear();
                            current_value.clear();
                        }
                        expecting_value = false;
                    }
                    ' ' if !in_quotes && !expecting_value => {
                        if !current_key.is_empty() {
                            attributes.insert(current_key.clone(), "".to_string());
                            current_key.clear();
                        }
                    }
                    _ => {
                        if expecting_value {
                            current_value.push(ch);
                        } else {
                            current_key.push(ch);
                        }
                    }
                }
            }
            
            if !current_key.is_empty() {
                attributes.insert(current_key, current_value);
            }
        }
        
        attributes
    }

    fn consume_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position..].chars().next().unwrap();
            self.position += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    fn consume_until(&mut self, delimiter: char) -> String {
        let mut result = String::new();
        while self.position < self.input.len() {
            let ch = self.input[self.position..].chars().next().unwrap();
            if ch == delimiter {
                self.position += ch.len_utf8();
                break;
            }
            result.push(ch);
            self.position += ch.len_utf8();
        }
        result
    }

    fn consume_whitespace(&mut self) {
        while self.position < self.input.len() {
            let ch = self.input[self.position..].chars().next().unwrap();
            if ch.is_whitespace() {
                self.position += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    fn peek_next(&self) -> char {
        if self.position + 1 < self.input.len() {
            self.input[self.position + 1..].chars().next().unwrap_or('\0')
        } else {
            '\0'
        }
    }

    fn skip_comment(&mut self) {
        // Skip <!-- ... -->
        self.position += 4; // Skip "<!--"
        while self.position < self.input.len() {
            if self.input[self.position..].starts_with("-->") {
                self.position += 3;
                break;
            }
            self.position += 1;
        }
    }

    fn skip_doctype(&mut self) {
        // Skip <!DOCTYPE ... >
        while self.position < self.input.len() {
            if self.input[self.position..].starts_with('>') {
                self.position += 1;
                break;
            }
            self.position += 1;
        }
    }

    fn skip_declaration(&mut self) {
        // Skip <!something ... >
        while self.position < self.input.len() {
            if self.input[self.position..].starts_with('>') {
                self.position += 1;
                break;
            }
            self.position += 1;
        }
    }

    fn parse_script_or_style_element(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let start_tag = self.consume_until('>');
        let tag_name = start_tag.split_whitespace().next().unwrap_or("").to_lowercase();
        
        // Find the closing tag
        let closing_tag = format!("</{}>", tag_name);
        let content_start = self.position;
        
        while self.position < self.input.len() {
            if self.input[self.position..].starts_with(&closing_tag) {
                break;
            }
            self.position += 1;
        }
        
        let content = &self.input[content_start..self.position];
        
        if tag_name == "style" {
            println!("Rust: Extracted CSS from <style> tag: {} chars", content.len());
            self.extracted_css.push(content.to_string());
            self.parsing_stats.css_blocks_extracted += 1;
        }
        
        // Skip the closing tag
        if self.position < self.input.len() {
            self.position += closing_tag.len();
        }
        
        // Create tokens for the element
        tokens.push(Token {
            token_type: TokenType::OpenTag,
            value: tag_name.clone(),
            attributes: HashMap::new(),
        });
        
        if !content.trim().is_empty() {
            tokens.push(Token {
                token_type: TokenType::Text,
                value: content.to_string(),
                attributes: HashMap::new(),
            });
        }
        
        tokens.push(Token {
            token_type: TokenType::CloseTag,
            value: tag_name,
            attributes: HashMap::new(),
        });
        
        tokens
    }
} 
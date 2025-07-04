use crate::dom::node::{DOMNode, NodeType};
use crate::parser::css::{parse_css, Stylesheet};
use std::collections::HashMap;
use std::time::Instant;
use crate::dom::node::DOMArena;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub attributes: HashMap<String, String>,
    pub position: usize, // Track position for better error reporting
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    OpenTag,
    CloseTag,
    Text,
    SelfClosingTag,
    Comment,
    Doctype,
    ScriptContent,
    StyleContent,
}

// Enhanced parser state for better handling of complex HTML
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
    InCDATA,         // Inside <![CDATA[...]]>
    InProcessingInstruction, // Inside <?...?>
}

// Enhanced streaming HTML parser with better JavaScript and CSS handling
pub struct StreamingHTMLParser {
    buffer: String,
    state: ParserState,
    tokens: Vec<Token>,
    partial_token: Option<String>,
    inside_script_or_style: bool,
    script_or_style_tag: String,
    external_stylesheets: Vec<String>,
    extracted_css: Vec<String>,
    extracted_scripts: Vec<String>, // Store JavaScript for execution
    parsing_stats: ParsingStats,
    current_position: usize,
    script_src_urls: Vec<String>, // External script URLs
    style_href_urls: Vec<String>, // External style URLs
}

impl StreamingHTMLParser {
    pub fn new() -> Self {
        println!("[STREAMING] Initializing enhanced streaming HTML parser");
        Self {
            buffer: String::new(),
            state: ParserState::Initial,
            tokens: Vec::new(),
            partial_token: None,
            inside_script_or_style: false,
            script_or_style_tag: String::new(),
            external_stylesheets: Vec::new(),
            extracted_css: Vec::new(),
            extracted_scripts: Vec::new(),
            parsing_stats: ParsingStats::default(),
            current_position: 0,
            script_src_urls: Vec::new(),
            style_href_urls: Vec::new(),
        }
    }

    /// Process a new chunk of HTML data with enhanced parsing
    pub fn process_chunk(&mut self, chunk: &str) -> Vec<Token> {
        println!("[STREAMING] Processing chunk of {} characters", chunk.len());
        
        self.buffer.push_str(chunk);
        self.parsing_stats.total_chars += chunk.len();
        
        let new_tokens = self.process_buffer_enhanced();
        new_tokens
    }

    /// Enhanced buffer processing with better state management
    fn process_buffer_enhanced(&mut self) -> Vec<Token> {
        let mut new_tokens = Vec::new();
        let mut processed_pos = 0;
        let mut iteration_count = 0;
        let max_iterations = self.buffer.len() * 2; // Safety limit to prevent infinite loops
        
        while processed_pos < self.buffer.len() && iteration_count < max_iterations {
            iteration_count += 1;
            let start_pos = processed_pos;
            let mut made_progress = false;
            
            match self.state {
                ParserState::Initial => {
                    if let Some(lt_pos) = self.buffer[processed_pos..].find('<') {
                        if lt_pos > 0 {
                            let text = self.buffer[processed_pos..processed_pos + lt_pos].to_string();
                            if !text.trim().is_empty() {
                                let token = Token {
                                    token_type: TokenType::Text,
                                    value: text,
                                    attributes: HashMap::new(),
                                    position: self.current_position + processed_pos,
                                };
                                new_tokens.push(token);
                                self.parsing_stats.tokens_created += 1;
                            }
                        }
                        processed_pos += lt_pos;
                        made_progress = true;
                        if processed_pos + 1 < self.buffer.len() {
                            let next_char = self.buffer.chars().nth(processed_pos + 1);
                            match next_char {
                                Some('/') => self.state = ParserState::InCloseTag,
                                Some('!') => {
                                    if self.buffer[processed_pos..].starts_with("<!--") {
                                        self.state = ParserState::InComment;
                                    } else if self.buffer[processed_pos..].to_lowercase().starts_with("<!doctype") {
                                        self.state = ParserState::InDoctype;
                                    } else if self.buffer[processed_pos..].starts_with("<![CDATA[") {
                                        self.state = ParserState::InCDATA;
                                    } else {
                                        self.state = ParserState::InTag;
                                    }
                                }
                                Some('?') => self.state = ParserState::InProcessingInstruction,
                                Some(_) => self.state = ParserState::InTag,
                                None => {
                                    eprintln!("[HTML PARSER] Unexpected end of buffer after '<' at position {}", processed_pos);
                                    self.state = ParserState::InTag;
                                }
                            }
                        } else {
                            self.state = ParserState::InTag;
                        }
                    } else {
                        let text = self.buffer[processed_pos..].to_string();
                        if !text.trim().is_empty() {
                            let token = Token {
                                token_type: TokenType::Text,
                                value: text,
                                attributes: HashMap::new(),
                                position: self.current_position + processed_pos,
                            };
                            new_tokens.push(token);
                            self.parsing_stats.tokens_created += 1;
                        }
                        processed_pos = self.buffer.len();
                        made_progress = true;
                        break;
                    }
                }
                ParserState::InTag => {
                    if let Some(gt_pos) = self.buffer[processed_pos..].find('>') {
                        let tag_content = self.buffer[processed_pos..processed_pos + gt_pos + 1].to_string();
                        let token = self.parse_tag_enhanced(&tag_content);
                        if let Some(token) = token {
                            new_tokens.push(token.clone());
                            self.parsing_stats.tokens_created += 1;
                            if let TokenType::OpenTag = token.token_type {
                                match token.value.as_str() {
                                    "script" => {
                                        self.inside_script_or_style = true;
                                        self.script_or_style_tag = "script".to_string();
                                        if let Some(src) = token.attributes.get("src") {
                                            self.script_src_urls.push(src.clone());
                                        }
                                        self.state = ParserState::InScript;
                                    }
                                    "style" => {
                                        self.inside_script_or_style = true;
                                        self.script_or_style_tag = "style".to_string();
                                        self.state = ParserState::InStyle;
                                    }
                                    "link" => {
                                        if let Some(rel) = token.attributes.get("rel") {
                                            if rel == "stylesheet" {
                                                if let Some(href) = token.attributes.get("href") {
                                                    self.style_href_urls.push(href.clone());
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        processed_pos += gt_pos + 1;
                        self.state = ParserState::Initial;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InCloseTag => {
                    if let Some(gt_pos) = self.buffer[processed_pos..].find('>') {
                        let tag_content = self.buffer[processed_pos..processed_pos + gt_pos + 1].to_string();
                        let token = self.parse_close_tag(&tag_content);
                        if let Some(token) = token {
                            new_tokens.push(token);
                            self.parsing_stats.tokens_created += 1;
                        }
                        processed_pos += gt_pos + 1;
                        self.state = ParserState::Initial;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InScript => {
                    let close_tag = "</script>";
                    if let Some(close_pos) = self.buffer[processed_pos..].find(close_tag) {
                        let script_content = self.buffer[processed_pos..processed_pos + close_pos].to_string();
                        if !script_content.trim().is_empty() {
                            let token = Token {
                                token_type: TokenType::ScriptContent,
                                value: script_content.clone(),
                                attributes: HashMap::new(),
                                position: self.current_position + processed_pos,
                            };
                            new_tokens.push(token.clone());
                            self.extracted_scripts.push(script_content.clone());
                            self.parsing_stats.tokens_created += 1;
                        }
                        processed_pos += close_pos;
                        self.inside_script_or_style = false;
                        self.script_or_style_tag.clear();
                        self.state = ParserState::InTag;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InStyle => {
                    let close_tag = "</style>";
                    if let Some(close_pos) = self.buffer[processed_pos..].find(close_tag) {
                        let style_content = self.buffer[processed_pos..processed_pos + close_pos].to_string();
                        if !style_content.trim().is_empty() {
                            let token = Token {
                                token_type: TokenType::StyleContent,
                                value: style_content.clone(),
                                attributes: HashMap::new(),
                                position: self.current_position + processed_pos,
                            };
                            new_tokens.push(token.clone());
                            self.extracted_css.push(style_content.clone());
                            self.parsing_stats.tokens_created += 1;
                        }
                        processed_pos += close_pos;
                        self.inside_script_or_style = false;
                        self.script_or_style_tag.clear();
                        self.state = ParserState::InTag;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InComment => {
                    let close_tag = "-->";
                    if let Some(close_pos) = self.buffer[processed_pos..].find(close_tag) {
                        let comment_content = self.buffer[processed_pos..processed_pos + close_pos].to_string();
                        let token = Token {
                            token_type: TokenType::Comment,
                            value: comment_content,
                            attributes: HashMap::new(),
                            position: self.current_position + processed_pos,
                        };
                        new_tokens.push(token);
                        self.parsing_stats.tokens_created += 1;
                        processed_pos += close_pos + close_tag.len();
                        self.state = ParserState::Initial;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InDoctype => {
                    let close_tag = ">";
                    if let Some(close_pos) = self.buffer[processed_pos..].find(close_tag) {
                        let doctype_content = self.buffer[processed_pos..processed_pos + close_pos + 1].to_string();
                        let token = Token {
                            token_type: TokenType::Doctype,
                            value: doctype_content,
                            attributes: HashMap::new(),
                            position: self.current_position + processed_pos,
                        };
                        new_tokens.push(token);
                        self.parsing_stats.tokens_created += 1;
                        processed_pos += close_pos + 1;
                        self.state = ParserState::Initial;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InCDATA => {
                    let close_tag = "]]>";
                    if let Some(close_pos) = self.buffer[processed_pos..].find(close_tag) {
                        let cdata_content = self.buffer[processed_pos..processed_pos + close_pos].to_string();
                        let token = Token {
                            token_type: TokenType::Text,
                            value: cdata_content,
                            attributes: HashMap::new(),
                            position: self.current_position + processed_pos,
                        };
                        new_tokens.push(token);
                        self.parsing_stats.tokens_created += 1;
                        processed_pos += close_pos + close_tag.len();
                        self.state = ParserState::Initial;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InProcessingInstruction => {
                    let close_tag = "?>";
                    if let Some(close_pos) = self.buffer[processed_pos..].find(close_tag) {
                        let pi_content = self.buffer[processed_pos..processed_pos + close_pos + close_tag.len()].to_string();
                        let token = Token {
                            token_type: TokenType::Comment,
                            value: pi_content,
                            attributes: HashMap::new(),
                            position: self.current_position + processed_pos,
                        };
                        new_tokens.push(token);
                        self.parsing_stats.tokens_created += 1;
                        processed_pos += close_pos + close_tag.len();
                        self.state = ParserState::Initial;
                        made_progress = true;
                    } else {
                        self.partial_token = Some(self.buffer[processed_pos..].to_string());
                    }
                }
                ParserState::InText => {
                    if let Some(lt_pos) = self.buffer[processed_pos..].find('<') {
                        let text = self.buffer[processed_pos..processed_pos + lt_pos].to_string();
                        if !text.trim().is_empty() {
                            let token = Token {
                                token_type: TokenType::Text,
                                value: text,
                                attributes: HashMap::new(),
                                position: self.current_position + processed_pos,
                            };
                            new_tokens.push(token);
                            self.parsing_stats.tokens_created += 1;
                        }
                        processed_pos += lt_pos;
                        self.state = ParserState::InTag;
                        made_progress = true;
                    } else {
                        let text = self.buffer[processed_pos..].to_string();
                        if !text.trim().is_empty() {
                            let token = Token {
                                token_type: TokenType::Text,
                                value: text,
                                attributes: HashMap::new(),
                                position: self.current_position + processed_pos,
                            };
                            new_tokens.push(token);
                            self.parsing_stats.tokens_created += 1;
                        }
                        processed_pos = self.buffer.len();
                        made_progress = true;
                        break;
                    }
                }
            }
            // Fallback: if no progress was made, treat next char as text and advance
            if !made_progress {
                if processed_pos < self.buffer.len() {
                    let fallback_char = self.buffer[processed_pos..].chars().next().unwrap();
                    let token = Token {
                        token_type: TokenType::Text,
                        value: fallback_char.to_string(),
                        attributes: HashMap::new(),
                        position: self.current_position + processed_pos,
                    };
                    new_tokens.push(token);
                    self.parsing_stats.tokens_created += 1;
                    processed_pos += fallback_char.len_utf8();
                    self.state = ParserState::Initial;
                }
            }
            // Safety check: ensure we're making progress
            if processed_pos == start_pos {
                eprintln!("[HTML PARSER] Warning: No progress made at position {}, advancing by 1", processed_pos);
                processed_pos += 1;
                self.state = ParserState::Initial;
            }
        }
        if iteration_count >= max_iterations {
            eprintln!("[HTML PARSER] Warning: Maximum iterations reached ({}) at position {}", max_iterations, processed_pos);
            self.buffer.clear();
            self.state = ParserState::Initial;
        }
        self.current_position += processed_pos;
        if processed_pos > 0 {
            self.buffer = self.buffer[processed_pos..].to_string();
        }
        new_tokens
    }

    /// Enhanced tag parsing with better attribute handling
    fn parse_tag_enhanced(&mut self, tag_content: &str) -> Option<Token> {
        let trimmed = tag_content.trim();
        if trimmed.is_empty() || !trimmed.starts_with('<') || !trimmed.ends_with('>') {
            return None;
        }
        
        let content = &trimmed[1..trimmed.len()-1]; // Remove < >
        let mut parts = content.splitn(2, ' ');
        let tag_name = parts.next()?.to_lowercase();
        
        let attributes = if let Some(attr_part) = parts.next() {
            self.parse_attributes_enhanced(attr_part)
        } else {
            HashMap::new()
        };
        
        let token_type = if content.ends_with('/') {
            TokenType::SelfClosingTag
        } else {
            TokenType::OpenTag
        };
        
        Some(Token {
            token_type,
            value: tag_name,
            attributes,
            position: self.current_position,
        })
    }

    /// Parse closing tags
    fn parse_close_tag(&mut self, tag_content: &str) -> Option<Token> {
        let trimmed = tag_content.trim();
        if !trimmed.starts_with("</") || !trimmed.ends_with('>') {
            return None;
        }
        
        let tag_name = trimmed[2..trimmed.len()-1].to_lowercase();
        
        Some(Token {
            token_type: TokenType::CloseTag,
            value: tag_name,
            attributes: HashMap::new(),
            position: self.current_position,
        })
    }

    /// Enhanced attribute parsing with better quote handling
    fn parse_attributes_enhanced(&self, attr_string: &str) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        let mut current_attr = String::new();
        let mut current_value = String::new();
        let mut in_quotes = false;
        let mut quote_char = '\0';
        let mut parsing_value = false;
        
        for ch in attr_string.chars() {
            match ch {
                '"' | '\'' => {
                    if !in_quotes {
                        in_quotes = true;
                        quote_char = ch;
                        parsing_value = true;
                    } else if ch == quote_char {
                        in_quotes = false;
                        // Store the attribute
                        if !current_attr.is_empty() {
                            attributes.insert(current_attr.trim().to_lowercase(), current_value.trim().to_string());
                            current_attr.clear();
                            current_value.clear();
                        }
                        parsing_value = false;
                    } else {
                        if parsing_value {
                            current_value.push(ch);
                        }
                    }
                }
                '=' if !in_quotes => {
                    parsing_value = true;
                }
                ' ' | '\t' | '\n' | '\r' => {
                    if !in_quotes {
                        if !current_attr.is_empty() && !current_value.is_empty() {
                            attributes.insert(current_attr.trim().to_lowercase(), current_value.trim().to_string());
                            current_attr.clear();
                            current_value.clear();
                        }
                        parsing_value = false;
                    } else if parsing_value {
                        current_value.push(ch);
                    }
                }
                _ => {
                    if parsing_value {
                        current_value.push(ch);
                    } else {
                        current_attr.push(ch);
                    }
                }
            }
        }
        
        // Handle last attribute
        if !current_attr.is_empty() {
            attributes.insert(current_attr.trim().to_lowercase(), current_value.trim().to_string());
        }
        
        attributes
    }

    // Getters for extracted content
    pub fn get_extracted_scripts(&self) -> &[String] {
        &self.extracted_scripts
    }
    
    pub fn get_script_src_urls(&self) -> &[String] {
        &self.script_src_urls
    }
    
    pub fn get_style_href_urls(&self) -> &[String] {
        &self.style_href_urls
    }

    /// Feed a chunk of bytes to the parser (alias for process_chunk)
    pub fn feed_chunk(&mut self, chunk: &[u8]) {
        if let Ok(chunk_str) = String::from_utf8(chunk.to_vec()) {
            self.process_chunk(&chunk_str);
        }
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
        println!("Rust: HTML Parser initialized for {} characters", self.input.len());
        
        if self.input.len() > Self::MAX_DOCUMENT_SIZE {
            eprintln!("[ERROR] Document too large: {} chars (max: {})", 
                self.input.len(), Self::MAX_DOCUMENT_SIZE);
            return DOMNode::new(NodeType::Document);
        }
        
        // Show first 200 chars for debugging
        let preview = self.input.chars().take(200).collect::<String>();
        println!("Rust: First 200 chars of input: \"{}\"", preview);
        
        println!("Rust: Processing HTML document of {} characters", self.input.len());
        
        // Use enhanced tokenization
        let tokens = self.tokenize_streaming();
        
        if tokens.len() > Self::MAX_TOKENS {
            eprintln!("[ERROR] Too many tokens: {} (max: {})", tokens.len(), Self::MAX_TOKENS);
            return DOMNode::new(NodeType::Document);
        }
        
        // Build DOM with enhanced parsing using a shared arena
        let mut arena = DOMArena::new();
        let mut root = DOMNode::new(NodeType::Document);
        let root_id = root.id.clone();
        arena.add_node(root);
        self.build_dom_enhanced(&tokens, &mut arena.get_node(&root_id).unwrap().lock().unwrap(), &mut arena);
        
        // Extract CSS from style tags and inline styles
        self.extract_css_enhanced(&tokens);
        
        self.parsing_stats.parsing_time_ms = start_time.elapsed().as_millis() as u64;
        let root_node = arena.get_node(&root_id).unwrap().lock().unwrap().clone();
        self.parsing_stats.dom_nodes_created = self.count_nodes(&root_node, &arena);
        
        println!("Rust: DOM built with {} nodes in {}ms", 
            self.parsing_stats.dom_nodes_created, self.parsing_stats.parsing_time_ms);
        println!("Rust: Extracted {} CSS blocks", self.extracted_css.len());
        println!("Rust: Total parsing time: {}ms", self.parsing_stats.parsing_time_ms);
        
        root_node
    }

    /// Tokenize using the streaming parser for compatibility
    pub fn tokenize_streaming(&mut self) -> Vec<Token> {
        let mut streaming = StreamingHTMLParser::new();
        streaming.process_chunk(&self.input)
    }

    /// Build DOM using the enhanced builder for compatibility
    pub fn build_dom_enhanced(&mut self, tokens: &[Token], root: &mut DOMNode, arena: &mut DOMArena) {
        let mut stack: Vec<String> = vec![root.id.clone()];
        
        for token in tokens {
            match token.token_type {
                TokenType::OpenTag => {
                    let mut node = DOMNode::new(NodeType::Element(token.value.clone()));
                    
                    // Copy attributes
                    for (key, value) in &token.attributes {
                        node.attributes.insert(key.clone(), value.clone());
                    }
                    
                    let node_id = node.id.clone();
                    arena.add_node(node);
                    
                    // Add to parent
                    if let Some(parent_id) = stack.last() {
                        if let Some(parent) = arena.get_node(parent_id) {
                            let mut parent = parent.lock().unwrap();
                            parent.children.push(node_id.clone());
                        }
                    }
                    
                    // Push to stack if not self-closing
                    if !self.is_self_closing_tag(&token.value) {
                        stack.push(node_id);
                    }
                }
                TokenType::CloseTag => {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                }
                TokenType::Text => {
                    if !token.value.trim().is_empty() {
                        let mut text_node = DOMNode::new(NodeType::Text);
                        text_node.text_content = token.value.clone();
                        
                        let text_node_id = text_node.id.clone();
                        arena.add_node(text_node);
                        
                        // Add to parent
                        if let Some(parent_id) = stack.last() {
                            if let Some(parent) = arena.get_node(parent_id) {
                                let mut parent = parent.lock().unwrap();
                                parent.children.push(text_node_id);
                            }
                        }
                    }
                }
                TokenType::ScriptContent | TokenType::StyleContent => {
                    // Create content node
                    let mut content_node = DOMNode::new(NodeType::Element(
                        if token.token_type == TokenType::ScriptContent { "script".to_string() } else { "style".to_string() }
                    ));
                    content_node.text_content = token.value.clone();
                    
                    let content_node_id = content_node.id.clone();
                    arena.add_node(content_node);
                    
                    // Add to parent
                    if let Some(parent_id) = stack.last() {
                        if let Some(parent) = arena.get_node(parent_id) {
                            let mut parent = parent.lock().unwrap();
                            parent.children.push(content_node_id);
                        }
                    }
                }
                _ => {}
            }
        }
        
        println!("[SUMMARY] DOM building complete: {} nodes", self.count_nodes(root, arena));
    }

    /// Check if tag is self-closing
    fn is_self_closing_tag(&self, tag_name: &str) -> bool {
        matches!(tag_name, "img" | "br" | "hr" | "input" | "meta" | "link" | "area" | "base" | "col" | "embed" | "source" | "track" | "wbr")
    }

    /// Calculate maximum depth of DOM tree
    fn calculate_max_depth(&self, node: &DOMNode, arena: &DOMArena) -> usize {
        let mut max_depth = 0;
        self.calculate_depth_recursive(node, 0, &mut max_depth, arena);
        max_depth
    }

    fn calculate_depth_recursive(&self, node: &DOMNode, current_depth: usize, max_depth: &mut usize, arena: &DOMArena) {
        *max_depth = (*max_depth).max(current_depth);
        for child_id in &node.children {
            if let Some(child_node) = arena.get_node(child_id) {
                let child = child_node.lock().unwrap();
                self.calculate_depth_recursive(&child, current_depth + 1, max_depth, arena);
            }
        }
    }

    fn count_nodes(&self, node: &DOMNode, arena: &DOMArena) -> usize {
        let mut count = 1;
        for child_id in &node.children {
            if let Some(child_node) = arena.get_node(child_id) {
                let child = child_node.lock().unwrap();
                count += self.count_nodes(&child, arena);
            }
        }
        count
    }

    /// Return a dummy stylesheet for compatibility
    pub fn get_stylesheet(&self) -> crate::parser::css::Stylesheet {
        crate::parser::css::Stylesheet::new()
    }

    pub fn get_extracted_scripts(&self) -> &[String] {
        // For now, return empty slice
        // TODO: Implement script extraction
        &[]
    }

    pub fn get_script_src_urls(&self) -> &[String] {
        // For now, return empty slice
        // TODO: Implement script URL extraction
        &[]
    }

    /// Stub for build_dom_from_tokens for compatibility
    pub fn build_dom_from_tokens(&mut self, tokens: &[Token], root: &mut DOMNode) {
        self.build_dom_enhanced(tokens, root, &mut DOMArena::new());
    }

    /// Enhanced CSS extraction
    fn extract_css_enhanced(&mut self, tokens: &[Token]) {
        for token in tokens {
            match token.token_type {
                TokenType::StyleContent => {
                    println!("[CSS] Extracted CSS from <style> tag: {} chars", token.value.len());
                    self.extracted_css.push(token.value.clone());
                    self.parsing_stats.css_blocks_extracted += 1;
                }
                TokenType::OpenTag => {
                    if token.value == "style" {
                        // Inline style tag - content will be in next token
                        println!("[CSS] Found <style> tag");
                    }
                }
                _ => {}
            }
        }
        
        println!("[CSS] Extraction complete for {} style tags", self.parsing_stats.css_blocks_extracted);
    }
} 
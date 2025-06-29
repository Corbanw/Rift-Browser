use crate::dom::node::StyleMap;
use std::collections::HashMap;
use std::time::Instant;

pub struct CSSParser {
    input: String,
    position: usize,
    pub parsing_stats: CSSParsingStats,
}

#[derive(Debug, Clone)]
pub struct CSSParsingStats {
    pub total_chars: usize,
    pub rules_parsed: usize,
    pub selectors_parsed: usize,
    pub declarations_parsed: usize,
    pub parsing_time_ms: u64,
    pub memory_usage_mb: f64,
}

impl Default for CSSParsingStats {
    fn default() -> Self {
        Self {
            total_chars: 0,
            rules_parsed: 0,
            selectors_parsed: 0,
            declarations_parsed: 0,
            parsing_time_ms: 0,
            memory_usage_mb: 0.0,
        }
    }
}

impl CSSParser {
    // Enhanced limits for large stylesheets
    pub const MAX_CSS_SIZE: usize = 5_000_000; // 5MB max
    pub const MAX_RULES: usize = 50_000; // 50K rules max
    pub const MAX_SELECTORS_PER_RULE: usize = 100; // 100 selectors per rule max
    pub const MAX_DECLARATIONS_PER_RULE: usize = 200; // 200 declarations per rule max
    pub const PROGRESS_INTERVAL: usize = 1_000; // Log progress every 1K rules

    pub fn new(input: String) -> Self {
        let total_chars = input.len();
        println!("Rust: CSS Parser initialized for {} characters", total_chars);
        
        Self {
            input,
            position: 0,
            parsing_stats: CSSParsingStats {
                total_chars,
                ..Default::default()
            },
        }
    }

    pub fn parse_inline_styles(&mut self) -> StyleMap {
        let start_time = Instant::now();
        let mut styles = StyleMap::default();
        
        while self.position < self.input.len() {
            self.consume_whitespace();
            
            if self.position >= self.input.len() {
                break;
            }
            
            let property = self.parse_property_name();
            self.consume_whitespace();
            
            if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == ':' {
                self.consume_char(); // consume ':'
                self.consume_whitespace();
                
                let value = self.parse_property_value();
                
                // Apply the style to our StyleMap
                self.apply_style(&mut styles, &property, &value);
                self.parsing_stats.declarations_parsed += 1;
                
                self.consume_whitespace();
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == ';' {
                    self.consume_char(); // consume ';'
                }
            }
        }
        
        self.parsing_stats.parsing_time_ms = start_time.elapsed().as_millis() as u64;
        println!("Rust: Inline CSS parsed: {} declarations in {}ms", 
            self.parsing_stats.declarations_parsed, self.parsing_stats.parsing_time_ms);
        
        styles
    }

    fn parse_property_name(&mut self) -> String {
        let mut property = String::new();
        
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_char.is_alphanumeric() || current_char == '-' || current_char == '_' {
                property.push(current_char);
                self.position += 1;
            } else {
                break;
            }
        }
        
        property
    }

    fn parse_property_value(&mut self) -> String {
        let mut value = String::new();
        let mut in_quotes = false;
        let mut quote_char = '\0';
        let mut paren_depth = 0;
        
        if self.position < self.input.len() && 
            (self.input.chars().nth(self.position).unwrap() == '"' || 
             self.input.chars().nth(self.position).unwrap() == '\'') {
            quote_char = self.input.chars().nth(self.position).unwrap();
            self.consume_char();
            in_quotes = true;
        }
        
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            
            if in_quotes {
                if current_char == quote_char {
                    self.consume_char();
                    break;
                }
            } else {
                match current_char {
                    '(' => paren_depth += 1,
                    ')' => {
                        if paren_depth > 0 {
                            paren_depth -= 1;
                        } else {
                            break;
                        }
                    }
                    ';' if paren_depth == 0 => break,
                    '}' if paren_depth == 0 => break,
                    _ => {}
                }
            }
            
            value.push(current_char);
            self.position += 1;
        }
        
        value.trim().to_string()
    }

    fn apply_style(&self, styles: &mut StyleMap, property: &str, value: &str) {
        match property.to_lowercase().as_str() {
            // Layout properties
            "display" => styles.display = value.to_string(),
            "position" => styles.position = value.to_string(),
            "top" => styles.top = value.to_string(),
            "right" => styles.right = value.to_string(),
            "bottom" => styles.bottom = value.to_string(),
            "left" => styles.left = value.to_string(),
            "z-index" | "zindex" => styles.z_index = value.to_string(),
            
            // Sizing properties
            "width" => styles.width = value.to_string(),
            "height" => styles.height = value.to_string(),
            "min-width" | "minwidth" => styles.min_width = value.to_string(),
            "max-width" | "maxwidth" => styles.max_width = value.to_string(),
            "min-height" | "minheight" => styles.min_height = value.to_string(),
            "max-height" | "maxheight" => styles.max_height = value.to_string(),
            
            // Visual properties
            "background-color" | "backgroundcolor" => styles.background_color = value.to_string(),
            "background" => styles.background = value.to_string(),
            "color" => styles.color = value.to_string(),
            "opacity" => styles.opacity = value.to_string(),
            "visibility" => styles.visibility = value.to_string(),
            
            // Typography properties
            "font-size" | "fontsize" => styles.font_size = value.to_string(),
            "font-family" | "fontfamily" => styles.font_family = value.to_string(),
            "font-weight" | "fontweight" => styles.font_weight = value.to_string(),
            "font-style" | "fontstyle" => styles.font_style = value.to_string(),
            "text-align" | "textalign" => styles.text_align = value.to_string(),
            "text-decoration" | "textdecoration" => styles.text_decoration = value.to_string(),
            "line-height" | "lineheight" => styles.line_height = value.to_string(),
            "letter-spacing" | "letterspacing" => styles.letter_spacing = value.to_string(),
            "word-spacing" | "wordspacing" => styles.word_spacing = value.to_string(),
            
            // Border properties
            "border-width" | "borderwidth" => styles.border_width = value.to_string(),
            "border-color" | "bordercolor" => styles.border_color = value.to_string(),
            "border-style" | "borderstyle" => styles.border_style = value.to_string(),
            "border" => styles.border = value.to_string(),
            "border-radius" | "borderradius" => styles.border_radius = value.to_string(),
            
            // Spacing properties
            "padding" => styles.padding = value.to_string(),
            "padding-top" | "paddingtop" => styles.padding_top = value.to_string(),
            "padding-right" | "paddingright" => styles.padding_right = value.to_string(),
            "padding-bottom" | "paddingbottom" => styles.padding_bottom = value.to_string(),
            "padding-left" | "paddingleft" => styles.padding_left = value.to_string(),
            "margin" => styles.margin = value.to_string(),
            "margin-top" | "margintop" => styles.margin_top = value.to_string(),
            "margin-right" | "marginright" => styles.margin_right = value.to_string(),
            "margin-bottom" | "marginbottom" => styles.margin_bottom = value.to_string(),
            "margin-left" | "marginleft" => styles.margin_left = value.to_string(),
            
            // Flexbox properties
            "flex-direction" | "flexdirection" => styles.flex_direction = value.to_string(),
            "flex-wrap" | "flexwrap" => styles.flex_wrap = value.to_string(),
            "justify-content" | "justifycontent" => styles.justify_content = value.to_string(),
            "align-items" | "alignitems" => styles.align_items = value.to_string(),
            "align-content" | "aligncontent" => styles.align_content = value.to_string(),
            "flex-grow" | "flexgrow" => styles.flex_grow = value.to_string(),
            "flex-shrink" | "flexshrink" => styles.flex_shrink = value.to_string(),
            "flex-basis" | "flexbasis" => styles.flex_basis = value.to_string(),
            "order" => styles.order = value.to_string(),
            
            // Grid properties
            "grid-template-columns" | "gridtemplatecolumns" => styles.grid_template_columns = value.to_string(),
            "grid-template-rows" | "gridtemplaterows" => styles.grid_template_rows = value.to_string(),
            "grid-gap" | "gridgap" => styles.grid_gap = value.to_string(),
            "grid-column" | "gridcolumn" => styles.grid_column = value.to_string(),
            "grid-row" | "gridrow" => styles.grid_row = value.to_string(),
            "grid-area" | "gridarea" => styles.grid_area = value.to_string(),
            
            // Text rendering
            "word-wrap" | "wordwrap" => styles.word_wrap = value.to_string(),
            "white-space" | "whitespace" => styles.white_space = value.to_string(),
            "text-overflow" | "textoverflow" => styles.text_overflow = value.to_string(),
            "overflow" => styles.overflow = value.to_string(),
            "overflow-x" | "overflowx" => styles.overflow_x = value.to_string(),
            "overflow-y" | "overflowy" => styles.overflow_y = value.to_string(),
            
            // Transform properties
            "transform" => styles.transform = value.to_string(),
            "transform-origin" | "transformorigin" => styles.transform_origin = value.to_string(),
            
            // Theme support
            "color-scheme" | "colorscheme" => styles.color_scheme = value.to_string(),
            
            // Box model
            "box-sizing" | "boxsizing" => styles.box_sizing = value.to_string(),
            
            // Cursor
            "cursor" => styles.cursor = value.to_string(),
            
            // Pointer events
            "pointer-events" | "pointerevents" => styles.pointer_events = value.to_string(),
            
            // User select
            "user-select" | "userselect" => styles.user_select = value.to_string(),
            
            _ => {} // Ignore unknown properties
        }
    }

    fn consume_char(&mut self) -> char {
        let current_char = self.input.chars().nth(self.position).unwrap();
        self.position += 1;
        current_char
    }

    fn consume_whitespace(&mut self) {
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_char.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }
}

pub fn parse_inline_styles(style_attr: &str) -> StyleMap {
    let mut parser = CSSParser::new(style_attr.to_string());
    parser.parse_inline_styles()
}

#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: String,
    pub declarations: HashMap<String, String>,
    pub specificity: u32, // CSS specificity for rule ordering
}

#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<CssRule>,
    pub parsing_stats: CSSParsingStats,
}

impl Stylesheet {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            parsing_stats: CSSParsingStats::default(),
        }
    }
    
    pub fn add_rule(&mut self, selector: String, declarations: HashMap<String, String>) {
        let specificity = Self::calculate_specificity(&selector);
        self.rules.push(CssRule {
            selector,
            declarations,
            specificity,
        });
    }
    
    fn calculate_specificity(selector: &str) -> u32 {
        let mut specificity = 0;
        
        // Count ID selectors
        specificity += selector.matches('#').count() as u32 * 100;
        
        // Count class selectors, attribute selectors, and pseudo-classes
        specificity += selector.matches('.').count() as u32 * 10;
        specificity += selector.matches('[').count() as u32 * 10;
        specificity += selector.matches(':').count() as u32 * 10;
        
        // Count element selectors and pseudo-elements
        specificity += selector.matches(|c: char| c.is_alphabetic()).count() as u32;
        specificity += selector.matches("::").count() as u32;
        
        specificity
    }
}

fn remove_css_comments(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next(); // skip '*'
            // Skip until '*/'
            while let Some(c2) = chars.next() {
                if c2 == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn parse_css(css: &str) -> Stylesheet {
    let start_time = Instant::now();
    let mut stylesheet = Stylesheet::new();
    
    // Validate CSS size
    if css.len() > CSSParser::MAX_CSS_SIZE {
        println!("Rust: WARNING: CSS too large ({} chars, max: {}), truncating", 
            css.len(), CSSParser::MAX_CSS_SIZE);
        let truncated_css = &css[..CSSParser::MAX_CSS_SIZE];
        stylesheet.parsing_stats.total_chars = truncated_css.len();
        return parse_css_internal(truncated_css, &mut stylesheet, start_time);
    }
    
    stylesheet.parsing_stats.total_chars = css.len();
    parse_css_internal(css, &mut stylesheet, start_time)
}

fn parse_css_internal(css: &str, stylesheet: &mut Stylesheet, start_time: Instant) -> Stylesheet {
    let css = remove_css_comments(css);
    let mut input = css.as_str();
    let mut rule_count = 0;
    
    println!("Rust: Starting CSS parsing of {} characters", css.len());
    
    while let Some(start) = input.find('{') {
        if rule_count >= CSSParser::MAX_RULES {
            println!("Rust: CSS rule limit reached ({}), stopping parsing", rule_count);
            break;
        }
        
        if let Some(end) = input[start..].find('}') {
            let selector = input[..start].trim().to_string();
            let declarations_text = input[start + 1..start + end].trim();
            
            // Parse declarations
            let mut declarations = HashMap::new();
            let mut decl_count = 0;
            
            for line in declarations_text.split(';') {
                if decl_count >= CSSParser::MAX_DECLARATIONS_PER_RULE {
                    println!("Rust: Declaration limit reached for rule {}, stopping", rule_count);
                    break;
                }
                
                let line = line.trim();
                if !line.is_empty() {
                    if let Some(colon_pos) = line.find(':') {
                        let property = line[..colon_pos].trim().to_string();
                        let value = line[colon_pos + 1..].trim().to_string();
                        declarations.insert(property, value);
                        decl_count += 1;
                        stylesheet.parsing_stats.declarations_parsed += 1;
                    }
                }
            }
            
            stylesheet.add_rule(selector, declarations);
            rule_count += 1;
            stylesheet.parsing_stats.rules_parsed += 1;
            stylesheet.parsing_stats.selectors_parsed += 1;
            
            // Progress logging for large stylesheets
            if css.len() > 100_000 && rule_count % CSSParser::PROGRESS_INTERVAL == 0 {
                println!("Rust: CSS parsing progress: {} rules parsed", rule_count);
            }
            
            input = &input[start + end + 1..];
        } else {
            break;
        }
    }
    
    stylesheet.parsing_stats.parsing_time_ms = start_time.elapsed().as_millis() as u64;
    println!("Rust: CSS parsing completed: {} rules, {} declarations in {}ms", 
        stylesheet.parsing_stats.rules_parsed, 
        stylesheet.parsing_stats.declarations_parsed,
        stylesheet.parsing_stats.parsing_time_ms);
    
    stylesheet.clone()
} 
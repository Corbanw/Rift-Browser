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

    /// Enhanced CSS parsing with better selector and declaration handling
    pub fn parse_enhanced(&mut self) -> Stylesheet {
        let start_time = Instant::now();
        let mut stylesheet = Stylesheet::new();
        
        // Remove comments first
        let cleaned_css = self.remove_comments_enhanced(&self.input);
        
        let mut current_pos = 0;
        while current_pos < cleaned_css.len() {
            // Skip whitespace
            while current_pos < cleaned_css.len() && cleaned_css.chars().nth(current_pos).unwrap().is_whitespace() {
                current_pos += 1;
            }
            
            if current_pos >= cleaned_css.len() {
                break;
            }
            
            // Parse rule
            if let Some((selectors, declarations, new_pos)) = self.parse_rule_enhanced(&cleaned_css, current_pos) {
                for selector in selectors {
                    stylesheet.add_rule(selector, declarations.clone());
                    self.parsing_stats.selectors_parsed += 1;
                }
                self.parsing_stats.rules_parsed += 1;
                current_pos = new_pos;
            } else {
                current_pos += 1;
            }
        }
        
        self.parsing_stats.parsing_time_ms = start_time.elapsed().as_millis() as u64;
        println!("Rust: CSS parsing completed: {} rules, {} declarations in {}ms", 
            self.parsing_stats.rules_parsed, self.parsing_stats.declarations_parsed, self.parsing_stats.parsing_time_ms);
        
        stylesheet
    }

    /// Enhanced comment removal
    fn remove_comments_enhanced(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '/' && chars.peek() == Some(&'*') {
                chars.next(); // consume '*'
                // Skip until */
                while let Some(ch) = chars.next() {
                    if ch == '*' && chars.peek() == Some(&'/') {
                        chars.next(); // consume '/'
                        break;
                    }
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    /// Enhanced rule parsing
    fn parse_rule_enhanced(&mut self, css: &str, start_pos: usize) -> Option<(Vec<String>, HashMap<String, String>, usize)> {
        let mut pos = start_pos;
        
        // Parse selectors
        let selectors = self.parse_selectors_enhanced(css, pos)?;
        pos = selectors.1;
        
        // Skip whitespace and find opening brace
        while pos < css.len() && css.chars().nth(pos).unwrap().is_whitespace() {
            pos += 1;
        }
        
        if pos >= css.len() || css.chars().nth(pos).unwrap() != '{' {
            return None;
        }
        pos += 1; // consume '{'
        
        // Parse declarations
        let declarations = self.parse_declarations_enhanced(css, pos)?;
        pos = declarations.1;
        
        // Skip whitespace and find closing brace
        while pos < css.len() && css.chars().nth(pos).unwrap().is_whitespace() {
            pos += 1;
        }
        
        if pos >= css.len() || css.chars().nth(pos).unwrap() != '}' {
            return None;
        }
        pos += 1; // consume '}'
        
        Some((selectors.0, declarations.0, pos))
    }

    /// Enhanced selector parsing
    fn parse_selectors_enhanced(&mut self, css: &str, start_pos: usize) -> Option<(Vec<String>, usize)> {
        let mut selectors = Vec::new();
        let mut pos = start_pos;
        let mut current_selector = String::new();
        let mut paren_depth = 0;
        
        while pos < css.len() {
            let ch = css.chars().nth(pos).unwrap();
            
            match ch {
                '{' if paren_depth == 0 => {
                    break;
                }
                ',' if paren_depth == 0 => {
                    if !current_selector.trim().is_empty() {
                        selectors.push(current_selector.trim().to_string());
                    }
                    current_selector.clear();
                    pos += 1;
                    continue;
                }
                '(' => {
                    paren_depth += 1;
                    current_selector.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    current_selector.push(ch);
                }
                _ => {
                    current_selector.push(ch);
                }
            }
            pos += 1;
        }
        
        // Add the last selector
        if !current_selector.trim().is_empty() {
            selectors.push(current_selector.trim().to_string());
        }
        
        if selectors.is_empty() {
            None
        } else {
            Some((selectors, pos))
        }
    }

    /// Enhanced declaration parsing
    fn parse_declarations_enhanced(&mut self, css: &str, start_pos: usize) -> Option<(HashMap<String, String>, usize)> {
        let mut declarations = HashMap::new();
        let mut pos = start_pos;
        
        while pos < css.len() {
            // Skip whitespace
            while pos < css.len() && css.chars().nth(pos).unwrap().is_whitespace() {
                pos += 1;
            }
            
            if pos >= css.len() {
                break;
            }
            
            let ch = css.chars().nth(pos).unwrap();
            if ch == '}' {
                break;
            }
            
            // Parse property name
            let property_start = pos;
            while pos < css.len() {
                let ch = css.chars().nth(pos).unwrap();
                if ch == ':' || ch.is_whitespace() {
                    break;
                }
                pos += 1;
            }
            
            let property = css[property_start..pos].trim().to_lowercase();
            
            // Skip whitespace and colon
            while pos < css.len() && (css.chars().nth(pos).unwrap().is_whitespace() || css.chars().nth(pos).unwrap() == ':') {
                pos += 1;
            }
            
            // Parse property value
            let value_start = pos;
            let mut paren_depth = 0;
            let mut in_quotes = false;
            let mut quote_char = '\0';
            
            while pos < css.len() {
                let ch = css.chars().nth(pos).unwrap();
                
                if in_quotes {
                    if ch == quote_char {
                        in_quotes = false;
                    }
                } else {
                    match ch {
                        '"' | '\'' => {
                            in_quotes = true;
                            quote_char = ch;
                        }
                        '(' => paren_depth += 1,
                        ')' => paren_depth -= 1,
                        ';' if paren_depth == 0 => break,
                        '}' if paren_depth == 0 => break,
                        _ => {}
                    }
                }
                pos += 1;
            }
            
            let value = css[value_start..pos].trim().to_string();
            
            if !property.is_empty() && !value.is_empty() {
                declarations.insert(property, value);
                self.parsing_stats.declarations_parsed += 1;
            }
            
            // Skip semicolon
            if pos < css.len() && css.chars().nth(pos).unwrap() == ';' {
                pos += 1;
            }
        }
        
        Some((declarations, pos))
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
                self.apply_style_enhanced(&mut styles, &property, &value);
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

    /// Enhanced style application with more CSS properties
    fn apply_style_enhanced(&self, styles: &mut StyleMap, property: &str, value: &str) {
        match property.to_lowercase().as_str() {
            // Layout properties
            "display" => styles.display = value.to_string(),
            "position" => styles.position = value.to_string(),
            "top" => styles.top = value.to_string(),
            "right" => styles.right = value.to_string(),
            "bottom" => styles.bottom = value.to_string(),
            "left" => styles.left = value.to_string(),
            "z-index" | "zindex" => styles.z_index = value.to_string(),
            "float" => styles.float = value.to_string(),
            "clear" => styles.clear = value.to_string(),
            // Flexbox
            "flex" => styles.flex = value.to_string(),
            "flex-direction" | "flexdirection" => styles.flex_direction = value.to_string(),
            "flex-wrap" | "flexwrap" => styles.flex_wrap = value.to_string(),
            "justify-content" | "justifycontent" => styles.justify_content = value.to_string(),
            "align-items" | "alignitems" => styles.align_items = value.to_string(),
            "align-content" | "aligncontent" => styles.align_content = value.to_string(),
            "flex-grow" | "flexgrow" => styles.flex_grow = value.to_string(),
            "flex-shrink" | "flexshrink" => styles.flex_shrink = value.to_string(),
            "flex-basis" | "flexbasis" => styles.flex_basis = value.to_string(),
            "order" => styles.order = value.to_string(),
            // Grid
            "grid" => styles.grid = value.to_string(),
            "grid-template-columns" | "gridtemplatecolumns" => styles.grid_template_columns = value.to_string(),
            "grid-template-rows" | "gridtemplaterows" => styles.grid_template_rows = value.to_string(),
            "grid-gap" | "gridgap" => styles.grid_gap = value.to_string(),
            "grid-column" | "gridcolumn" => styles.grid_column = value.to_string(),
            "grid-row" | "gridrow" => styles.grid_row = value.to_string(),
            "grid-area" | "gridarea" => styles.grid_area = value.to_string(),
            // Box model
            "width" => styles.width = value.to_string(),
            "height" => styles.height = value.to_string(),
            "min-width" | "minwidth" => styles.min_width = value.to_string(),
            "max-width" | "maxwidth" => styles.max_width = value.to_string(),
            "min-height" | "minheight" => styles.min_height = value.to_string(),
            "max-height" | "maxheight" => styles.max_height = value.to_string(),
            "margin" => styles.margin = value.to_string(),
            "margin-top" | "margintop" => styles.margin_top = value.to_string(),
            "margin-right" | "marginright" => styles.margin_right = value.to_string(),
            "margin-bottom" | "marginbottom" => styles.margin_bottom = value.to_string(),
            "margin-left" | "marginleft" => styles.margin_left = value.to_string(),
            "padding" => styles.padding = value.to_string(),
            "padding-top" | "paddingtop" => styles.padding_top = value.to_string(),
            "padding-right" | "paddingright" => styles.padding_right = value.to_string(),
            "padding-bottom" | "paddingbottom" => styles.padding_bottom = value.to_string(),
            "padding-left" | "paddingleft" => styles.padding_left = value.to_string(),
            // Visual properties
            "background-color" | "backgroundcolor" => styles.background_color = value.to_string(),
            "background" => styles.background = value.to_string(),
            "background-image" | "backgroundimage" => styles.background_image = value.to_string(),
            "background-repeat" | "backgroundrepeat" => styles.background_repeat = value.to_string(),
            "background-position" | "backgroundposition" => styles.background_position = value.to_string(),
            "background-size" | "backgroundsize" => styles.background_size = value.to_string(),
            "color" => styles.color = value.to_string(),
            "opacity" => styles.opacity = value.to_string(),
            "visibility" => styles.visibility = value.to_string(),
            "overflow" => styles.overflow = value.to_string(),
            "overflow-x" | "overflowx" => styles.overflow_x = value.to_string(),
            "overflow-y" | "overflowy" => styles.overflow_y = value.to_string(),
            // Typography properties
            "font-size" | "fontsize" => styles.font_size = value.to_string(),
            "font-family" | "fontfamily" => styles.font_family = value.to_string(),
            "font-weight" | "fontweight" => styles.font_weight = value.to_string(),
            "font-style" | "fontstyle" => styles.font_style = value.to_string(),
            "font-variant" | "fontvariant" => styles.font_variant = value.to_string(),
            "text-align" | "textalign" => styles.text_align = value.to_string(),
            "text-decoration" | "textdecoration" => styles.text_decoration = value.to_string(),
            "text-transform" | "texttransform" => styles.text_transform = value.to_string(),
            "text-indent" | "textindent" => styles.text_indent = value.to_string(),
            "line-height" | "lineheight" => styles.line_height = value.to_string(),
            "letter-spacing" | "letterspacing" => styles.letter_spacing = value.to_string(),
            "word-spacing" | "wordspacing" => styles.word_spacing = value.to_string(),
            "white-space" | "whitespace" => styles.white_space = value.to_string(),
            // Border properties
            "border-width" | "borderwidth" => styles.border_width = value.to_string(),
            "border-color" | "bordercolor" => styles.border_color = value.to_string(),
            "border-style" | "borderstyle" => styles.border_style = value.to_string(),
            "border" => styles.border = value.to_string(),
            "border-radius" | "borderradius" => styles.border_radius = value.to_string(),
            "border-top" | "bordertop" => styles.border_top = value.to_string(),
            "border-right" | "borderright" => styles.border_right = value.to_string(),
            "border-bottom" | "borderbottom" => styles.border_bottom = value.to_string(),
            "border-left" | "borderleft" => styles.border_left = value.to_string(),
            "outline" => styles.outline = value.to_string(),
            "outline-width" | "outlinewidth" => styles.outline_width = value.to_string(),
            "outline-color" | "outlinecolor" => styles.outline_color = value.to_string(),
            "outline-style" | "outlinestyle" => styles.outline_style = value.to_string(),
            // Effects
            "box-shadow" | "boxshadow" => styles.box_shadow = value.to_string(),
            "text-shadow" | "textshadow" => styles.text_shadow = value.to_string(),
            // Cursor
            "cursor" => styles.cursor = value.to_string(),
            // User select
            "user-select" | "userselect" => styles.user_select = value.to_string(),
            // Pointer events
            "pointer-events" | "pointerevents" => styles.pointer_events = value.to_string(),
            // Future: add more advanced CSS properties as needed
            _ => {
                // Unknown property - store it anyway for future use
                println!("[CSS] Unknown property: {} = {}", property, value);
            }
        }
    }

    fn consume_char(&mut self) -> char {
        let ch = self.input.chars().nth(self.position).unwrap();
        self.position += 1;
        ch
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
        let rule = CssRule {
            selector,
            declarations,
            specificity,
        };
        self.rules.push(rule);
    }

    /// Enhanced specificity calculation
    fn calculate_specificity(selector: &str) -> u32 {
        let mut specificity = 0usize;
        let mut parts = selector.split_whitespace();
        
        for part in parts {
            let mut part_specificity = 0usize;
            
            // Count ID selectors (#id)
            part_specificity += part.matches('#').count() * 100;
            
            // Count class selectors (.class) and attribute selectors ([attr])
            part_specificity += part.matches('.').count() * 10;
            part_specificity += part.matches('[').count() * 10;
            
            // Count element selectors (tag names)
            if !part.starts_with('#') && !part.starts_with('.') && !part.starts_with('[') && !part.starts_with(':') {
                part_specificity += 1;
            }
            
            // Count pseudo-classes (:hover, :active, etc.)
            part_specificity += part.matches(':').count() * 10;
            
            specificity += part_specificity;
        }
        
        specificity.try_into().unwrap_or(0)
    }
}

fn remove_css_comments(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume '*'
            // Skip until */
            while let Some(ch) = chars.next() {
                if ch == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    break;
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

pub fn parse_css(css: &str) -> Stylesheet {
    let start_time = Instant::now();
    let mut parser = CSSParser::new(css.to_string());
    let stylesheet = parser.parse_enhanced();
    
    println!("Rust: CSS parsing completed: {} rules, {} declarations in {}ms", 
        stylesheet.rules.len(), 
        stylesheet.rules.iter().map(|r| r.declarations.len()).sum::<usize>(),
        start_time.elapsed().as_millis());
    
    stylesheet
} 
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct JavaScriptParser {
    input: String,
    position: usize,
    pub parsing_stats: JavaScriptParsingStats,
}

#[derive(Debug, Clone)]
pub struct JavaScriptParsingStats {
    pub total_chars: usize,
    pub statements_parsed: usize,
    pub functions_parsed: usize,
    pub variables_parsed: usize,
    pub parsing_time_ms: u64,
    pub memory_usage_mb: f64,
}

impl Default for JavaScriptParsingStats {
    fn default() -> Self {
        Self {
            total_chars: 0,
            statements_parsed: 0,
            functions_parsed: 0,
            variables_parsed: 0,
            parsing_time_ms: 0,
            memory_usage_mb: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum JavaScriptToken {
    Identifier(String),
    String(String),
    Number(f64),
    Keyword(String),
    Operator(String),
    Punctuation(String),
    Comment(String),
    Whitespace(String),
}

#[derive(Debug, Clone)]
pub struct JavaScriptStatement {
    pub statement_type: StatementType,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum StatementType {
    VariableDeclaration,
    FunctionDeclaration,
    Expression,
    IfStatement,
    ForLoop,
    WhileLoop,
    ReturnStatement,
    Assignment,
    FunctionCall,
    Comment,
}

#[derive(Debug, Clone)]
pub struct JavaScriptEngine {
    variables: HashMap<String, String>,
    functions: HashMap<String, String>,
    parsing_stats: JavaScriptParsingStats,
}

impl JavaScriptEngine {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parsing_stats: JavaScriptParsingStats::default(),
        }
    }

    /// Parse and execute JavaScript code
    pub fn execute(&mut self, code: &str) -> Result<String, String> {
        let start_time = Instant::now();
        println!("[JS] Executing JavaScript code: {} characters", code.len());
        
        // Basic JavaScript execution
        let result = self.execute_basic_js(code);
        
        self.parsing_stats.parsing_time_ms = start_time.elapsed().as_millis() as u64;
        println!("[JS] Execution completed in {}ms", self.parsing_stats.parsing_time_ms);
        
        result
    }

    /// Basic JavaScript execution (simplified)
    fn execute_basic_js(&mut self, code: &str) -> Result<String, String> {
        let lines: Vec<&str> = code.lines().collect();
        let mut output = String::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue; // Skip comments and empty lines
            }
            
            // Handle variable declarations
            if trimmed.starts_with("var ") || trimmed.starts_with("let ") || trimmed.starts_with("const ") {
                if let Some(var_name) = self.parse_variable_declaration(trimmed) {
                    self.parsing_stats.variables_parsed += 1;
                    println!("[JS] Variable declared: {}", var_name);
                }
            }
            // Handle function declarations
            else if trimmed.starts_with("function ") {
                if let Some(func_name) = self.parse_function_declaration(trimmed) {
                    self.parsing_stats.functions_parsed += 1;
                    println!("[JS] Function declared: {}", func_name);
                }
            }
            // Handle console.log
            else if trimmed.starts_with("console.log(") {
                if let Some(log_content) = self.parse_console_log(trimmed) {
                    output.push_str(&format!("[JS LOG] {}\n", log_content));
                    println!("[JS] Console log: {}", log_content);
                }
            }
            // Handle DOM manipulation
            else if trimmed.contains("document.") {
                self.handle_dom_manipulation(trimmed);
            }
            // Handle basic expressions
            else if trimmed.contains('=') && !trimmed.starts_with("==") && !trimmed.starts_with("===") {
                if let Some(assignment) = self.parse_assignment(trimmed) {
                    println!("[JS] Assignment: {}", assignment);
                }
            }
            
            self.parsing_stats.statements_parsed += 1;
        }
        
        Ok(output)
    }

    /// Parse variable declaration
    fn parse_variable_declaration(&mut self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let var_name = parts[1];
            if var_name.ends_with(';') {
                let var_name = var_name.trim_end_matches(';');
                self.variables.insert(var_name.to_string(), "undefined".to_string());
                return Some(var_name.to_string());
            }
        }
        None
    }

    /// Parse function declaration
    fn parse_function_declaration(&mut self, line: &str) -> Option<String> {
        if line.contains("function ") && line.contains('(') {
            let func_start = line.find("function ").unwrap() + 9;
            let func_end = line.find('(').unwrap();
            let func_name = line[func_start..func_end].trim();
            self.functions.insert(func_name.to_string(), line.to_string());
            return Some(func_name.to_string());
        }
        None
    }

    /// Parse console.log statement
    fn parse_console_log(&self, line: &str) -> Option<String> {
        if line.contains("console.log(") && line.contains(')') {
            let start = line.find("console.log(").unwrap() + 12;
            let end = line.rfind(')').unwrap();
            let content = line[start..end].trim_matches('"').trim_matches('\'');
            return Some(content.to_string());
        }
        None
    }

    /// Handle DOM manipulation
    fn handle_dom_manipulation(&mut self, line: &str) {
        if line.contains("document.getElementById") {
            println!("[JS] DOM manipulation: getElementById");
        } else if line.contains("document.querySelector") {
            println!("[JS] DOM manipulation: querySelector");
        } else if line.contains(".innerHTML") {
            println!("[JS] DOM manipulation: innerHTML");
        } else if line.contains(".style.") {
            println!("[JS] DOM manipulation: style property");
        }
    }

    /// Parse assignment
    fn parse_assignment(&mut self, line: &str) -> Option<String> {
        if let Some(equal_pos) = line.find('=') {
            let var_name = line[..equal_pos].trim();
            let value = line[equal_pos + 1..].trim().trim_matches(';');
            self.variables.insert(var_name.to_string(), value.to_string());
            return Some(format!("{} = {}", var_name, value));
        }
        None
    }

    /// Get parsing statistics
    pub fn get_stats(&self) -> &JavaScriptParsingStats {
        &self.parsing_stats
    }

    /// Get all variables
    pub fn get_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Get all functions
    pub fn get_functions(&self) -> &HashMap<String, String> {
        &self.functions
    }
}

impl JavaScriptParser {
    pub fn new(input: String) -> Self {
        let total_chars = input.len();
        println!("[JS] JavaScript Parser initialized for {} characters", total_chars);
        
        Self {
            input,
            position: 0,
            parsing_stats: JavaScriptParsingStats {
                total_chars,
                ..Default::default()
            },
        }
    }

    /// Parse JavaScript code into tokens
    pub fn parse(&mut self) -> Vec<JavaScriptToken> {
        let start_time = Instant::now();
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            
            match current_char {
                // Whitespace
                c if c.is_whitespace() => {
                    let whitespace = self.consume_whitespace();
                    tokens.push(JavaScriptToken::Whitespace(whitespace));
                }
                // Comments
                '/' => {
                    if self.position + 1 < self.input.len() {
                        let next_char = self.input.chars().nth(self.position + 1).unwrap();
                        if next_char == '/' {
                            let comment = self.consume_single_line_comment();
                            tokens.push(JavaScriptToken::Comment(comment));
                        } else if next_char == '*' {
                            let comment = self.consume_multi_line_comment();
                            tokens.push(JavaScriptToken::Comment(comment));
                        } else {
                            tokens.push(JavaScriptToken::Operator("/".to_string()));
                            self.position += 1;
                        }
                    } else {
                        tokens.push(JavaScriptToken::Operator("/".to_string()));
                        self.position += 1;
                    }
                }
                // Strings
                '"' | '\'' => {
                    let string = self.consume_string(current_char);
                    tokens.push(JavaScriptToken::String(string));
                }
                // Numbers
                c if c.is_numeric() => {
                    let number = self.consume_number();
                    tokens.push(JavaScriptToken::Number(number));
                }
                // Identifiers and keywords
                c if c.is_alphabetic() || c == '_' => {
                    let identifier = self.consume_identifier();
                    if self.is_keyword(&identifier) {
                        tokens.push(JavaScriptToken::Keyword(identifier));
                    } else {
                        tokens.push(JavaScriptToken::Identifier(identifier));
                    }
                }
                // Operators and punctuation
                _ => {
                    let op = self.consume_operator_or_punctuation();
                    tokens.push(op);
                }
            }
        }
        
        self.parsing_stats.parsing_time_ms = start_time.elapsed().as_millis() as u64;
        println!("[JS] Parsing completed: {} tokens in {}ms", 
            tokens.len(), self.parsing_stats.parsing_time_ms);
        
        tokens
    }

    fn consume_whitespace(&mut self) -> String {
        let mut whitespace = String::new();
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_whitespace() {
                whitespace.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }
        whitespace
    }

    fn consume_single_line_comment(&mut self) -> String {
        let mut comment = String::new();
        self.position += 2; // Skip //
        
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch == '\n' {
                break;
            }
            comment.push(ch);
            self.position += 1;
        }
        
        comment
    }

    fn consume_multi_line_comment(&mut self) -> String {
        let mut comment = String::new();
        self.position += 2; // Skip /*
        
        while self.position + 1 < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            let next_ch = self.input.chars().nth(self.position + 1).unwrap();
            
            if ch == '*' && next_ch == '/' {
                self.position += 2;
                break;
            }
            
            comment.push(ch);
            self.position += 1;
        }
        
        comment
    }

    fn consume_string(&mut self, quote_char: char) -> String {
        let mut string = String::new();
        self.position += 1; // Skip opening quote
        
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch == quote_char {
                self.position += 1;
                break;
            }
            string.push(ch);
            self.position += 1;
        }
        
        string
    }

    fn consume_number(&mut self) -> f64 {
        let mut number_str = String::new();
        
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_numeric() || ch == '.' {
                number_str.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }
        
        number_str.parse::<f64>().unwrap_or(0.0)
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }
        
        identifier
    }

    fn consume_operator_or_punctuation(&mut self) -> JavaScriptToken {
        let ch = self.input.chars().nth(self.position).unwrap();
        self.position += 1;
        
        match ch {
            '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' => {
                JavaScriptToken::Punctuation(ch.to_string())
            }
            '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' => {
                JavaScriptToken::Operator(ch.to_string())
            }
            _ => JavaScriptToken::Punctuation(ch.to_string()),
        }
    }

    fn is_keyword(&self, word: &str) -> bool {
        let keywords = vec![
            "var", "let", "const", "function", "if", "else", "for", "while", "do", "switch",
            "case", "default", "break", "continue", "return", "try", "catch", "finally",
            "throw", "new", "delete", "typeof", "instanceof", "void", "this", "super",
            "class", "extends", "static", "async", "await", "import", "export", "from",
            "console", "document", "window", "undefined", "null", "true", "false"
        ];
        
        keywords.contains(&word)
    }
}

/// Parse and execute JavaScript code
pub fn execute_javascript(code: &str) -> Result<String, String> {
    let mut engine = JavaScriptEngine::new();
    engine.execute(code)
}

/// Parse JavaScript code into tokens
pub fn parse_javascript(code: &str) -> Vec<JavaScriptToken> {
    let mut parser = JavaScriptParser::new(code.to_string());
    parser.parse()
} 
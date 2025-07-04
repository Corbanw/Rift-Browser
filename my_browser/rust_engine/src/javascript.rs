use deno_core::{JsRuntime, RuntimeOptions, OpState, Extension, op2};
use deno_core::error::AnyError;
use deno_core::v8::{self, HandleScope, Local, Object, String as V8String, Function, Value, Array};
use deno_core::serde_v8;
use deno_core::serde_json::Value as JsonValue;

use crate::dom::node::{DOMNode, NodeType, StyleMap, DOMArena};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

/// DOM mutation event types
#[derive(Debug, Clone)]
pub enum DomMutationEvent {
    ElementCreated { id: String, tag_name: String },
    ElementRemoved { id: String },
    AttributeChanged { id: String, name: String, value: String },
    TextContentChanged { id: String, content: String },
    StyleChanged { id: String, property: String, value: String },
    LayoutRecalculationNeeded,
}

/// Event listener for DOM mutations
pub type DomMutationListener = Box<dyn Fn(DomMutationEvent) + Send + Sync>;

/// JavaScript runtime with full DOM integration
pub struct JavaScriptRuntime {
    runtime: JsRuntime,
    arena: Arc<Mutex<DOMArena>>,
    root_id: String,
    mutation_listeners: Arc<Mutex<Vec<DomMutationListener>>>,
    event_queue: Arc<Mutex<Vec<DomMutationEvent>>>,
    element_counter: Arc<Mutex<u32>>,
}

impl JavaScriptRuntime {
    /// Initialize a new JavaScript runtime with DOM bindings
    pub fn new(arena: Arc<Mutex<DOMArena>>, root_id: String) -> Result<Self, AnyError> {
        let element_counter = Arc::new(Mutex::new(0));
        let mutation_listeners = Arc::new(Mutex::new(Vec::new()));
        let event_queue = Arc::new(Mutex::new(Vec::new()));
        
        // Create runtime with DOM extensions
        let mut runtime = JsRuntime::new(RuntimeOptions::default());
        
        // Initialize DOM API
        runtime.execute_script("dom_init", include_str!("dom_api.js"))?;

        Ok(Self {
            runtime,
            arena,
            root_id,
            mutation_listeners,
            event_queue,
            element_counter,
        })
    }

    /// Execute JavaScript code in the runtime
    pub fn execute_script(&mut self, script_name: &str, code: &str) -> Result<(), AnyError> {
        println!("[JS] Executing script: {}", script_name);
        let script_name_static: &'static str = Box::leak(script_name.to_string().into_boxed_str());
        let code_owned = code.to_string();
        let _fut = self.runtime.execute_script(script_name_static, code_owned)?;
        Ok(())
    }

    /// Execute JavaScript code asynchronously
    pub async fn execute_script_async(&mut self, script_name: &str, code: &str) -> Result<(), AnyError> {
        println!("[JS] Executing async script: {}", script_name);
        let script_name_static: &'static str = Box::leak(script_name.to_string().into_boxed_str());
        let code_owned = code.to_string();
        let _fut = self.runtime.execute_script(script_name_static, code_owned)?;
        Ok(())
    }

    /// Run the event loop for async operations
    pub fn run_event_loop(&mut self) -> Result<(), AnyError> {
        println!("[JS] Running event loop");
        
        // Process mutation events
        let events = {
            let mut queue = self.event_queue.lock().unwrap();
            queue.drain(..).collect::<Vec<_>>()
        };
        
        for event in events {
            self.process_mutation_event(event);
        }
        
        Ok(())
    }

    /// Get a reference to the DOM tree
    pub fn get_dom_tree(&self) -> Arc<Mutex<DOMNode>> {
        self.arena.lock().unwrap().get_node(&self.root_id).expect("Root DOM node not found")
    }

    /// Add a mutation listener
    pub fn add_mutation_listener(&self, listener: DomMutationListener) {
        let mut listeners = self.mutation_listeners.lock().unwrap();
        listeners.push(listener);
    }

    /// Process a DOM mutation event
    fn process_mutation_event(&self, event: DomMutationEvent) {
        let listeners = self.mutation_listeners.lock().unwrap();
        for listener in listeners.iter() {
            listener(event.clone());
        }
    }

    /// Queue a mutation event
    pub fn queue_mutation_event(&self, event: DomMutationEvent) {
        let mut queue = self.event_queue.lock().unwrap();
        queue.push(event);
    }

    /// Create a new DOM element
    pub fn create_element(&self, tag_name: &str) -> Result<String, AnyError> {
        let mut counter = self.element_counter.lock().unwrap();
        *counter += 1;
        let element_id = format!("element_{}", counter);
        
        // Create the element in the DOM tree
        let mut arena = self.arena.lock().unwrap();
        let new_node = DOMNode {
            id: element_id.clone(),
            node_type: NodeType::Element(tag_name.to_string()),
            text_content: String::new(),
            attributes: HashMap::new(),
            styles: StyleMap::default(),
            children: Vec::new(),
            event_listeners: HashMap::new(),
            parent: None,
        };

        // Add to DOM tree (simplified - would need proper parent reference)
        if let Some(root) = arena.get_node(&self.root_id) {
            root.lock().unwrap().children.push(element_id.clone());
        }
        
        // Queue mutation event
        self.queue_mutation_event(DomMutationEvent::ElementCreated {
            id: element_id.clone(),
            tag_name: tag_name.to_string(),
        });
        
        Ok(element_id)
    }

    /// Remove a DOM element
    pub fn remove_element(&self, element_id: &str) -> Result<(), AnyError> {
        let mut arena = self.arena.lock().unwrap();
        
        // Find and remove the element (simplified)
        if let Some(element) = arena.get_node(element_id) {
            element.lock().unwrap().children.retain(|child| child != element_id);
        }
        
        // Queue mutation event
        self.queue_mutation_event(DomMutationEvent::ElementRemoved {
            id: element_id.to_string(),
        });
        
        Ok(())
    }

    /// Set an element attribute
    pub fn set_attribute(&self, element_id: &str, name: &str, value: &str) -> Result<(), AnyError> {
        let mut arena = self.arena.lock().unwrap();
        
        // Find and update the element
        if let Some(element) = arena.get_node(element_id) {
            element.lock().unwrap().attributes.insert(name.to_string(), value.to_string());
            
            // Queue mutation event
            self.queue_mutation_event(DomMutationEvent::AttributeChanged {
                id: element_id.to_string(),
                name: name.to_string(),
                value: value.to_string(),
            });
        }
        
        Ok(())
    }

    /// Set element text content
    pub fn set_text_content(&self, element_id: &str, content: &str) -> Result<(), AnyError> {
        let mut arena = self.arena.lock().unwrap();
        
        // Find and update the element
        if let Some(element) = arena.get_node(element_id) {
            element.lock().unwrap().text_content = content.to_string();
            
            // Queue mutation event
            self.queue_mutation_event(DomMutationEvent::TextContentChanged {
                id: element_id.to_string(),
                content: content.to_string(),
            });
        }
        
        Ok(())
    }

    /// Set element style property
    pub fn set_style_property(&self, element_id: &str, property: &str, value: &str) -> Result<(), AnyError> {
        let mut arena = self.arena.lock().unwrap();
        
        // Find and update the element
        if let Some(element) = arena.get_node(element_id) {
            element.lock().unwrap().styles.set_property(property, value);
            
            // Queue mutation event
            self.queue_mutation_event(DomMutationEvent::StyleChanged {
                id: element_id.to_string(),
                property: property.to_string(),
                value: value.to_string(),
            });
            
            // Queue layout recalculation
            self.queue_mutation_event(DomMutationEvent::LayoutRecalculationNeeded);
        }
        
        Ok(())
    }
}

/// JavaScript script manager for handling page scripts
pub struct ScriptManager {
    runtime: JavaScriptRuntime,
    executed_scripts: Vec<String>,
    dom_mutation_handlers: Vec<Box<dyn Fn() -> Result<(), AnyError> + Send + Sync>>,
}

impl ScriptManager {
    pub fn new(arena: Arc<Mutex<DOMArena>>, root_id: String) -> Result<Self, AnyError> {
        let runtime = JavaScriptRuntime::new(arena, root_id)?;
        Ok(Self {
            runtime,
            executed_scripts: Vec::new(),
            dom_mutation_handlers: Vec::new(),
        })
    }

    /// Initialize the JavaScript environment
    pub fn initialize(&mut self) -> Result<(), AnyError> {
        println!("[JS] JavaScript runtime initialized");
        Ok(())
    }

    /// Execute a script from a <script> tag
    pub fn execute_script(&mut self, script_content: &str, script_name: &str) -> Result<(), AnyError> {
        if self.executed_scripts.contains(&script_name.to_string()) {
            println!("[JS] Script {} already executed, skipping", script_name);
            return Ok(());
        }

        println!("[JS] Executing script: {}", script_name);
        self.runtime.execute_script(script_name, script_content)?;
        self.executed_scripts.push(script_name.to_string());
        Ok(())
    }

    /// Execute an external script from URL
    pub async fn execute_external_script(&mut self, script_url: &str) -> Result<(), AnyError> {
        println!("[JS] Fetching external script: {}", script_url);
        
        // Fetch the script content
        let response = reqwest::get(script_url).await?;
        let script_content = response.text().await?;
        
        self.execute_script(&script_content, script_url)?;
        Ok(())
    }

    /// Add a DOM mutation handler
    pub fn add_mutation_handler<F>(&mut self, handler: F)
    where
        F: Fn() -> Result<(), AnyError> + Send + Sync + 'static,
    {
        self.dom_mutation_handlers.push(Box::new(handler));
    }

    /// Get the runtime for direct access
    pub fn get_runtime(&mut self) -> &mut JavaScriptRuntime {
        &mut self.runtime
    }

    /// Run the JavaScript event loop
    pub fn run_event_loop(&mut self) -> Result<(), AnyError> {
        self.runtime.run_event_loop()?;
        
        // Run mutation handlers
        for handler in &self.dom_mutation_handlers {
            handler()?;
        }
        
        Ok(())
    }
}

// DOM API JavaScript code
const DOM_API_JS: &str = r#"
// Global document object with full DOM API
window = {};
document = {
    createElement: function(tagName) {
        console.log('Creating element:', tagName);
        const elementId = window._createElement(tagName);
        return {
            id: elementId,
            tagName: tagName,
            attributes: {},
            style: {},
            children: [],
            
            setAttribute: function(name, value) {
                this.attributes[name] = value;
                window._setAttribute(this.id, name, value);
            },
            
            getAttribute: function(name) {
                return this.attributes[name] || null;
            },
            
            removeAttribute: function(name) {
                delete this.attributes[name];
                window._removeAttribute(this.id, name);
            },
            
            setTextContent: function(content) {
                this.textContent = content;
                window._setTextContent(this.id, content);
            },
            
            appendChild: function(child) {
                this.children.push(child);
                window._appendChild(this.id, child.id);
            },
            
            removeChild: function(child) {
                const index = this.children.indexOf(child);
                if (index > -1) {
                    this.children.splice(index, 1);
                    window._removeChild(this.id, child.id);
                }
            },
            
            addEventListener: function(event, handler) {
                if (!this.eventListeners) this.eventListeners = {};
                if (!this.eventListeners[event]) this.eventListeners[event] = [];
                this.eventListeners[event].push(handler);
                window._addEventListener(this.id, event);
            },
            
            removeEventListener: function(event, handler) {
                if (this.eventListeners && this.eventListeners[event]) {
                    const index = this.eventListeners[event].indexOf(handler);
                    if (index > -1) {
                        this.eventListeners[event].splice(index, 1);
                    }
                }
            }
        };
    },
    
    getElementById: function(id) {
        console.log('Getting element by ID:', id);
        const element = window._getElementById(id);
        return element;
    },
    
    querySelector: function(selector) {
        console.log('Querying selector:', selector);
        const element = window._querySelector(selector);
        return element;
    },
    
    querySelectorAll: function(selector) {
        console.log('Querying all selectors:', selector);
        const elements = window._querySelectorAll(selector);
        return elements;
    },
    
    addEventListener: function(event, handler) {
        console.log('Adding document event listener:', event);
        if (!this.eventListeners) this.eventListeners = {};
        if (!this.eventListeners[event]) this.eventListeners[event] = [];
        this.eventListeners[event].push(handler);
    },
    
    createTextNode: function(text) {
        const textId = window._createTextNode(text);
        return {
            id: textId,
            nodeType: 3,
            textContent: text,
            setTextContent: function(content) {
                this.textContent = content;
                window._setTextContent(this.id, content);
            }
        };
    }
};

// Console API
console = {
    log: function(...args) {
        console.log('JS Console:', ...args);
    },
    error: function(...args) {
        console.log('JS Error:', ...args);
    },
    warn: function(...args) {
        console.log('JS Warn:', ...args);
    },
    info: function(...args) {
        console.log('JS Info:', ...args);
    }
};

// Timer APIs
setTimeout = function(callback, delay) {
    console.log('Setting timeout:', delay);
    return window._setTimeout(callback, delay);
};

setInterval = function(callback, delay) {
    console.log('Setting interval:', delay);
    return window._setInterval(callback, delay);
};

clearTimeout = function(id) {
    console.log('Clearing timeout:', id);
    window._clearTimeout(id);
};

clearInterval = function(id) {
    console.log('Clearing interval:', id);
    window._clearInterval(id);
};

// Promise and async support
Promise = Promise || function(executor) {
    let resolve, reject;
    const promise = {
        then: function(onFulfilled, onRejected) {
            return promise;
        },
        catch: function(onRejected) {
            return promise;
        }
    };
    executor(resolve, reject);
    return promise;
};

// Event system
Event = function(type, options) {
    this.type = type;
    this.target = null;
    this.currentTarget = null;
    this.bubbles = options && options.bubbles || false;
    this.cancelable = options && options.cancelable || false;
    this.defaultPrevented = false;
    
    this.preventDefault = function() {
        this.defaultPrevented = true;
    };
    
    this.stopPropagation = function() {
        this.bubbles = false;
    };
};

// CustomEvent for custom events
CustomEvent = function(type, options) {
    Event.call(this, type, options);
    this.detail = options && options.detail || null;
};

// XMLHttpRequest for AJAX
XMLHttpRequest = function() {
    this.readyState = 0;
    this.status = 0;
    this.responseText = '';
    this.onreadystatechange = null;
    
    this.open = function(method, url, async) {
        this.method = method;
        this.url = url;
        this.async = async;
        this.readyState = 1;
        if (this.onreadystatechange) this.onreadystatechange();
    };
    
    this.send = function(data) {
        this.readyState = 4;
        this.status = 200;
        this.responseText = '{"success": true}';
        if (this.onreadystatechange) this.onreadystatechange();
    };
};

// Fetch API
fetch = function(url, options) {
    return new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest();
        xhr.onreadystatechange = function() {
            if (xhr.readyState === 4) {
                if (xhr.status === 200) {
                    resolve({
                        ok: true,
                        status: xhr.status,
                        text: () => Promise.resolve(xhr.responseText),
                        json: () => Promise.resolve(JSON.parse(xhr.responseText))
                    });
                } else {
                    reject(new Error('Request failed'));
                }
            }
        };
        xhr.open(options?.method || 'GET', url, true);
        xhr.send(options?.body);
    });
};

// JSON API
JSON = {
    parse: function(text) {
        try {
            return eval('(' + text + ')');
        } catch (e) {
            throw new Error('Invalid JSON');
        }
    },
    stringify: function(obj) {
        return JSON.stringify(obj);
    }
};

// Math and other global objects
Math = Math || {};
Date = Date || function() { return new Date(); };
Array = Array || function() { return []; };
Object = Object || function() { return {}; };
String = String || function() { return ''; };
Number = Number || function() { return 0; };
Boolean = Boolean || function() { return false; };
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_runtime_creation() {
        let arena = Arc::new(Mutex::new(DOMArena::new()));
        let root_id = String::new();
        let runtime = JavaScriptRuntime::new(arena, root_id);
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_script_execution() {
        let arena = Arc::new(Mutex::new(DOMArena::new()));
        let root_id = String::new();
        let mut runtime = JavaScriptRuntime::new(arena, root_id).unwrap();
        let result = runtime.execute_script("test", "console.log('Hello World');");
        assert!(result.is_ok());
    }
} 
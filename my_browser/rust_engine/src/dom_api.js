// Comprehensive DOM API for the browser engine
// This provides full DOM manipulation capabilities with event handling

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
            
            // --- DOM property accessors ---
            get textContent() {
                return window._getTextContent(this.id);
            },
            set textContent(value) {
                window._setTextContent(this.id, value);
            },
            get innerHTML() {
                return window._getInnerHtml(this.id);
            },
            set innerHTML(value) {
                window._setInnerHtml(this.id, value);
            },
            get outerHTML() {
                return window._getOuterHtml(this.id);
            },
            set outerHTML(value) {
                window._setOuterHtml(this.id, value);
            },
            get id() {
                return window._getId(this.id);
            },
            set id(value) {
                window._setId(this.id, value);
            },
            get tagName() {
                return window._getTagName(this.id);
            },
            get nodeType() {
                return window._getNodeType(this.id);
            },
            
            setAttribute: function(name, value) {
                window._setAttribute(this.id, name, value);
            },
            
            getAttribute: function(name) {
                return window._getAttribute(this.id, name);
            },
            
            removeAttribute: function(name) {
                window._removeAttribute(this.id, name);
            },
            
            hasAttribute: function(name) {
                return window._hasAttribute(this.id, name);
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
            },
            
            classList: {
                add: function(className) {
                    window._classListAdd(this.id, className);
                },
                remove: function(className) {
                    window._classListRemove(this.id, className);
                },
                toggle: function(className) {
                    window._classListToggle(this.id, className);
                },
                contains: function(className) {
                    return window._classListContains(this.id, className);
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

// --- Node Prototype ---
function Node() {}
Node.prototype = {
    get parentNode() {
        // TODO: FFI call to Rust to get parent node
        return window._getParentNode(this.id);
    },
    get childNodes() {
        // TODO: FFI call to Rust to get child nodes
        return window._getChildNodes(this.id);
    },
    get firstChild() {
        // TODO: FFI call to Rust to get first child
        return window._getFirstChild(this.id);
    },
    get lastChild() {
        // TODO: FFI call to Rust to get last child
        return window._getLastChild(this.id);
    },
    get nextSibling() {
        // TODO: FFI call to Rust to get next sibling
        return window._getNextSibling(this.id);
    },
    get previousSibling() {
        // TODO: FFI call to Rust to get previous sibling
        return window._getPreviousSibling(this.id);
    },
    insertBefore: function(newNode, referenceNode) {
        // TODO: FFI call to Rust to insert newNode before referenceNode
        window._insertBefore(this.id, newNode.id, referenceNode ? referenceNode.id : null);
    },
    replaceChild: function(newNode, oldNode) {
        // TODO: FFI call to Rust to replace oldNode with newNode
        window._replaceChild(this.id, newNode.id, oldNode.id);
    },
    cloneNode: function(deep) {
        // TODO: FFI call to Rust to clone node
        return window._cloneNode(this.id, !!deep);
    },
    remove: function() {
        // TODO: FFI call to Rust to remove this node
        window._removeNode(this.id);
    },
    contains: function(node) {
        // TODO: FFI call to Rust to check containment
        return window._containsNode(this.id, node.id);
    },
    get textContent() {
        // TODO: FFI call to Rust to get text content
        return window._getTextContent(this.id);
    },
    set textContent(value) {
        // TODO: FFI call to Rust to set text content
        window._setTextContent(this.id, value);
    },
    get innerHTML() {
        // TODO: FFI call to Rust to get innerHTML
        return window._getInnerHTML(this.id);
    },
    set innerHTML(value) {
        // TODO: FFI call to Rust to set innerHTML
        window._setInnerHTML(this.id, value);
    },
    get outerHTML() {
        // TODO: FFI call to Rust to get outerHTML
        return window._getOuterHTML(this.id);
    },
    set outerHTML(value) {
        // TODO: FFI call to Rust to set outerHTML
        window._setOuterHTML(this.id, value);
    }
};

// --- Element Prototype ---
function Element() {}
Element.prototype = Object.create(Node.prototype);
Element.prototype.classList = {
    add: function(className) {
        window._addClass(this.id, className);
    },
    remove: function(className) {
        window._removeClass(this.id, className);
    },
    contains: function(className) {
        return window._hasClass(this.id, className);
    }
};
Element.prototype.getAttribute = function(name) {
    return window._getAttribute(this.id, name);
};
Element.prototype.setAttribute = function(name, value) {
    window._setAttribute(this.id, name, value);
};
Element.prototype.removeAttribute = function(name) {
    window._removeAttribute(this.id, name);
};
Object.defineProperty(Element.prototype, 'className', {
    get: function() { return window._getClassName(this.id); },
    set: function(value) { window._setClassName(this.id, value); }
});
Object.defineProperty(Element.prototype, 'style', {
    get: function() { return window._getStyle(this.id); },
    set: function(value) { window._setStyle(this.id, value); }
});

// --- Patch createElement to use Element prototype ---
const _oldCreateElement = document.createElement;
document.createElement = function(tagName) {
    const el = _oldCreateElement.call(document, tagName);
    Object.setPrototypeOf(el, Element.prototype);
    return el;
};

// --- Patch createTextNode to use Node prototype ---
const _oldCreateTextNode = document.createTextNode;
document.createTextNode = function(text) {
    const node = _oldCreateTextNode.call(document, text);
    Object.setPrototypeOf(node, Node.prototype);
    return node;
};

// --- Event Handling Glue ---
window._eventCallbackRegistry = {};
window._eventCallbackIdCounter = 1;

function getOrCreateEventListeners(elem, type) {
    if (!elem._eventListeners) elem._eventListeners = {};
    if (!elem._eventListeners[type]) elem._eventListeners[type] = [];
    return elem._eventListeners[type];
}

function getOrCreateCallbackId(handler) {
    if (!handler._callbackId) {
        handler._callbackId = window._eventCallbackIdCounter++;
        window._eventCallbackRegistry[handler._callbackId] = handler;
    }
    return handler._callbackId;
}

// Patch element addEventListener/removeEventListener
function patchEventAPI(elem) {
    elem.addEventListener = function(type, handler) {
        const listeners = getOrCreateEventListeners(this, type);
        if (listeners.includes(handler)) return;
        listeners.push(handler);
        const callbackId = getOrCreateCallbackId(handler);
        window._addEventListener(this.id, type, callbackId);
    };
    elem.removeEventListener = function(type, handler) {
        const listeners = getOrCreateEventListeners(this, type);
        const idx = listeners.indexOf(handler);
        if (idx !== -1) listeners.splice(idx, 1);
        if (handler._callbackId) {
            window._removeEventListener(this.id, type, handler._callbackId);
            delete window._eventCallbackRegistry[handler._callbackId];
        }
    };
}

// Patch all created elements
const origCreateElement = document.createElement;
document.createElement = function(tagName) {
    const elem = origCreateElement.call(document, tagName);
    patchEventAPI(elem);
    return elem;
};

// Patch document itself for event listeners
patchEventAPI(document);

// Called from Dart/Flutter when an event occurs
window._invokeEventCallback = function(nodeId, type, callbackId) {
    // Find the element by nodeId (you may need a global registry of elements)
    // For now, just call the handler
    const handler = window._eventCallbackRegistry[callbackId];
    if (handler) {
        handler({ type, target: { id: nodeId } });
    }
};

// --- TODO: Implement FFI hooks in Rust for all window._* methods above ---
// ... existing code ... 
// DOM node manipulation FFI functions for the browser engine
// Extracted from functions.rs for modularization

use crate::dom::node::{DOMNode, DOMArena, NodeType, FFILayoutBox, NODE_ID_COUNTER};
use std::ffi::{c_char, CString};
use std::ptr;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::ffi::{safe_c_string_to_rust, safe_rust_string_to_c};

static ARENA: Lazy<Mutex<DOMArena>> = Lazy::new(|| Mutex::new(DOMArena::new()));

// --- DOM FFI function implementations ---
// (Full implementations restored from the old monolithic mod.rs)

#[no_mangle]
pub extern "C" fn dom_get_parent_node(node_id: u32) -> u32 {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        if let Some(parent_id) = &node.lock().unwrap().parent {
            return parent_id.parse().unwrap_or(0);
        }
    } else {
        eprintln!("dom_get_parent_node: node not found for id {}", node_id);
    }
    0
}

fn id_to_string(id: u32) -> String {
    id.to_string()
}

#[no_mangle]
pub extern "C" fn dom_get_child_nodes(node_id: u32, out_buf: *mut u32, max_len: usize) -> usize {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        let children = &node.lock().unwrap().children;
        let count = children.len().min(max_len);
        unsafe {
            for (i, child_id) in children.iter().take(count).enumerate() {
                let val = child_id.parse().unwrap_or(0);
                *out_buf.add(i) = val;
            }
        }
        return count;
    } else {
        eprintln!("dom_get_child_nodes: node not found for id {}", node_id);
    }
    0
}

#[no_mangle]
pub extern "C" fn dom_get_first_child(node_id: u32) -> u32 {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        if let Some(first) = node.lock().unwrap().children.first() {
            return first.parse().unwrap_or(0);
        }
    } else {
        eprintln!("dom_get_first_child: node not found for id {}", node_id);
    }
    0
}

#[no_mangle]
pub extern "C" fn dom_get_last_child(node_id: u32) -> u32 {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        if let Some(last) = node.lock().unwrap().children.last() {
            return last.parse().unwrap_or(0);
        }
    } else {
        eprintln!("dom_get_last_child: node not found for id {}", node_id);
    }
    0
}

#[no_mangle]
pub extern "C" fn dom_get_next_sibling(node_id: u32) -> u32 {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        if let Some(parent_id) = &node.lock().unwrap().parent {
            if let Some(parent) = arena.get_node(parent_id) {
                let siblings = &parent.lock().unwrap().children;
                if let Some(pos) = siblings.iter().position(|cid| cid == &id) {
                    if pos + 1 < siblings.len() {
                        return siblings[pos + 1].parse().unwrap_or(0);
                    }
                }
            } else {
                eprintln!("dom_get_next_sibling: parent not found for node id {}", node_id);
            }
        }
    } else {
        eprintln!("dom_get_next_sibling: node not found for id {}", node_id);
    }
    0
}

#[no_mangle]
pub extern "C" fn dom_get_previous_sibling(node_id: u32) -> u32 {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        if let Some(parent_id) = &node.lock().unwrap().parent {
            if let Some(parent) = arena.get_node(parent_id) {
                let siblings = &parent.lock().unwrap().children;
                if let Some(pos) = siblings.iter().position(|cid| cid == &id) {
                    if pos > 0 {
                        return siblings[pos - 1].parse().unwrap_or(0);
                    }
                }
            } else {
                eprintln!("dom_get_previous_sibling: parent not found for node id {}", node_id);
            }
        }
    } else {
        eprintln!("dom_get_previous_sibling: node not found for id {}", node_id);
    }
    0
}

#[no_mangle]
pub extern "C" fn dom_insert_before(parent_id: u32, new_node_id: u32, reference_node_id: u32) {
    let mut arena = ARENA.lock().unwrap();
    let parent_id_str = id_to_string(parent_id);
    let new_node_id_str = id_to_string(new_node_id);
    let reference_node_id_str = id_to_string(reference_node_id);
    if let Some(parent) = arena.get_node(&parent_id_str) {
        let mut parent = parent.lock().unwrap();
        let pos = parent.children.iter().position(|cid| cid == &reference_node_id_str);
        match pos {
            Some(idx) => parent.children.insert(idx, new_node_id_str.clone()),
            None => parent.children.push(new_node_id_str.clone()),
        }
        if let Some(new_node) = arena.get_node(&new_node_id_str) {
            new_node.lock().unwrap().parent = Some(parent_id_str);
        } else {
            eprintln!("dom_insert_before: new_node not found for id {}", new_node_id);
        }
    } else {
        eprintln!("dom_insert_before: parent not found for id {}", parent_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_replace_child(parent_id: u32, new_node_id: u32, old_node_id: u32) {
    let mut arena = ARENA.lock().unwrap();
    let parent_id_str = id_to_string(parent_id);
    let new_node_id_str = id_to_string(new_node_id);
    let old_node_id_str = id_to_string(old_node_id);
    if let Some(parent) = arena.get_node(&parent_id_str) {
        let mut parent = parent.lock().unwrap();
        if let Some(pos) = parent.children.iter().position(|cid| cid == &old_node_id_str) {
            parent.children[pos] = new_node_id_str.clone();
            if let Some(new_node) = arena.get_node(&new_node_id_str) {
                new_node.lock().unwrap().parent = Some(parent_id_str.clone());
            } else {
                eprintln!("dom_replace_child: new_node not found for id {}", new_node_id);
            }
            if let Some(old_node) = arena.get_node(&old_node_id_str) {
                old_node.lock().unwrap().parent = None;
            } else {
                eprintln!("dom_replace_child: old_node not found for id {}", old_node_id);
            }
        } else {
            eprintln!("dom_replace_child: old_node_id {} not found in parent's children", old_node_id);
        }
    } else {
        eprintln!("dom_replace_child: parent not found for id {}", parent_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_clone_node(node_id: u32, deep: bool) -> u32 {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        let node = node.lock().unwrap();
        let new_id = if deep {
            let clone = node.deep_clone(&mut arena);
            let new_id = clone.id.parse().unwrap_or(0);
            arena.add_node(clone);
            new_id
        } else {
            let mut clone = node.clone();
            clone.id = NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string();
            clone.parent = None;
            clone.children.clear();
            let new_id = clone.id.parse().unwrap_or(0);
            arena.add_node(clone);
            new_id
        };
        return new_id;
    } else {
        eprintln!("dom_clone_node: node not found for id {}", node_id);
    }
    0
}

#[no_mangle]
pub extern "C" fn dom_remove_node(node_id: u32) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        let parent_id_opt = node.lock().unwrap().parent.clone();
        if let Some(parent_id) = parent_id_opt {
            if let Some(parent) = arena.get_node(&parent_id) {
                parent.lock().unwrap().children.retain(|cid| cid != &id);
            } else {
                eprintln!("dom_remove_node: parent not found for id {}", parent_id);
            }
        }
        node.lock().unwrap().parent = None;
    } else {
        eprintln!("dom_remove_node: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_contains_node(parent_id: u32, child_id: u32) -> bool {
    let arena = ARENA.lock().unwrap();
    let parent_id_str = id_to_string(parent_id);
    let child_id_str = id_to_string(child_id);
    fn contains(arena: &DOMArena, parent_id: &str, child_id: &str) -> bool {
        if parent_id == child_id {
            return true;
        }
        if let Some(parent) = arena.get_node(parent_id) {
            for cid in &parent.lock().unwrap().children {
                if contains(arena, cid, child_id) {
                    return true;
                }
            }
        }
        false
    }
    if !arena.nodes.contains_key(&parent_id_str) {
        eprintln!("dom_contains_node: parent not found for id {}", parent_id);
        return false;
    }
    if !arena.nodes.contains_key(&child_id_str) {
        eprintln!("dom_contains_node: child not found for id {}", child_id);
        return false;
    }
    contains(&arena, &parent_id_str, &child_id_str)
}

#[no_mangle]
pub extern "C" fn dom_get_attribute(node_id: u32, name: *const c_char) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let name = match safe_c_string_to_rust(name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_get_attribute: name conversion failed: {}", e);
            return ptr::null_mut();
        }
    };
    if let Some(node) = arena.get_node(&id) {
        if let Some(val) = node.lock().unwrap().attributes.get(&name) {
            return CString::new(val.as_str()).unwrap().into_raw();
        }
    } else {
        eprintln!("dom_get_attribute: node not found for id {}", node_id);
    }
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn dom_set_attribute(node_id: u32, name: *const c_char, value: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let name = match safe_c_string_to_rust(name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_set_attribute: name conversion failed: {}", e);
            return;
        }
    };
    let value = match safe_c_string_to_rust(value) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_set_attribute: value conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        node.lock().unwrap().attributes.insert(name, value);
    } else {
        eprintln!("dom_set_attribute: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_remove_attribute(node_id: u32, name: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let name = match safe_c_string_to_rust(name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_remove_attribute: name conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        node.lock().unwrap().attributes.remove(&name);
    } else {
        eprintln!("dom_remove_attribute: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_has_attribute(node_id: u32, name: *const c_char) -> bool {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let name = match safe_c_string_to_rust(name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_has_attribute: name conversion failed: {}", e);
            return false;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        node.lock().unwrap().attributes.contains_key(&name)
    } else {
        eprintln!("dom_has_attribute: node not found for id {}", node_id);
        false
    }
}

#[no_mangle]
pub extern "C" fn dom_class_list_add(node_id: u32, class_name: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let class_name = match safe_c_string_to_rust(class_name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_class_list_add: class_name conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let mut node = node.lock().unwrap();
        let mut classes: Vec<String> = node.attributes.get("class")
            .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);
        if !classes.contains(&class_name) {
            classes.push(class_name);
            node.attributes.insert("class".to_string(), classes.join(" "));
        }
    } else {
        eprintln!("dom_class_list_add: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_class_list_remove(node_id: u32, class_name: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let class_name = match safe_c_string_to_rust(class_name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_class_list_remove: class_name conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let mut node = node.lock().unwrap();
        if let Some(class_attr) = node.attributes.get("class") {
            let classes: Vec<String> = class_attr.split_whitespace()
                .filter(|c| *c != class_name)
                .map(|s| s.to_string())
                .collect();
            node.attributes.insert("class".to_string(), classes.join(" "));
        }
    } else {
        eprintln!("dom_class_list_remove: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_class_list_toggle(node_id: u32, class_name: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let class_name = match safe_c_string_to_rust(class_name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_class_list_toggle: class_name conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let mut node = node.lock().unwrap();
        let mut classes: Vec<String> = node.attributes.get("class")
            .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);
        if classes.contains(&class_name) {
            classes.retain(|c| c != &class_name);
        } else {
            classes.push(class_name);
        }
        node.attributes.insert("class".to_string(), classes.join(" "));
    } else {
        eprintln!("dom_class_list_toggle: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_class_list_contains(node_id: u32, class_name: *const c_char) -> bool {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let class_name = match safe_c_string_to_rust(class_name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_class_list_contains: class_name conversion failed: {}", e);
            return false;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        if let Some(class_attr) = node.lock().unwrap().attributes.get("class") {
            return class_attr.split_whitespace().any(|c| c == class_name);
        }
    } else {
        eprintln!("dom_class_list_contains: node not found for id {}", node_id);
    }
    false
}

#[no_mangle]
pub extern "C" fn dom_get_text_content(node_id: u32) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    fn get_text(node: &DOMNode, arena: &DOMArena) -> String {
        match &node.node_type {
            NodeType::Text => node.text_content.clone(),
            _ => node.children.iter()
                .filter_map(|cid| arena.get_node(cid))
                .map(|c| get_text(&c.lock().unwrap(), arena))
                .collect::<Vec<_>>().join("")
        }
    }
    if let Some(node) = arena.get_node(&id) {
        let text = get_text(&node.lock().unwrap(), &arena);
        CString::new(text).unwrap().into_raw()
    } else {
        eprintln!("dom_get_text_content: node not found for id {}", node_id);
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn dom_set_text_content(node_id: u32, value: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let value = match safe_c_string_to_rust(value) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_set_text_content: value conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let mut node = node.lock().unwrap();
        match node.node_type {
            NodeType::Text => node.text_content = value,
            _ => {
                node.children.clear();
                let mut text_node = DOMNode::new(NodeType::Text);
                text_node.text_content = value;
                let new_id = NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string();
                text_node.id = new_id.clone();
                text_node.parent = Some(id.clone());
                arena.add_node(text_node);
                node.children.push(new_id);
            }
        }
    } else {
        eprintln!("dom_set_text_content: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_get_id(node_id: u32) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        let id_val = node.lock().unwrap().attributes.get("id").cloned().unwrap_or_default();
        CString::new(id_val).unwrap().into_raw()
    } else {
        eprintln!("dom_get_id: node not found for id {}", node_id);
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn dom_set_id(node_id: u32, value: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let value = match safe_c_string_to_rust(value) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_set_id: value conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        node.lock().unwrap().attributes.insert("id".to_string(), value);
    } else {
        eprintln!("dom_set_id: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_get_tag_name(node_id: u32) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        let tag = match &node.lock().unwrap().node_type {
            NodeType::Element(t) => t.clone(),
            _ => String::new(),
        };
        CString::new(tag).unwrap().into_raw()
    } else {
        eprintln!("dom_get_tag_name: node not found for id {}", node_id);
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn dom_get_node_type(node_id: u32) -> u32 {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        match node.lock().unwrap().node_type {
            NodeType::Element(_) => 1,
            NodeType::Text => 3,
            NodeType::Document => 9,
        }
    } else {
        eprintln!("dom_get_node_type: node not found for id {}", node_id);
        0
    }
}

fn serialize_html(node: &DOMNode, arena: &DOMArena, outer: bool) -> String {
    match &node.node_type {
        NodeType::Text => node.text_content.clone(),
        NodeType::Element(tag) => {
            let attrs = node.attributes.iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<_>>().join(" ");
            let open = if attrs.is_empty() { tag.clone() } else { format!("{} {}", tag, attrs) };
            let children_html = node.children.iter()
                .filter_map(|cid| arena.get_node(cid))
                .map(|c| serialize_html(&c.lock().unwrap(), arena, true))
                .collect::<Vec<_>>().join("");
            if outer {
                format!("<{}>{}</{}>", open, children_html, tag)
            } else {
                children_html
            }
        }
        NodeType::Document => node.children.iter()
            .filter_map(|cid| arena.get_node(cid))
            .map(|c| serialize_html(&c.lock().unwrap(), arena, true))
            .collect::<Vec<_>>().join("")
    }
}

#[no_mangle]
pub extern "C" fn dom_get_inner_html(node_id: u32) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        let html = serialize_html(&node.lock().unwrap(), &arena, false);
        CString::new(html).unwrap().into_raw()
    } else {
        eprintln!("dom_get_inner_html: node not found for id {}", node_id);
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn dom_get_outer_html(node_id: u32) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    if let Some(node) = arena.get_node(&id) {
        let html = serialize_html(&node.lock().unwrap(), &arena, true);
        CString::new(html).unwrap().into_raw()
    } else {
        eprintln!("dom_get_outer_html: node not found for id {}", node_id);
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn dom_set_inner_html(node_id: u32, value: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let value = match safe_c_string_to_rust(value) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_set_inner_html: value conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let mut node = node.lock().unwrap();
        node.children.clear();
        let mut text_node = DOMNode::new(NodeType::Text);
        text_node.text_content = value;
        let new_id = NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_string();
        text_node.id = new_id.clone();
        text_node.parent = Some(id.clone());
        arena.add_node(text_node);
        node.children.push(new_id);
    } else {
        eprintln!("dom_set_inner_html: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_set_outer_html(node_id: u32, value: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let value = match safe_c_string_to_rust(value) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_set_outer_html: value conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        // For now, just replace with a text node
        let mut node = node.lock().unwrap();
        node.node_type = NodeType::Text;
        node.text_content = value;
        node.children.clear();
        node.attributes.clear();
    } else {
        eprintln!("dom_set_outer_html: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_get_style(node_id: u32, name: *const c_char) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let node = match arena.get_node(&id) {
        Some(n) => n,
        None => return safe_rust_string_to_c("")
    };
    let name_str = match safe_c_string_to_rust(name) {
        Ok(s) => s,
        Err(_) => return safe_rust_string_to_c("")
    };
    let node_lock = node.lock().unwrap();
    let value = node_lock.styles.get_property(&name_str).unwrap_or("");
    safe_rust_string_to_c(value)
}

#[no_mangle]
pub extern "C" fn dom_set_style(node_id: u32, name: *const c_char, value: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let node = match arena.get_node(&id) {
        Some(n) => n,
        None => return,
    };
    let name_str = match safe_c_string_to_rust(name) {
        Ok(s) => s,
        Err(_) => return,
    };
    let value_str = match safe_c_string_to_rust(value) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut node = node.lock().unwrap();
    node.styles.set_property(&name_str, &value_str);
}

#[no_mangle]
pub extern "C" fn dom_remove_style(node_id: u32, name: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let node = match arena.get_node(&id) {
        Some(n) => n,
        None => return,
    };
    let name_str = match safe_c_string_to_rust(name) {
        Ok(s) => s,
        Err(_) => return,
    };
    node.lock().unwrap().styles.remove_property(&name_str);
}

#[no_mangle]
pub extern "C" fn dom_get_style_css_text(node_id: u32) -> *mut c_char {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let node = match arena.get_node(&id) {
        Some(n) => n,
        None => return safe_rust_string_to_c("")
    };
    let styles = &node.lock().unwrap().styles;
    let mut css_text = String::new();
    macro_rules! push_prop {
        ($prop:expr, $val:expr) => {
            if !$val.is_empty() {
                css_text.push_str($prop);
                css_text.push(':');
                css_text.push_str($val);
                css_text.push(';');
            }
        };
    }
    push_prop!("display", &styles.display);
    push_prop!("width", &styles.width);
    push_prop!("height", &styles.height);
    push_prop!("background-color", &styles.background_color);
    push_prop!("color", &styles.color);
    push_prop!("font-size", &styles.font_size);
    push_prop!("font-family", &styles.font_family);
    push_prop!("border-width", &styles.border_width);
    push_prop!("border-color", &styles.border_color);
    push_prop!("padding", &styles.padding);
    push_prop!("margin", &styles.margin);
    push_prop!("font-weight", &styles.font_weight);
    push_prop!("text-align", &styles.text_align);
    push_prop!("position", &styles.position);
    push_prop!("top", &styles.top);
    push_prop!("right", &styles.right);
    push_prop!("bottom", &styles.bottom);
    push_prop!("left", &styles.left);
    push_prop!("z-index", &styles.z_index);
    push_prop!("min-width", &styles.min_width);
    push_prop!("max-width", &styles.max_width);
    push_prop!("min-height", &styles.min_height);
    push_prop!("max-height", &styles.max_height);
    push_prop!("background", &styles.background);
    push_prop!("opacity", &styles.opacity);
    push_prop!("visibility", &styles.visibility);
    push_prop!("font-style", &styles.font_style);
    push_prop!("text-decoration", &styles.text_decoration);
    push_prop!("letter-spacing", &styles.letter_spacing);
    push_prop!("word-spacing", &styles.word_spacing);
    push_prop!("border-style", &styles.border_style);
    push_prop!("border", &styles.border);
    push_prop!("border-radius", &styles.border_radius);
    push_prop!("padding-top", &styles.padding_top);
    push_prop!("padding-right", &styles.padding_right);
    push_prop!("padding-bottom", &styles.padding_bottom);
    push_prop!("padding-left", &styles.padding_left);
    push_prop!("margin-top", &styles.margin_top);
    push_prop!("margin-right", &styles.margin_right);
    push_prop!("margin-bottom", &styles.margin_bottom);
    push_prop!("margin-left", &styles.margin_left);
    push_prop!("flex-direction", &styles.flex_direction);
    push_prop!("flex-wrap", &styles.flex_wrap);
    push_prop!("justify-content", &styles.justify_content);
    push_prop!("align-items", &styles.align_items);
    push_prop!("align-content", &styles.align_content);
    push_prop!("flex-grow", &styles.flex_grow);
    push_prop!("flex-shrink", &styles.flex_shrink);
    push_prop!("flex-basis", &styles.flex_basis);
    push_prop!("order", &styles.order);
    push_prop!("grid-template-columns", &styles.grid_template_columns);
    push_prop!("grid-template-rows", &styles.grid_template_rows);
    push_prop!("grid-gap", &styles.grid_gap);
    push_prop!("grid-column", &styles.grid_column);
    push_prop!("grid-row", &styles.grid_row);
    push_prop!("grid-area", &styles.grid_area);
    push_prop!("line-height", &styles.line_height);
    push_prop!("word-wrap", &styles.word_wrap);
    push_prop!("white-space", &styles.white_space);
    push_prop!("text-overflow", &styles.text_overflow);
    push_prop!("overflow", &styles.overflow);
    push_prop!("overflow-x", &styles.overflow_x);
    push_prop!("overflow-y", &styles.overflow_y);
    push_prop!("transform", &styles.transform);
    push_prop!("transform-origin", &styles.transform_origin);
    push_prop!("color-scheme", &styles.color_scheme);
    push_prop!("box-sizing", &styles.box_sizing);
    push_prop!("cursor", &styles.cursor);
    push_prop!("pointer-events", &styles.pointer_events);
    push_prop!("user-select", &styles.user_select);
    push_prop!("float", &styles.float);
    push_prop!("clear", &styles.clear);
    push_prop!("background-image", &styles.background_image);
    push_prop!("background-repeat", &styles.background_repeat);
    push_prop!("background-position", &styles.background_position);
    push_prop!("background-size", &styles.background_size);
    push_prop!("font-variant", &styles.font_variant);
    push_prop!("text-transform", &styles.text_transform);
    push_prop!("text-indent", &styles.text_indent);
    push_prop!("border-top", &styles.border_top);
    push_prop!("border-right", &styles.border_right);
    push_prop!("border-bottom", &styles.border_bottom);
    push_prop!("border-left", &styles.border_left);
    push_prop!("outline", &styles.outline);
    push_prop!("outline-width", &styles.outline_width);
    push_prop!("outline-color", &styles.outline_color);
    push_prop!("outline-style", &styles.outline_style);
    push_prop!("flex", &styles.flex);
    push_prop!("grid", &styles.grid);
    push_prop!("transition", &styles.transition);
    push_prop!("animation", &styles.animation);
    push_prop!("box-shadow", &styles.box_shadow);
    push_prop!("text-shadow", &styles.text_shadow);
    safe_rust_string_to_c(&css_text)
}

#[no_mangle]
pub extern "C" fn dom_set_style_css_text(node_id: u32, css_text: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let node = match arena.get_node(&id) {
        Some(n) => n,
        None => return,
    };
    let css_text_str = match safe_c_string_to_rust(css_text) {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut node = node.lock().unwrap();
    node.styles.clear();
    for decl in css_text_str.split(';') {
        if let Some((k, v)) = decl.split_once(':') {
            node.styles.set_property(k.trim(), v.trim());
        }
    }
}

#[no_mangle]
pub extern "C" fn dom_add_event_listener(node_id: u32, event_type: *const c_char, callback_id: u32) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let event_type = match safe_c_string_to_rust(event_type) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_add_event_listener: event_type conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let mut node = node.lock().unwrap();
        node.event_listeners.entry(event_type).or_default().push(callback_id);
    } else {
        eprintln!("dom_add_event_listener: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_remove_event_listener(node_id: u32, event_type: *const c_char) {
    let mut arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let event_type = match safe_c_string_to_rust(event_type) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_remove_event_listener: event_type conversion failed: {}", e);
            return;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let mut node = node.lock().unwrap();
        node.event_listeners.remove(&event_type);
    } else {
        eprintln!("dom_remove_event_listener: node not found for id {}", node_id);
    }
}

#[no_mangle]
pub extern "C" fn dom_dispatch_event(node_id: u32, event_type: *const c_char) -> bool {
    let arena = ARENA.lock().unwrap();
    let id = id_to_string(node_id);
    let event_type = match safe_c_string_to_rust(event_type) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dom_dispatch_event: event_type conversion failed: {}", e);
            return false;
        }
    };
    if let Some(node) = arena.get_node(&id) {
        let node = node.lock().unwrap();
        if let Some(callback) = node.event_listeners.get(&event_type) {
            // In a real implementation, this would execute the callback
            // For now, we just return true to indicate the event was handled
            true
        } else {
            false
        }
    } else {
        eprintln!("dom_dispatch_event: node not found for id {}", node_id);
        false
    }
}

// ... (Insert all pub extern "C" fn dom_get_*, dom_set_*, dom_insert_*, dom_remove_*, dom_class_list_*, dom_add_event_listener, dom_remove_event_listener, dom_dispatch_event, id_to_string, serialize_html, get_text, etc. here) ... 
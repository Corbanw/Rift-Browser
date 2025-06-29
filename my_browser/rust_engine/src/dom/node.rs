use std::collections::HashMap;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FFILayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub font_weight: f32,
    // Pointers to C strings for node_type, text_content, background_color, color, font_family, border_color, text_align
    pub node_type: *const c_char,
    pub text_content: *const c_char,
    pub background_color: *const c_char,
    pub color: *const c_char,
    pub font_family: *const c_char,
    pub border_color: *const c_char,
    pub text_align: *const c_char,
    // Box model values as primitives
    pub margin_top: f32,
    pub margin_right: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub padding_top: f32,
    pub padding_right: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,
    pub border_width_top: f32,
    pub border_width_right: f32,
    pub border_width_bottom: f32,
    pub border_width_left: f32,
}

#[derive(Debug, Clone)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub node_type: String,
    pub text_content: String,
    pub background_color: String,
    pub color: String,
    pub font_size: f32,
    pub font_family: String,
    pub border_width: BoxValues,
    pub border_color: String,
    pub padding: BoxValues,
    pub margin: BoxValues,
    pub font_weight: f32,
    pub text_align: String,
    // Flexbox properties
    pub flex_direction: String,
    pub flex_wrap: String,
    pub justify_content: String,
    pub align_items: String,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: String,
    pub order: i32,
    // Grid properties
    pub grid_column: String,
    pub grid_row: String,
    // Text rendering
    pub line_height: f32,
    pub word_wrap: String,
    pub white_space: String,
    pub text_overflow: String,
    // Theme support
    pub color_scheme: String,
}

#[derive(Debug, Clone)]
pub struct DOMNode {
    pub node_type: NodeType,
    pub children: Vec<DOMNode>,
    pub text_content: String,
    pub attributes: HashMap<String, String>,
    pub styles: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element(String),
    Text,
    Document,
}

#[derive(Debug, Clone)]
pub struct StyleMap {
    pub display: String,
    pub width: String,
    pub height: String,
    pub background_color: String,
    pub color: String,
    pub font_size: String,
    pub font_family: String,
    pub border_width: String,
    pub border_color: String,
    pub padding: String,
    pub margin: String,
    pub font_weight: String,
    pub text_align: String,
    // Layout properties
    pub position: String,
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub left: String,
    pub z_index: String,
    // Sizing properties
    pub min_width: String,
    pub max_width: String,
    pub min_height: String,
    pub max_height: String,
    // Visual properties
    pub background: String,
    pub opacity: String,
    pub visibility: String,
    // Typography properties
    pub font_style: String,
    pub text_decoration: String,
    pub letter_spacing: String,
    pub word_spacing: String,
    // Border properties
    pub border_style: String,
    pub border: String,
    pub border_radius: String,
    // Spacing properties
    pub padding_top: String,
    pub padding_right: String,
    pub padding_bottom: String,
    pub padding_left: String,
    pub margin_top: String,
    pub margin_right: String,
    pub margin_bottom: String,
    pub margin_left: String,
    // Flexbox properties
    pub flex_direction: String,
    pub flex_wrap: String,
    pub justify_content: String,
    pub align_items: String,
    pub align_content: String,
    pub flex_grow: String,
    pub flex_shrink: String,
    pub flex_basis: String,
    pub order: String,
    // Grid properties
    pub grid_template_columns: String,
    pub grid_template_rows: String,
    pub grid_gap: String,
    pub grid_column: String,
    pub grid_row: String,
    pub grid_area: String,
    // Text rendering
    pub line_height: String,
    pub word_wrap: String,
    pub white_space: String,
    pub text_overflow: String,
    pub overflow: String,
    pub overflow_x: String,
    pub overflow_y: String,
    // Transform properties
    pub transform: String,
    pub transform_origin: String,
    // Theme support
    pub color_scheme: String,
    // Box model
    pub box_sizing: String,
    // Cursor
    pub cursor: String,
    // Pointer events
    pub pointer_events: String,
    // User select
    pub user_select: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoxValues {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for StyleMap {
    fn default() -> Self {
        Self {
            display: "block".to_string(),
            width: "auto".to_string(),
            height: "auto".to_string(),
            background_color: "transparent".to_string(),
            color: "black".to_string(),
            font_size: "16".to_string(),
            font_family: "Arial".to_string(),
            border_width: "0".to_string(),
            border_color: "black".to_string(),
            padding: "0".to_string(),
            margin: "0".to_string(),
            font_weight: "400".to_string(),
            text_align: "left".to_string(),
            flex_direction: "row".to_string(),
            flex_wrap: "nowrap".to_string(),
            justify_content: "flex-start".to_string(),
            align_items: "stretch".to_string(),
            align_content: "stretch".to_string(),
            flex_grow: "0".to_string(),
            flex_shrink: "1".to_string(),
            flex_basis: "0%".to_string(),
            order: "0".to_string(),
            grid_template_columns: "auto".to_string(),
            grid_template_rows: "auto".to_string(),
            grid_gap: "0".to_string(),
            grid_column: "auto".to_string(),
            grid_row: "auto".to_string(),
            grid_area: "auto".to_string(),
            line_height: "normal".to_string(),
            word_wrap: "normal".to_string(),
            white_space: "normal".to_string(),
            text_overflow: "clip".to_string(),
            color_scheme: "light".to_string(),
            position: "static".to_string(),
            top: "auto".to_string(),
            right: "auto".to_string(),
            bottom: "auto".to_string(),
            left: "auto".to_string(),
            z_index: "auto".to_string(),
            min_width: "0".to_string(),
            max_width: "none".to_string(),
            min_height: "0".to_string(),
            max_height: "none".to_string(),
            background: "transparent".to_string(),
            opacity: "1".to_string(),
            visibility: "visible".to_string(),
            font_style: "normal".to_string(),
            text_decoration: "none".to_string(),
            letter_spacing: "normal".to_string(),
            word_spacing: "normal".to_string(),
            border_style: "none".to_string(),
            border: "0".to_string(),
            border_radius: "0".to_string(),
            padding_top: "0".to_string(),
            padding_right: "0".to_string(),
            padding_bottom: "0".to_string(),
            padding_left: "0".to_string(),
            margin_top: "0".to_string(),
            margin_right: "0".to_string(),
            margin_bottom: "0".to_string(),
            margin_left: "0".to_string(),
            overflow: "visible".to_string(),
            overflow_x: "visible".to_string(),
            overflow_y: "visible".to_string(),
            transform: "none".to_string(),
            transform_origin: "50% 50%".to_string(),
            box_sizing: "content-box".to_string(),
            cursor: "default".to_string(),
            pointer_events: "auto".to_string(),
            user_select: "text".to_string(),
        }
    }
}

impl StyleMap {
    pub fn merge(&mut self, other: &StyleMap) {
        if !other.display.is_empty() { self.display = other.display.clone(); }
        if !other.width.is_empty() { self.width = other.width.clone(); }
        if !other.height.is_empty() { self.height = other.height.clone(); }
        if !other.background_color.is_empty() { self.background_color = other.background_color.clone(); }
        if !other.color.is_empty() { self.color = other.color.clone(); }
        if !other.font_size.is_empty() { self.font_size = other.font_size.clone(); }
        if !other.font_family.is_empty() { self.font_family = other.font_family.clone(); }
        if !other.border_width.is_empty() { self.border_width = other.border_width.clone(); }
        if !other.border_color.is_empty() { self.border_color = other.border_color.clone(); }
        if !other.padding.is_empty() { self.padding = other.padding.clone(); }
        if !other.margin.is_empty() { self.margin = other.margin.clone(); }
        if !other.font_weight.is_empty() { self.font_weight = other.font_weight.clone(); }
        if !other.text_align.is_empty() { self.text_align = other.text_align.clone(); }
        if !other.position.is_empty() { self.position = other.position.clone(); }
        if !other.top.is_empty() { self.top = other.top.clone(); }
        if !other.right.is_empty() { self.right = other.right.clone(); }
        if !other.bottom.is_empty() { self.bottom = other.bottom.clone(); }
        if !other.left.is_empty() { self.left = other.left.clone(); }
        if !other.z_index.is_empty() { self.z_index = other.z_index.clone(); }
        if !other.min_width.is_empty() { self.min_width = other.min_width.clone(); }
        if !other.max_width.is_empty() { self.max_width = other.max_width.clone(); }
        if !other.min_height.is_empty() { self.min_height = other.min_height.clone(); }
        if !other.max_height.is_empty() { self.max_height = other.max_height.clone(); }
        if !other.background.is_empty() { self.background = other.background.clone(); }
        if !other.opacity.is_empty() { self.opacity = other.opacity.clone(); }
        if !other.visibility.is_empty() { self.visibility = other.visibility.clone(); }
        if !other.font_style.is_empty() { self.font_style = other.font_style.clone(); }
        if !other.text_decoration.is_empty() { self.text_decoration = other.text_decoration.clone(); }
        if !other.letter_spacing.is_empty() { self.letter_spacing = other.letter_spacing.clone(); }
        if !other.word_spacing.is_empty() { self.word_spacing = other.word_spacing.clone(); }
        if !other.border_style.is_empty() { self.border_style = other.border_style.clone(); }
        if !other.border.is_empty() { self.border = other.border.clone(); }
        if !other.border_radius.is_empty() { self.border_radius = other.border_radius.clone(); }
        if !other.padding_top.is_empty() { self.padding_top = other.padding_top.clone(); }
        if !other.padding_right.is_empty() { self.padding_right = other.padding_right.clone(); }
        if !other.padding_bottom.is_empty() { self.padding_bottom = other.padding_bottom.clone(); }
        if !other.padding_left.is_empty() { self.padding_left = other.padding_left.clone(); }
        if !other.margin_top.is_empty() { self.margin_top = other.margin_top.clone(); }
        if !other.margin_right.is_empty() { self.margin_right = other.margin_right.clone(); }
        if !other.margin_bottom.is_empty() { self.margin_bottom = other.margin_bottom.clone(); }
        if !other.margin_left.is_empty() { self.margin_left = other.margin_left.clone(); }
        if !other.flex_direction.is_empty() { self.flex_direction = other.flex_direction.clone(); }
        if !other.flex_wrap.is_empty() { self.flex_wrap = other.flex_wrap.clone(); }
        if !other.justify_content.is_empty() { self.justify_content = other.justify_content.clone(); }
        if !other.align_items.is_empty() { self.align_items = other.align_items.clone(); }
        if !other.align_content.is_empty() { self.align_content = other.align_content.clone(); }
        if !other.flex_grow.is_empty() { self.flex_grow = other.flex_grow.clone(); }
        if !other.flex_shrink.is_empty() { self.flex_shrink = other.flex_shrink.clone(); }
        if !other.flex_basis.is_empty() { self.flex_basis = other.flex_basis.clone(); }
        if !other.order.is_empty() { self.order = other.order.clone(); }
        if !other.grid_template_columns.is_empty() { self.grid_template_columns = other.grid_template_columns.clone(); }
        if !other.grid_template_rows.is_empty() { self.grid_template_rows = other.grid_template_rows.clone(); }
        if !other.grid_gap.is_empty() { self.grid_gap = other.grid_gap.clone(); }
        if !other.grid_column.is_empty() { self.grid_column = other.grid_column.clone(); }
        if !other.grid_row.is_empty() { self.grid_row = other.grid_row.clone(); }
        if !other.grid_area.is_empty() { self.grid_area = other.grid_area.clone(); }
        if !other.line_height.is_empty() { self.line_height = other.line_height.clone(); }
        if !other.word_wrap.is_empty() { self.word_wrap = other.word_wrap.clone(); }
        if !other.white_space.is_empty() { self.white_space = other.white_space.clone(); }
        if !other.text_overflow.is_empty() { self.text_overflow = other.text_overflow.clone(); }
        if !other.overflow.is_empty() { self.overflow = other.overflow.clone(); }
        if !other.overflow_x.is_empty() { self.overflow_x = other.overflow_x.clone(); }
        if !other.overflow_y.is_empty() { self.overflow_y = other.overflow_y.clone(); }
        if !other.transform.is_empty() { self.transform = other.transform.clone(); }
        if !other.transform_origin.is_empty() { self.transform_origin = other.transform_origin.clone(); }
        if !other.color_scheme.is_empty() { self.color_scheme = other.color_scheme.clone(); }
        if !other.box_sizing.is_empty() { self.box_sizing = other.box_sizing.clone(); }
        if !other.cursor.is_empty() { self.cursor = other.cursor.clone(); }
        if !other.pointer_events.is_empty() { self.pointer_events = other.pointer_events.clone(); }
        if !other.user_select.is_empty() { self.user_select = other.user_select.clone(); }
    }
}

impl LayoutBox {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            node_type: String::new(),
            text_content: String::new(),
            background_color: "transparent".to_string(),
            color: "black".to_string(),
            font_size: 16.0,
            font_family: "Arial".to_string(),
            border_width: BoxValues::default(),
            border_color: String::new(),
            padding: BoxValues::default(),
            margin: BoxValues::default(),
            font_weight: 400.0,
            text_align: "left".to_string(),
            flex_direction: String::new(),
            flex_wrap: String::new(),
            justify_content: String::new(),
            align_items: String::new(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: String::new(),
            order: 0,
            grid_column: String::new(),
            grid_row: String::new(),
            line_height: 0.0,
            word_wrap: String::new(),
            white_space: String::new(),
            text_overflow: String::new(),
            color_scheme: String::new(),
        }
    }

    /// Convert to FFI-safe FFILayoutBox. Caller is responsible for freeing C strings.
    pub fn to_ffi(&self) -> FFILayoutBox {
        use std::ffi::CString;
        
        // Helper function to safely create CString, returns null on error
        fn safe_cstring(s: &str) -> *const c_char {
            // Truncate very long strings to prevent issues
            let truncated = if s.len() > 500 {
                &s[..500]
            } else {
                s
            };
            
            // Clean the string: remove null bytes and non-printable chars
            let clean_string: String = truncated
                .chars()
                .filter(|c| *c != '\0' && (*c >= ' ' || *c == '\n' || *c == '\t'))
                .collect();
            
            match CString::new(clean_string) {
                Ok(cstr) => cstr.into_raw() as *const c_char,
                Err(_) => {
                    // If string creation still fails, return empty string
                    match CString::new("") {
                        Ok(empty) => empty.into_raw() as *const c_char,
                        Err(_) => std::ptr::null(),
                    }
                }
            }
        }
        
        FFILayoutBox {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            font_size: self.font_size,
            font_weight: self.font_weight,
            node_type: safe_cstring(&self.node_type),
            text_content: safe_cstring(&self.text_content),
            background_color: safe_cstring(&self.background_color),
            color: safe_cstring(&self.color),
            font_family: safe_cstring(&self.font_family),
            border_color: safe_cstring(&self.border_color),
            text_align: safe_cstring(&self.text_align),
            margin_top: self.margin.top,
            margin_right: self.margin.right,
            margin_bottom: self.margin.bottom,
            margin_left: self.margin.left,
            padding_top: self.padding.top,
            padding_right: self.padding.right,
            padding_bottom: self.padding.bottom,
            padding_left: self.padding.left,
            border_width_top: self.border_width.top,
            border_width_right: self.border_width.right,
            border_width_bottom: self.border_width.bottom,
            border_width_left: self.border_width.left,
        }
    }
}

impl DOMNode {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            children: Vec::new(),
            text_content: String::new(),
            attributes: HashMap::new(),
            styles: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, child: DOMNode) {
        self.children.push(child);
    }

    pub fn set_text_content(&mut self, text: String) {
        self.text_content = text;
    }

    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn set_style(&mut self, key: String, value: String) {
        self.styles.insert(key, value);
    }
} 
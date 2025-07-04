use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use std::io::Write;

pub(crate) static NODE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

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
    pub id: String,
    pub node_type: NodeType,
    pub children: Vec<String>, // IDs of children
    pub parent: Option<String>,
    pub text_content: String,
    pub attributes: HashMap<String, String>,
    pub styles: StyleMap,
    pub event_listeners: HashMap<String, Vec<u32>>,
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
    // Additional CSS properties
    pub float: String,
    pub clear: String,
    pub background_image: String,
    pub background_repeat: String,
    pub background_position: String,
    pub background_size: String,
    pub font_variant: String,
    pub text_transform: String,
    pub text_indent: String,
    pub border_top: String,
    pub border_right: String,
    pub border_bottom: String,
    pub border_left: String,
    pub outline: String,
    pub outline_width: String,
    pub outline_color: String,
    pub outline_style: String,
    pub flex: String,
    pub grid: String,
    pub transition: String,
    pub animation: String,
    pub box_shadow: String,
    pub text_shadow: String,
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
            // Additional CSS properties
            float: "none".to_string(),
            clear: "none".to_string(),
            background_image: "none".to_string(),
            background_repeat: "repeat".to_string(),
            background_position: "0% 0%".to_string(),
            background_size: "auto".to_string(),
            font_variant: "normal".to_string(),
            text_transform: "none".to_string(),
            text_indent: "0".to_string(),
            border_top: "none".to_string(),
            border_right: "none".to_string(),
            border_bottom: "none".to_string(),
            border_left: "none".to_string(),
            outline: "none".to_string(),
            outline_width: "medium".to_string(),
            outline_color: "invert".to_string(),
            outline_style: "none".to_string(),
            flex: "0 1 auto".to_string(),
            grid: "none".to_string(),
            transition: "none".to_string(),
            animation: "none".to_string(),
            box_shadow: "none".to_string(),
            text_shadow: "none".to_string(),
        }
    }
}

impl StyleMap {
    pub fn set_property(&mut self, property: &str, value: &str) {
        match property {
            "display" => self.display = value.to_string(),
            "width" => self.width = value.to_string(),
            "height" => self.height = value.to_string(),
            "background-color" => self.background_color = value.to_string(),
            "color" => self.color = value.to_string(),
            "font-size" => self.font_size = value.to_string(),
            "font-family" => self.font_family = value.to_string(),
            "border-width" => self.border_width = value.to_string(),
            "border-color" => self.border_color = value.to_string(),
            "padding" => self.padding = value.to_string(),
            "margin" => self.margin = value.to_string(),
            "font-weight" => self.font_weight = value.to_string(),
            "text-align" => self.text_align = value.to_string(),
            "position" => self.position = value.to_string(),
            "top" => self.top = value.to_string(),
            "right" => self.right = value.to_string(),
            "bottom" => self.bottom = value.to_string(),
            "left" => self.left = value.to_string(),
            "z-index" => self.z_index = value.to_string(),
            "min-width" => self.min_width = value.to_string(),
            "max-width" => self.max_width = value.to_string(),
            "min-height" => self.min_height = value.to_string(),
            "max-height" => self.max_height = value.to_string(),
            "background" => self.background = value.to_string(),
            "opacity" => self.opacity = value.to_string(),
            "visibility" => self.visibility = value.to_string(),
            "font-style" => self.font_style = value.to_string(),
            "text-decoration" => self.text_decoration = value.to_string(),
            "letter-spacing" => self.letter_spacing = value.to_string(),
            "word-spacing" => self.word_spacing = value.to_string(),
            "border-style" => self.border_style = value.to_string(),
            "border" => self.border = value.to_string(),
            "border-radius" => self.border_radius = value.to_string(),
            "padding-top" => self.padding_top = value.to_string(),
            "padding-right" => self.padding_right = value.to_string(),
            "padding-bottom" => self.padding_bottom = value.to_string(),
            "padding-left" => self.padding_left = value.to_string(),
            "margin-top" => self.margin_top = value.to_string(),
            "margin-right" => self.margin_right = value.to_string(),
            "margin-bottom" => self.margin_bottom = value.to_string(),
            "margin-left" => self.margin_left = value.to_string(),
            "flex-direction" => self.flex_direction = value.to_string(),
            "flex-wrap" => self.flex_wrap = value.to_string(),
            "justify-content" => self.justify_content = value.to_string(),
            "align-items" => self.align_items = value.to_string(),
            "align-content" => self.align_content = value.to_string(),
            "flex-grow" => self.flex_grow = value.to_string(),
            "flex-shrink" => self.flex_shrink = value.to_string(),
            "flex-basis" => self.flex_basis = value.to_string(),
            "order" => self.order = value.to_string(),
            "grid-template-columns" => self.grid_template_columns = value.to_string(),
            "grid-template-rows" => self.grid_template_rows = value.to_string(),
            "grid-gap" => self.grid_gap = value.to_string(),
            "grid-column" => self.grid_column = value.to_string(),
            "grid-row" => self.grid_row = value.to_string(),
            "grid-area" => self.grid_area = value.to_string(),
            "line-height" => self.line_height = value.to_string(),
            "word-wrap" => self.word_wrap = value.to_string(),
            "white-space" => self.white_space = value.to_string(),
            "text-overflow" => self.text_overflow = value.to_string(),
            "overflow" => self.overflow = value.to_string(),
            "overflow-x" => self.overflow_x = value.to_string(),
            "overflow-y" => self.overflow_y = value.to_string(),
            "transform" => self.transform = value.to_string(),
            "transform-origin" => self.transform_origin = value.to_string(),
            "color-scheme" => self.color_scheme = value.to_string(),
            "box-sizing" => self.box_sizing = value.to_string(),
            "cursor" => self.cursor = value.to_string(),
            "pointer-events" => self.pointer_events = value.to_string(),
            "user-select" => self.user_select = value.to_string(),
            "float" => self.float = value.to_string(),
            "clear" => self.clear = value.to_string(),
            "background-image" => self.background_image = value.to_string(),
            "background-repeat" => self.background_repeat = value.to_string(),
            "background-position" => self.background_position = value.to_string(),
            "background-size" => self.background_size = value.to_string(),
            "font-variant" => self.font_variant = value.to_string(),
            "text-transform" => self.text_transform = value.to_string(),
            "text-indent" => self.text_indent = value.to_string(),
            "border-top" => self.border_top = value.to_string(),
            "border-right" => self.border_right = value.to_string(),
            "border-bottom" => self.border_bottom = value.to_string(),
            "border-left" => self.border_left = value.to_string(),
            "outline" => self.outline = value.to_string(),
            "outline-width" => self.outline_width = value.to_string(),
            "outline-color" => self.outline_color = value.to_string(),
            "outline-style" => self.outline_style = value.to_string(),
            "flex" => self.flex = value.to_string(),
            "grid" => self.grid = value.to_string(),
            "transition" => self.transition = value.to_string(),
            "animation" => self.animation = value.to_string(),
            "box-shadow" => self.box_shadow = value.to_string(),
            "text-shadow" => self.text_shadow = value.to_string(),
            _ => {
                // For unknown properties, we could store them in a generic map
                // For now, just ignore them
                println!("[CSS] Unknown property: {} = {}", property, value);
            }
        }
    }

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
        // Additional CSS properties
        if !other.float.is_empty() { self.float = other.float.clone(); }
        if !other.clear.is_empty() { self.clear = other.clear.clone(); }
        if !other.background_image.is_empty() { self.background_image = other.background_image.clone(); }
        if !other.background_repeat.is_empty() { self.background_repeat = other.background_repeat.clone(); }
        if !other.background_position.is_empty() { self.background_position = other.background_position.clone(); }
        if !other.background_size.is_empty() { self.background_size = other.background_size.clone(); }
        if !other.font_variant.is_empty() { self.font_variant = other.font_variant.clone(); }
        if !other.text_transform.is_empty() { self.text_transform = other.text_transform.clone(); }
        if !other.text_indent.is_empty() { self.text_indent = other.text_indent.clone(); }
        if !other.border_top.is_empty() { self.border_top = other.border_top.clone(); }
        if !other.border_right.is_empty() { self.border_right = other.border_right.clone(); }
        if !other.border_bottom.is_empty() { self.border_bottom = other.border_bottom.clone(); }
        if !other.border_left.is_empty() { self.border_left = other.border_left.clone(); }
        if !other.outline.is_empty() { self.outline = other.outline.clone(); }
        if !other.outline_width.is_empty() { self.outline_width = other.outline_width.clone(); }
        if !other.outline_color.is_empty() { self.outline_color = other.outline_color.clone(); }
        if !other.outline_style.is_empty() { self.outline_style = other.outline_style.clone(); }
        if !other.flex.is_empty() { self.flex = other.flex.clone(); }
        if !other.grid.is_empty() { self.grid = other.grid.clone(); }
        if !other.transition.is_empty() { self.transition = other.transition.clone(); }
        if !other.animation.is_empty() { self.animation = other.animation.clone(); }
        if !other.box_shadow.is_empty() { self.box_shadow = other.box_shadow.clone(); }
        if !other.text_shadow.is_empty() { self.text_shadow = other.text_shadow.clone(); }
    }

    pub fn get_property(&self, property: &str) -> Option<&str> {
        match property {
            "display" => Some(&self.display),
            "width" => Some(&self.width),
            "height" => Some(&self.height),
            "background-color" => Some(&self.background_color),
            "color" => Some(&self.color),
            "font-size" => Some(&self.font_size),
            "font-family" => Some(&self.font_family),
            "border-width" => Some(&self.border_width),
            "border-color" => Some(&self.border_color),
            "padding" => Some(&self.padding),
            "margin" => Some(&self.margin),
            "font-weight" => Some(&self.font_weight),
            "text-align" => Some(&self.text_align),
            "position" => Some(&self.position),
            "top" => Some(&self.top),
            "right" => Some(&self.right),
            "bottom" => Some(&self.bottom),
            "left" => Some(&self.left),
            "z-index" => Some(&self.z_index),
            "min-width" => Some(&self.min_width),
            "max-width" => Some(&self.max_width),
            "min-height" => Some(&self.min_height),
            "max-height" => Some(&self.max_height),
            "background" => Some(&self.background),
            "opacity" => Some(&self.opacity),
            "visibility" => Some(&self.visibility),
            "font-style" => Some(&self.font_style),
            "text-decoration" => Some(&self.text_decoration),
            "letter-spacing" => Some(&self.letter_spacing),
            "word-spacing" => Some(&self.word_spacing),
            "border-style" => Some(&self.border_style),
            "border" => Some(&self.border),
            "border-radius" => Some(&self.border_radius),
            "padding-top" => Some(&self.padding_top),
            "padding-right" => Some(&self.padding_right),
            "padding-bottom" => Some(&self.padding_bottom),
            "padding-left" => Some(&self.padding_left),
            "margin-top" => Some(&self.margin_top),
            "margin-right" => Some(&self.margin_right),
            "margin-bottom" => Some(&self.margin_bottom),
            "margin-left" => Some(&self.margin_left),
            "flex-direction" => Some(&self.flex_direction),
            "flex-wrap" => Some(&self.flex_wrap),
            "justify-content" => Some(&self.justify_content),
            "align-items" => Some(&self.align_items),
            "align-content" => Some(&self.align_content),
            "flex-grow" => Some(&self.flex_grow),
            "flex-shrink" => Some(&self.flex_shrink),
            "flex-basis" => Some(&self.flex_basis),
            "order" => Some(&self.order),
            "grid-template-columns" => Some(&self.grid_template_columns),
            "grid-template-rows" => Some(&self.grid_template_rows),
            "grid-gap" => Some(&self.grid_gap),
            "grid-column" => Some(&self.grid_column),
            "grid-row" => Some(&self.grid_row),
            "grid-area" => Some(&self.grid_area),
            "line-height" => Some(&self.line_height),
            "word-wrap" => Some(&self.word_wrap),
            "white-space" => Some(&self.white_space),
            "text-overflow" => Some(&self.text_overflow),
            "overflow" => Some(&self.overflow),
            "overflow-x" => Some(&self.overflow_x),
            "overflow-y" => Some(&self.overflow_y),
            "transform" => Some(&self.transform),
            "transform-origin" => Some(&self.transform_origin),
            "color-scheme" => Some(&self.color_scheme),
            "box-sizing" => Some(&self.box_sizing),
            "cursor" => Some(&self.cursor),
            "pointer-events" => Some(&self.pointer_events),
            "user-select" => Some(&self.user_select),
            "float" => Some(&self.float),
            "clear" => Some(&self.clear),
            "background-image" => Some(&self.background_image),
            "background-repeat" => Some(&self.background_repeat),
            "background-position" => Some(&self.background_position),
            "background-size" => Some(&self.background_size),
            "font-variant" => Some(&self.font_variant),
            "text-transform" => Some(&self.text_transform),
            "text-indent" => Some(&self.text_indent),
            "border-top" => Some(&self.border_top),
            "border-right" => Some(&self.border_right),
            "border-bottom" => Some(&self.border_bottom),
            "border-left" => Some(&self.border_left),
            "outline" => Some(&self.outline),
            "outline-width" => Some(&self.outline_width),
            "outline-color" => Some(&self.outline_color),
            "outline-style" => Some(&self.outline_style),
            "flex" => Some(&self.flex),
            "grid" => Some(&self.grid),
            "transition" => Some(&self.transition),
            "animation" => Some(&self.animation),
            "box-shadow" => Some(&self.box_shadow),
            "text-shadow" => Some(&self.text_shadow),
            _ => None,
        }
    }

    pub fn remove_property(&mut self, property: &str) {
        self.set_property(property, "");
    }

    pub fn clear(&mut self) {
        self.display.clear();
        self.width.clear();
        self.height.clear();
        self.background_color.clear();
        self.color.clear();
        self.font_size.clear();
        self.font_family.clear();
        self.border_width.clear();
        self.border_color.clear();
        self.padding.clear();
        self.margin.clear();
        self.font_weight.clear();
        self.text_align.clear();
        self.position.clear();
        self.top.clear();
        self.right.clear();
        self.bottom.clear();
        self.left.clear();
        self.z_index.clear();
        self.min_width.clear();
        self.max_width.clear();
        self.min_height.clear();
        self.max_height.clear();
        self.background.clear();
        self.opacity.clear();
        self.visibility.clear();
        self.font_style.clear();
        self.text_decoration.clear();
        self.letter_spacing.clear();
        self.word_spacing.clear();
        self.border_style.clear();
        self.border.clear();
        self.border_radius.clear();
        self.padding_top.clear();
        self.padding_right.clear();
        self.padding_bottom.clear();
        self.padding_left.clear();
        self.margin_top.clear();
        self.margin_right.clear();
        self.margin_bottom.clear();
        self.margin_left.clear();
        self.flex_direction.clear();
        self.flex_wrap.clear();
        self.justify_content.clear();
        self.align_items.clear();
        self.align_content.clear();
        self.flex_grow.clear();
        self.flex_shrink.clear();
        self.flex_basis.clear();
        self.order.clear();
        self.grid_template_columns.clear();
        self.grid_template_rows.clear();
        self.grid_gap.clear();
        self.grid_column.clear();
        self.grid_row.clear();
        self.grid_area.clear();
        self.line_height.clear();
        self.word_wrap.clear();
        self.white_space.clear();
        self.text_overflow.clear();
        self.overflow.clear();
        self.overflow_x.clear();
        self.overflow_y.clear();
        self.transform.clear();
        self.transform_origin.clear();
        self.color_scheme.clear();
        self.box_sizing.clear();
        self.cursor.clear();
        self.pointer_events.clear();
        self.user_select.clear();
        self.float.clear();
        self.clear.clear();
        self.background_image.clear();
        self.background_repeat.clear();
        self.background_position.clear();
        self.background_size.clear();
        self.font_variant.clear();
        self.text_transform.clear();
        self.text_indent.clear();
        self.border_top.clear();
        self.border_right.clear();
        self.border_bottom.clear();
        self.border_left.clear();
        self.outline.clear();
        self.outline_width.clear();
        self.outline_color.clear();
        self.outline_style.clear();
        self.flex.clear();
        self.grid.clear();
        self.transition.clear();
        self.animation.clear();
        self.box_shadow.clear();
        self.text_shadow.clear();
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
        let id = NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string();
        Self {
            id,
            node_type,
            children: Vec::new(),
            parent: None,
            text_content: String::new(),
            attributes: HashMap::new(),
            styles: StyleMap::default(),
            event_listeners: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, child_id: String, arena: &mut DOMArena) {
        self.children.push(child_id.clone());
        if let Some(child) = arena.get_node(&child_id) {
            child.lock().unwrap().parent = Some(self.id.clone());
        }
    }

    pub fn append_child(&mut self, child_id: String, arena: &mut DOMArena) {
        self.add_child(child_id, arena);
    }

    pub fn remove_child(&mut self, child_id: &str, arena: &mut DOMArena) -> Option<String> {
        if let Some(pos) = self.children.iter().position(|id| id == child_id) {
            self.children.remove(pos);
            if let Some(child) = arena.get_node(child_id) {
                child.lock().unwrap().parent = None;
            }
            Some(child_id.to_string())
        } else {
            None
        }
    }

    pub fn find_element_by_id<'a>(&'a self, id: &str, arena: &'a DOMArena) -> Option<Arc<Mutex<DOMNode>>> {
        if self.id == id {
            return arena.get_node(&self.id);
        }
        for child_id in &self.children {
            if let Some(child) = arena.get_node(child_id) {
                if let Some(found) = child.lock().unwrap().find_element_by_id(id, arena) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn find_element_by_class_ref_arena<'a>(&'a self, class: &str, arena: &'a DOMArena) -> Option<Arc<Mutex<DOMNode>>> {
        if let Some(class_attr) = self.attributes.get("class") {
            if class_attr.split_whitespace().any(|c| c == class) {
                return arena.get_node(&self.id);
            }
        }
        for child_id in &self.children {
            if let Some(child) = arena.get_node(child_id) {
                if let Some(found) = child.lock().unwrap().find_element_by_class_ref_arena(class, arena) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn find_element_by_tag_ref_arena<'a>(&'a self, tag: &str, arena: &'a DOMArena) -> Option<Arc<Mutex<DOMNode>>> {
        if let NodeType::Element(ref t) = self.node_type {
            if t == tag {
                return arena.get_node(&self.id);
            }
        }
        for child_id in &self.children {
            if let Some(child) = arena.get_node(child_id) {
                if let Some(found) = child.lock().unwrap().find_element_by_tag_ref_arena(tag, arena) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn find_elements_by_selector_arena<'a>(&'a self, selector: &str, results: &mut Vec<Arc<Mutex<DOMNode>>>, arena: &'a DOMArena) {
        // Example: only tag selector for now
        if let NodeType::Element(ref t) = self.node_type {
            if t == selector {
                if let Some(node) = arena.get_node(&self.id) {
                    results.push(node);
                }
            }
        }
        for child_id in &self.children {
            if let Some(child) = arena.get_node(child_id) {
                child.lock().unwrap().find_elements_by_selector_arena(selector, results, arena);
            }
        }
    }

    pub fn set_text_content(&mut self, text: String) {
        self.text_content = text;
    }

    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn set_style(&mut self, key: String, value: String) {
        self.styles.set_property(&key, &value);
    }

    /// Find an element by CSS selector (simplified implementation)
    pub fn query_selector(&self, selector: &str, arena: &DOMArena) -> Option<Arc<Mutex<DOMNode>>> {
        // Simple implementation for basic selectors
        if selector.starts_with('#') {
            // ID selector
            let id = &selector[1..];
            self.find_element_by_id(id, arena)
        } else if selector.starts_with('.') {
            // Class selector
            let class = &selector[1..];
            self.find_element_by_class_ref_arena(class, arena)
        } else {
            // Tag selector
            self.find_element_by_tag_ref_arena(selector, arena)
        }
    }

    /// Find all elements matching a CSS selector
    pub fn query_selector_all(&self, selector: &str, arena: &DOMArena) -> Vec<Arc<Mutex<DOMNode>>> {
        let mut results = Vec::new();
        self.find_elements_by_selector_arena(selector, &mut results, arena);
        results
    }

    /// Helper method to find element by ID (immutable reference)
    fn find_element_by_id_ref(&self, id: &str, arena: &DOMArena) -> Option<String> {
        if let Some(node_id) = self.attributes.get("id") {
            if node_id == id {
                return Some(self.id.clone());
            }
        }
        for child_id in &self.children {
            if let Some(child_node) = arena.get_node(child_id) {
                let child = child_node.lock().unwrap();
                if let Some(found_id) = child.find_element_by_id_ref(id, arena) {
                    return Some(found_id);
                }
            }
        }
        None
    }

    /// Get the ID of this element
    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    /// Set the ID of this element
    pub fn set_id(&mut self, id: String) {
        self.attributes.insert("id".to_string(), id);
    }

    /// Get the tag name of this element
    pub fn get_tag_name(&self) -> Option<&String> {
        if let NodeType::Element(tag_name) = &self.node_type {
            Some(tag_name)
        } else {
            None
        }
    }

    /// Create a new element node
    pub fn create_element(tag_name: &str) -> Self {
        Self::new(NodeType::Element(tag_name.to_string()))
    }

    /// Create a new text node
    pub fn create_text_node(text: &str) -> Self {
        let mut node = Self::new(NodeType::Text);
        node.text_content = text.to_string();
        node
    }
}

pub struct DOMArena {
    pub nodes: HashMap<String, Arc<Mutex<DOMNode>>>,
}

impl DOMArena {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, node: DOMNode) -> Arc<Mutex<DOMNode>> {
        let id = node.id.clone();
        let rc = Arc::new(Mutex::new(node));
        self.nodes.insert(id, rc.clone());
        rc
    }

    pub fn get_node(&self, id: &str) -> Option<Arc<Mutex<DOMNode>>> {
        self.nodes.get(id).cloned()
    }

    pub fn remove_node(&mut self, id: &str) -> Option<Arc<Mutex<DOMNode>>> {
        self.nodes.remove(id)
    }
}

// Deep clone utility for DOMNode
impl DOMNode {
    pub fn deep_clone(&self, arena: &mut DOMArena) -> DOMNode {
        let mut clone = self.clone();
        clone.id = NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string();
        clone.parent = None;
        clone.children = self.children.iter().filter_map(|cid| {
            if let Some(child) = arena.get_node(cid) {
                let child_clone = child.lock().unwrap().deep_clone(arena);
                let child_id = child_clone.id.clone();
                arena.add_node(child_clone);
                Some(child_id)
            } else {
                None
            }
        }).collect();
        clone.event_listeners = HashMap::new();
        clone
    }
} 
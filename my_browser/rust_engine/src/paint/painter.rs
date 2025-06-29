use crate::dom::node::LayoutBox;
use crate::paint::display_list::{DrawCommand, DisplayList};

pub struct Painter;

impl Painter {
    pub fn new() -> Self {
        Painter
    }

    // This will eventually walk the layout tree and emit draw commands
    pub fn paint(&self) {
        // TODO: Implement painting logic
    }

    pub fn from_layout_boxes(layout_boxes: &[LayoutBox]) -> DisplayList {
        let mut display_list = Vec::new();
        for b in layout_boxes {
            // Draw background rect if not transparent
            if b.background_color != "transparent" && !b.background_color.is_empty() {
                display_list.push(DrawCommand::Rect {
                    x: b.x,
                    y: b.y,
                    w: b.width,
                    h: b.height,
                    color: parse_color(&b.background_color),
                });
            }
            // Draw text if present
            if !b.text_content.is_empty() {
                display_list.push(DrawCommand::Text {
                    x: b.x,
                    y: b.y,
                    content: b.text_content.clone(),
                    font: b.font_family.clone(),
                    size: b.font_size,
                    color: parse_color(&b.color),
                });
            }
            // TODO: Add border, image, etc.
        }
        display_list
    }
}

fn parse_color(s: &str) -> u32 {
    // Very basic: expects #RRGGBB or #AARRGGBB
    if s.starts_with('#') {
        let hex = &s[1..];
        if hex.len() == 6 {
            // #RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            return (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        } else if hex.len() == 8 {
            // #AARRGGBB
            let a = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0xFF);
            let r = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[6..8], 16).unwrap_or(0);
            return ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }
    0xFF000000 // Default to opaque black
} 
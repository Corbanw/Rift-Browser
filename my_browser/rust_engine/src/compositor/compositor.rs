// compositor/compositor.rs

use crate::paint::display_list::{DrawCommand, DisplayList};

pub struct Compositor;

impl Compositor {
    pub fn new() -> Self {
        Compositor
    }

    // For now, just flatten the display list (no real compositing yet)
    pub fn composite(&self, display_list: DisplayList) -> DisplayList {
        // TODO: Implement real compositing (z-index, layers, etc.)
        display_list
    }
} 
#[derive(Debug, Clone)]
pub enum DrawCommand {
    Rect { x: f32, y: f32, w: f32, h: f32, color: u32 },
    Text { x: f32, y: f32, content: String, font: String, size: f32, color: u32 },
    Image { x: f32, y: f32, src: String },
}

pub type DisplayList = Vec<DrawCommand>; 
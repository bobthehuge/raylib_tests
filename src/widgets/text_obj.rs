use crate::*;

pub struct TextObj {
    text: String,
    x: i32,
    y: i32,
    size: i32,
    color: Color,
}

impl TextObj {
    pub fn new(text: String, x: i32, y: i32, size: i32, color: Color) -> TextObj{
        TextObj{
            text: text,
            x: x,
            y: y,
            size: size,
            color: color,
        }
    }

    pub fn text_width(&self) -> i32 {
        measure_text(&self.text, self.size)
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, offset: i32) {
        d.draw_text(
            &self.text,
            self.x + offset,
            self.y,
            self.size,
            self.color,
        )
    }
}

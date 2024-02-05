use crate::*;

pub struct TextObj {
    text: String,
    x: i32,
    y: i32,
    size: i32,
    color: Color,
    visible: bool,
}

impl TextObj {
    pub fn new(text: String, x: i32, y: i32, size: i32, color: Color) -> TextObj{
        TextObj{
            text: text,
            x: x,
            y: y,
            size: size,
            color: color,
            visible: true,
        }
    }

    pub fn text_width(&self) -> i32 {
        measure_text(&self.text, self.size)
    }
}

impl WidgetRender for TextObj {
    fn render(&self, d: &mut RaylibDrawHandle) -> RenderResult {
        if self.visible {
            d.draw_text(
                &self.text,
                self.x - self.text_width() / 2,
                self.y,
                self.size,
                self.color,
            )
        };

        RenderResult::None()
    }
}

impl WidgetVisibility for TextObj {
    #[inline]
    fn is_visible(&self) -> bool {
        self.visible
    }

    #[inline]
    fn show(&mut self) {
        self.visible = true;
    }

    #[inline]
    fn hide(&mut self) {
        self.visible = false;
    }

    #[inline]
    fn set_visibility(&mut self, state: bool) {
        self.visible = state;
    }
}

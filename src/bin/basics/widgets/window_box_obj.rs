use crate::*;

pub struct WindowBoxObj {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub header_size: f32,
    pub text: Option<CString>,
    pub callback: Box<dyn Fn(RenderResult, &mut ScopeType)>,
    visible: bool,
    moving: bool,
}

impl WindowBoxObj {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        text: Option<CString>
        Box<dyn Fn(RenderResult, &mut ScopeType)>) -> Self {
        WindowBoxObj{
            x,
            y,
            width,
            height,
            header_size: 24.0,
            text,
            callback,
            visible: true,
            moving: false,
        }
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle{
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn header_rect(&self) -> Rectangle {
        Rectangle{
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.header_size,
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) -> bool {
        if self.visible {
            d.gui_window_box(
                self.rect(),
                self.text.as_deref(),
            )
        } else {
            false
        }
    }
}

impl WidgetVisibility for WindowBoxObj {
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

impl WidgetMobility for WindowBoxObj {
    #[inline]
    fn is_moving(&self) -> bool {
        self.moving
    }

    #[inline]
    fn set_moving(&mut self, state: bool) {
        self.moving = state;
    }
}

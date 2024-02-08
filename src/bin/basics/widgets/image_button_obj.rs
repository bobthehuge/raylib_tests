use crate::*;

pub struct ImageButtonObj {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub tex: Texture2D,
    pub callback: Box<dyn Fn(RenderResult, &mut ScopeType)>,
    visible: bool,
    moving: bool,
}

impl ImageButtonObj {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        tex: Texture2D
        callback: Box<dyn Fn(RenderResult, &mut ScopeType)>) -> Self {
        ImageButtonObj {
            x,
            y,
            width,
            height,
            tex,
            callback,
            visible: true,
            moving: false,
        }
    }
    
    fn from_rect(bounds: Rectangle, tex: Texture2D) 
        -> ImageButtonObj {
        ImageButtonObj {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
            tex: tex,
            visible: false,
            moving: false,
        }
    }

    fn rect(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl WidgetRender for ImageButtonObj {
    fn render(&self, d: &mut RaylibDrawHandle) -> RenderResult {
        RenderResult::Bool(if self.visible {
            d.gui_image_button(
                self.rect(),
                None,
                &self.tex,
            )
        } else {
            false
        })
    }
}

impl WidgetVisibility for ImageButtonObj {
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

use crate::widgets::*;
use crate::widgets::button::*;
use raylib::prelude::*;

pub struct Block {
    rect: Rectangle,
    id: String,
    callback: fn(&Self, &WidgetResult),
    bt1: Button,
    bt2: Button,
    pub state1: bool,
    pub state2: bool,
}

impl Block {
    pub fn new(
        org: Vector2,
        id: String, 
        callback: fn(&Self, &WidgetResult)) -> Self {
        Self {
            rect: Rectangle::new(org.x, org.y, 72f32, 38f32),
            id: id.clone(),
            callback,
            bt1: Button::new(
                Rectangle::new(org.x+4.0, org.y+4.0, 30.0, 30.0),
                id.clone() + "_bt1",
                |obj, res| {
                    match res {
                        WidgetResult::Bool(pressed) => {
                            if *pressed {
                                println!("Pressed obj with id {}", obj.id)
                            }
                        }
                        _ => {}
                    }
                }
            ),
            bt2: Button::new(
                Rectangle::new(org.x+38.0, org.y+4.0, 30.0, 30.0),
                id + "_bt2",
                |obj, res| {
                    match res {
                        WidgetResult::Bool(pressed) => {
                            if *pressed {
                                println!("Pressed obj with id {}", obj.id)
                            }
                        }
                        _ => {}
                    }
                }
            ),
            state1: false,
            state2: false,
        }
    }
}

impl WidgetRectangle for Block {
    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }
}

impl WidgetCollidable for Block {
    fn check_rect_collision(&self, rect: Rectangle) -> bool {
        self.rect.check_collision_recs(&rect)
    }

    fn check_circ_collision(&self, center: Vector2, radius: f32) -> bool {
        self.rect.check_collision_circle_rec(center, radius)
    }
}

impl Widget for Block {
    fn render(
        &mut self,
        handle: &mut RaylibDrawHandle
    ) -> WidgetResult {
        handle.draw_rectangle_rec(self.rect, Color::BLUE);
    
        self.state1 = match self.bt1.call(handle) {
            WidgetResult::Bool(b) => b,
            _ => false,
        };

        self.state2 = match self.bt2.call(handle) {
            WidgetResult::Bool(b) => b,
            _ => false,
        };

        WidgetResult::Bool(self.state1 && self.state2)
    }

    fn call(&mut self, handle: &mut RaylibDrawHandle) -> WidgetResult {
        let res = self.render(handle);
        if self.is_ready() { (self.callback)(&self, &res); }
        res
    }

    fn ready(&mut self) { self.bt1.ready(); self.bt2.ready() }
    fn unready(&mut self) { self.bt1.unready(); self.bt2.unready() }
    fn is_ready(&self) -> bool { self.bt1.is_ready() && self.bt2.is_ready() }
    fn get_id(&self) -> &String { &self.id }
    fn set_id(&mut self, id: String) { self.id = id }

    fn as_widget_rectangle(&self) ->
        Option<&dyn WidgetRectangle> {
            Some(self as _)
        }

    fn as_widget_rectangle_mut(&mut self) ->
        Option<&mut dyn WidgetRectangle> {
            Some(self as _)
        }

    fn as_widget_collidable(&self) ->
        Option<&dyn WidgetCollidable> {
            Some(self as _)
        }

    fn as_widget_collidable_mut(&mut self) ->
        Option<&mut dyn WidgetCollidable> {
            Some(self as _)
        }
}

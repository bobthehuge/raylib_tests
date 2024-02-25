use crate::ieef64::*;
use crate::widgets::*;
use raylib::prelude::*;
use std::collections::HashMap;

pub struct Map {
    pub comps: HashMap<Vec2ieeF64, Box<dyn Widget>>,
    pub id: String,
    pub callback: fn(&Self, &WidgetResult),
}

impl Map {
    pub fn new(
        id: String,
        callback: fn(&Self, &WidgetResult)
    ) -> Self {
        Map {
            comps: HashMap::new(),
            id,
            callback,
        }
    }
}

impl Widget for Map {
    fn render(
        &mut self,
        handle: &mut RaylibDrawHandle
    ) -> WidgetResult {
        WidgetResult::Vec(
            self.comps
            .values_mut()
            .map(|x| x.call(handle))
            .collect::<Vec<WidgetResult>>()
        )
    }

    fn call(&mut self, handle: &mut RaylibDrawHandle) -> WidgetResult {
        let res = self.render(handle);
        if self.is_ready() { (self.callback)(&self, &res); }
        res
    }
    
    fn ready(&mut self) { self.comps.values_mut().for_each(|x| x.ready()) }
    fn unready(&mut self) { self.comps.values_mut().for_each(|x| x.unready()) }
    fn is_ready(&self) -> bool { self.comps.values().all(|x| x.is_ready()) }

    fn get_id(&self) -> &String { &self.id }
    fn set_id(&mut self, id: String) { self.id = id }
}

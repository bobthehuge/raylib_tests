use crate::*;

pub struct ButtonObj {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: Option<CString>,
    visible: bool,
    moving: bool,
}

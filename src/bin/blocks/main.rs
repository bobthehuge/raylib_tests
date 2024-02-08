use raylib::prelude::*;

use std::collections::HashMap;
use std::mem;
use std::ffi::CString;
use std::time::Duration;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

struct Clickable {
    pub rect: Rectangle,
    pub text: String,
}

impl Clickable {
    fn new(rect: Rectangle, text: String) -> Self {
        Clickable{
            rect,
            text,
        }
    }

    fn render(&self, handle: &mut RaylibDrawHandle) {
        let _ = handle.gui_button(
            self.rect,
            Some(&CString::new(&*self.text).expect("CString::new failed")),
        );
    }
}

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Hash, Eq, PartialEq)]
struct IEE_F64((u64, i16, i8));

impl IEE_F64 {
    fn new(val: f64) -> Self {
        IEE_F64(integer_decode(val))
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Vec2IEE_F64 {
    x: IEE_F64,
    y: IEE_F64,
}

impl Vec2IEE_F64 {
    fn new(x: f64, y: f64) -> Self {
        Vec2IEE_F64{x:IEE_F64::new(x), y:IEE_F64::new(y)}
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Raylib Widgets")
        .build();
    
    let mut objs_map: HashMap<Vec2IEE_F64, Clickable> = HashMap::new();
    let mut count = 0i32;
    let mut objs_add_ready = true;


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // helpful to compute mouse delta and get collisions
        let new_mouse_pos = d.get_mouse_position();
        let new_mouse_box = Rectangle{
            x: new_mouse_pos.x,
            y: new_mouse_pos.y,
            width: 1.0,
            height: 1.0,
        };
        
        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) && objs_add_ready {
            objs_add_ready = false;
            count += 1;
            objs_map.insert(
                Vec2IEE_F64::new(new_mouse_pos.x as f64, new_mouse_pos.y as f64),
                Clickable::new(
                    Rectangle::new(
                        new_mouse_pos.x, 
                        new_mouse_pos.y,
                        50.0,
                        24.0,
                    ), 
                    format!("TEST_{}", count))
            );

            println!("added object n°{}", count);
        }

        if d.is_mouse_button_up(MouseButton::MOUSE_LEFT_BUTTON) {
            objs_add_ready = true;
        }

        if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && 
            d.is_key_pressed(KeyboardKey::KEY_R) && 
                objs_add_ready{
            count = 0;
            objs_map.clear();
        }

        d.clear_background(Color::WHITE);

        objs_map.iter().for_each(|(_,x)| x.render(&mut d));
    }

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

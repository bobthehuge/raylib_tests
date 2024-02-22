pub mod widgets;
pub mod editor;

use raylib::prelude::*;

use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::time::Duration;

use crate::editor::*;
use crate::widgets::*;
use crate::widgets::clickable::*;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const SELECTION_BOX_SIZE: f32 = 5.0;
const BG_COLOR: Color = Color::WHITE;

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
struct IeeF64((u64, i16, i8));

impl IeeF64 {
    fn new(val: f64) -> Self {
        IeeF64(integer_decode(val))
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Vec2ieeF64 {
    x: IeeF64,
    y: IeeF64,
}

impl Vec2ieeF64 {
    fn new(x: f64, y: f64) -> Self {
        Vec2ieeF64 {x: IeeF64::new(x), y: IeeF64::new(y)}
    }
}

fn main() {
    let (mut rl, thread) = init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Raylib Widgets")
        .build();

    let mut editor = Editor::new();
    // let mut objs_map: HashMap<Vec2ieeF64, Clickable> = HashMap::new();
    let mut objs_map = Widget::new(|_,_| WidgetResult::Bool(true));
    let mut count = 0i32;
    let mut objs_add_ready = true;

    let mut selection_box = Rectangle::default();
    let _ = rl.begin_drawing(&thread);

    editor.set_mode(EditorMode::Edit);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // helpful to compute mouse delta and get collisions
        let new_mouse_pos = d.get_mouse_position();
        let _new_mouse_box = Rectangle{
            x: new_mouse_pos.x,
            y: new_mouse_pos.y,
            width: 1.0,
            height: 1.0,
        };
        
        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let collision = objs_map.comps.iter()
                .find(|(_, x)|
                    x.rect.check_collision_circle_rec(
                        Vector2{
                            x: new_mouse_pos.x,
                            y: new_mouse_pos.y,
                        },
                        SELECTION_BOX_SIZE * 2.0,
                    )
                );

            match collision {
                Some((_, v)) => {
                    selection_box = Rectangle::new(
                        v.rect.x - SELECTION_BOX_SIZE,
                        v.rect.y - SELECTION_BOX_SIZE,
                        v.rect.width + SELECTION_BOX_SIZE * 2.0,
                        v.rect.height + SELECTION_BOX_SIZE * 2.0,
                    );
                },
                _ => {
                    editor.lock_mode();

                    objs_add_ready = false;
                    selection_box = Rectangle::new(
                        new_mouse_pos.x - SELECTION_BOX_SIZE,
                        new_mouse_pos.y - SELECTION_BOX_SIZE,
                        50.0 + SELECTION_BOX_SIZE * 2.0,
                        24.0 + SELECTION_BOX_SIZE * 2.0,
                    );
                    count += 1;
                    objs_map.comps.insert(
                        Vec2ieeF64::new(new_mouse_pos.x as f64, new_mouse_pos.y as f64),
                        &Clickable::new(
                            Rectangle::new(
                                new_mouse_pos.x,
                                new_mouse_pos.y,
                                50.0,
                                50.0,
                            ),
                            format!("TEST_{}", count),
                            |obj, res| { 
                                match res {
                                    WidgetResult::Bool(pressed) => {
                                        if pressed {
                                            println!("Pressed button {}", obj.text)
                                        }
                                    }
                                    _ => {}
                                }

                                WidgetResult::Bool(true)
                            },
                        )
                    );
                    println!("added object n°{}", count);
                }
            }
        }


        if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && 
            d.is_key_pressed(KeyboardKey::KEY_R) && objs_add_ready {

            count = 0;
            objs_map.comps.clear();
            selection_box = Rectangle::default();
        }

//////////////////////////////// DRAWING PHASE ////////////////////////////////

        d.clear_background(BG_COLOR);

        if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            let x = selection_box.x + SELECTION_BOX_SIZE;
            let y = selection_box.y + SELECTION_BOX_SIZE;

            let target = objs_map.comps.remove(&Vec2ieeF64::new(x as f64, y as f64));

            match target {
                Some(_) => {
                    println!("Removed object '{}'", target.unwrap().text);
                    // let _ = d.draw_rectangle_rec(selection_box, BG_COLOR);
                    count -= 1;
                    selection_box = Rectangle::default();
                }
                None => {}
            }
        }

        let _ = objs_map.comps.iter_mut().for_each(|(_,x)| { x.render(&mut d); });

        let _ = d.draw_rectangle_lines_ex(selection_box, 2, Color::RED);

///////////////////////////////////////////////////////////////////////////////

        if d.is_mouse_button_up(MouseButton::MOUSE_LEFT_BUTTON) {
            objs_add_ready = true;
            objs_map.comps.iter().for_each(|(_,x)| { *x.ready() });
        }
    }

    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

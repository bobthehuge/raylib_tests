use crate::ieef64::*;
use crate::widgets::*;
use crate::widgets::block::*;
use crate::widgets::button::*;
use crate::widgets::map::*;

use raylib::prelude::*;
// use std::collections::HashMap;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const SELECTION_BOX_SIZE: f32 = 5.0;
const BG_COLOR: Color = Color::WHITE;

fn insert_new_button(objs_map: &mut Map, mouse_pos: Vector2) {
    objs_map.comps.insert(
        Vec2ieeF64::from_vec2(mouse_pos),
        Box::new(Button::new(
            Rectangle::new(
                mouse_pos.x,
                mouse_pos.y,
                50.0,
                50.0,
            ),
            format!("BUTTON_{}", objs_map.comps.len()+1),
            |obj, res| { 
                match res {
                    WidgetResult::Bool(pressed) => {
                        if *pressed {
                            println!("Pressed button '{}'", obj.id)
                        }
                    }
                    _ => {}
                }
            },
        ))
    );

    println!("Added object n°{}", objs_map.comps.len());
}

fn insert_new_block(objs_map: &mut Map, mouse_pos: Vector2) {
    objs_map.comps.insert(
        Vec2ieeF64::from_vec2(mouse_pos),
        Box::new(Block::new(
            mouse_pos,
            format!("BLOCK_{}", objs_map.comps.len()+1),
            |_,_| { }
        ))
    );

    println!("Added object n°{}", objs_map.comps.len());
}

pub fn main() {
    let (mut rl, thread) = init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Raylib Widgets")
        .build();

    rl.set_target_fps(60);

    let mut objs_map = Map::new(
        "block1".to_owned(), 
        |_,_| {}
    );

    let mut next_widget_type = WidgetType::None;
    let mut objs_add_ready = true;

    let mut selection_box = Rectangle::EMPTY;
    let _ = rl.begin_drawing(&thread);

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
            let target = objs_map.comps
                .iter()
                .filter_map(|(_, x)| x.as_widget_collidable())
                .find(|x|
                    x.check_circ_collision(
                        new_mouse_pos,
                        SELECTION_BOX_SIZE * 2.0,
                    )
                );

            match target {
                Some(v) => {
                    selection_box = Rectangle::new(
                        v.get_rect().x - SELECTION_BOX_SIZE,
                        v.get_rect().y - SELECTION_BOX_SIZE,
                        v.get_rect().width + SELECTION_BOX_SIZE * 2.0,
                        v.get_rect().height + SELECTION_BOX_SIZE * 2.0,
                    );

                    next_widget_type = WidgetType::None;
                },
                _ => {
                    objs_add_ready = false;
                    selection_box = Rectangle::new(
                        new_mouse_pos.x - SELECTION_BOX_SIZE,
                        new_mouse_pos.y - SELECTION_BOX_SIZE,
                        50.0 + SELECTION_BOX_SIZE * 2.0,
                        24.0 + SELECTION_BOX_SIZE * 2.0,
                    );

                    match next_widget_type {
                        WidgetType::Block => {
                            insert_new_block(&mut objs_map, new_mouse_pos);
                        },
                        WidgetType::Button => {
                            insert_new_button(&mut objs_map, new_mouse_pos);
                        },
                        WidgetType::None => {
                            selection_box = Rectangle::EMPTY;
                        },
                    }
                }
            }
        }


        if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && 
            d.is_key_pressed(KeyboardKey::KEY_R) && objs_add_ready {

            objs_map.comps.clear();
            selection_box = Rectangle::EMPTY;
        }

        if d.is_key_pressed(KeyboardKey::KEY_ZERO) && objs_add_ready {
            next_widget_type = WidgetType::None;
        }
        if d.is_key_pressed(KeyboardKey::KEY_ONE) && objs_add_ready {
            next_widget_type = WidgetType::Button;
        }
        if d.is_key_pressed(KeyboardKey::KEY_TWO) && objs_add_ready {
            next_widget_type = WidgetType::Block;
        }

//////////////////////////////// DRAWING PHASE ////////////////////////////////

        d.clear_background(BG_COLOR);

        if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            let x = selection_box.x + SELECTION_BOX_SIZE;
            let y = selection_box.y + SELECTION_BOX_SIZE;

            let target = objs_map.comps.remove(&Vec2ieeF64::new(x, y));

            match target {
                Some(b) => {
                    println!("Removed object '{}'", b.get_id());
                    selection_box = Rectangle::EMPTY;
                },
                _ => {}
            }
        }

        let _ = objs_map.call(&mut d);

        if selection_box != Rectangle::EMPTY {
            d.draw_rectangle_lines_ex(selection_box, 2, Color::RED);
        }

///////////////////////////////////////////////////////////////////////////////

        if d.is_mouse_button_up(MouseButton::MOUSE_LEFT_BUTTON) {
            objs_add_ready = true;
            objs_map.ready();
        }
    }
}

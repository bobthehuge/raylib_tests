mod chunks;
mod widgets;
mod Scope;

use raylib::prelude::*;
use raylib::ffi::KeyboardKey;
use raylib::core::text::measure_text;
use raylib::core::texture::Image;
use raylib::math::{
    Rectangle,
    Vector2,
    Vector3,
};

use std::ffi::{ CString };
use std::time::Duration;

use chunks::Map;

use widgets::{
    RenderResult,
    WidgetMobility,
    WidgetVisibility,
    WidgetRender,
    window_box_obj::WindowBoxObj,
    button_obj::ButtonObj,
    text_obj::TextObj,
    image_button_obj::ImageButtonObj,
};

use crate::widgets::Widget;
use crate::Scope::ScopeType;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
const TOOLBAR_HEIGHT: i32 = 40;

const TOOLBAR_PAN_RECT: Rectangle = Rectangle{
    x: 0.0, 
    y: 0.0, 
    height: TOOLBAR_HEIGHT as f32,
    width: WINDOW_WIDTH as f32, 
};

const EXIT_ID: usize = 0;
const PREVIEW_ID: usize = 1;
const TEST_ID: usize = 2;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .undecorated()
        .title("Bob's Raylib Tests")
        .build();

    rl.set_exit_key(None);

    let mut preview_window_obj = WindowBoxObj::new(
        0.0,
        TOOLBAR_HEIGHT as f32,
        1002.0,
        1026.0,
        Some(CString::new("Preview").expect("CString::new failed")),
    );
    preview_window_obj.hide();

    let mut test_button_obj =  ButtonObj::new(
        (WINDOW_WIDTH / 2 - 50) as f32,
        (WINDOW_HEIGHT / 2 + 32) as f32,
        100.0,
        24.0,
        Some(CString::new("TEST.").expect("CString::new failed")),
        Box::new(|_result: RenderResult, _scope: &mut ScopeType| {
            let RenderResult::Bool(res) = _result else { unreachable!() };
            let ScopeType::Vector(scope) = _scope else { unreachable!() };

            match &mut scope[0] {
                Widget::WindowBox(wb) => {
                    if res { wb.show() }
                }
                _ => unreachable!()
            }
        }),
    );

    let mut greet_text_obj = TextObj::new(
        String::from("votai."),
        WINDOW_WIDTH / 2,
        WINDOW_HEIGHT / 2,
        20,
        Color::BLACK,
    );

    let map = Map::new(Vector3::new(1000.0, 1000.0, 10.0), 10.0);

    let mut exit_imbutton_obj = ImageButtonObj::new(
        (WINDOW_WIDTH - 32) as f32, 
        8.0, 
        24.0, 
        24.0,
        rl.load_texture_from_image(&thread, 
            &Image::load_image("assets/EXIT_BUTTON.png")
            .unwrap()
        ).unwrap(),
    );
    // exit_imbutton_obj.show();

    let map_tex: Texture2D = rl.load_texture_from_image(
        &thread,
        &map.render_to_image(),
    ).unwrap();

    let mut old_mouse_pos = rl.get_mouse_position();

    let global_scope_vec= vec![
        Widget::ImageButton(exit_imbutton_obj),
        Widget::WindowBox(preview_window_obj),
        Widget::Button(test_button_obj),
    ];

    let mut global_scope = ScopeType::Vector(global_scope_vec);

    let Widget::ImageButton(exit_imbutton_obj) =
        global_scope.get(EXIT_ID) else {unreachable!()};

    let Widget::WindowBox(preview_window_obj) =
        global_scope.get(PREVIEW_ID) else {unreachable!()};

    let Widget::Button(test_button_obj) =
        global_scope.get(TEST_ID) else {unreachable!()};

    'mainloop: while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // helpful to compute mouse delta and get collisions
        let new_mouse_pos = d.get_mouse_position();
        let new_mouse_collision_box = Rectangle{
            x: new_mouse_pos.x,
            y: new_mouse_pos.y,
            width: 1.0,
            height: 1.0,
        };

        d.clear_background(Color::WHITE);

        d.gui_panel(
            TOOLBAR_PAN_RECT
        );

        match exit_imbutton_obj.render(&mut d) {
            RenderResult::Bool(b) => if b { break 'mainloop; },
            _ => {}
        }

        match test_button_obj.render(&mut d) {
            RenderResult::Bool(b) => if b { preview_window_obj.show(); },
            _ => {}
        }

        let _ = greet_text_obj.render(&mut d);

        if preview_window_obj.is_visible() {
            // closing mechanic
            if preview_window_obj.render(&mut d) || 
                d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                preview_window_obj.hide();
            }
            
            // set moving mode
            if preview_window_obj.header_rect()
                .check_collision_recs(&new_mouse_collision_box){
                    if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON){
                        preview_window_obj.set_moving(true);
                }
            }

            // moving mechanic
            if preview_window_obj.is_moving() {
                let delta = old_mouse_pos - new_mouse_pos;

                preview_window_obj.x = (preview_window_obj.x - delta.x)
                    .clamp(-preview_window_obj.width + 50.0, 
                        WINDOW_WIDTH as f32 - 50.0);

                preview_window_obj.y = (preview_window_obj.y - delta.y)
                    .clamp(0.0, WINDOW_HEIGHT as f32 - 50.0);

                if d.is_mouse_button_up(MouseButton::MOUSE_LEFT_BUTTON){
                    preview_window_obj.set_moving(false);
                }
            }

            // draw map inside preview window
            d.draw_texture(
                &map_tex,
                preview_window_obj.x as i32 + 1, 
                (preview_window_obj.y + preview_window_obj.header_size) as i32, 
                Color::WHITE
            );
        }

        // refresh mouse position
        // MUST STAY AT THE END OF THE LOOP
        old_mouse_pos = new_mouse_pos;

        // cap framerate at 60fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

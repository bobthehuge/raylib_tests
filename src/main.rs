mod chunks;
mod widgets;

use raylib::prelude::*;
use raylib::ffi::KeyboardKey;
use raylib::core::text::measure_text;
use raylib::core::texture::Image;
use raylib::math::{
    Rectangle,
    Vector2,
    Vector3,
};

use std::ffi::{CString};
use std::time::Duration;

use chunks::Map;

use widgets::{
    WidgetMobility,
    WidgetVisibility,
    window_box_obj::WindowBoxObj,
    button_obj::ButtonObj,
    text_obj::TextObj,
};

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
const TOOLBAR_HEIGHT: i32 = 40;

const EXIT_BUTTON_RECT: Rectangle = Rectangle{
    x: (WINDOW_WIDTH - 32) as f32, 
    y: 8.0, 
    height: 24.0, 
    width: 24.0,
};

const TOOLBAR_PAN_RECT: Rectangle = Rectangle{
    x: 0.0, 
    y: 0.0, 
    height: TOOLBAR_HEIGHT as f32,
    width: WINDOW_WIDTH as f32, 
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .undecorated()
        .title("Bob's Raylib Tests")
        .build();

    rl.set_exit_key(None);

    let test_button_obj =  ButtonObj::new(
        (WINDOW_WIDTH / 2 - 50) as f32, 
        (WINDOW_HEIGHT / 2 + 32) as f32, 
        100.0,
        24.0, 
        Some(CString::new("TEST.").expect("CString::new failed")),
    );

    let mut preview_window_obj = WindowBoxObj::new(
        0.0,
        TOOLBAR_HEIGHT as f32,
        1002.0,
        1026.0,
        Some(CString::new("Preview").expect("CString::new failed")),
    );

    let greet_text_obj = TextObj::new(
        String::from("votai."),
        WINDOW_WIDTH / 2,
        WINDOW_HEIGHT / 2,
        20,
        Color::BLACK,
    );

    let map = Map::new(Vector3::new(1000.0, 1000.0, 10.0), 10.0);

    let exit_button_image = Image::load_image("assets/EXIT_BUTTON.png")
        .unwrap();

    let exit_button_tex = rl
        .load_texture_from_image(&thread, &exit_button_image)
        .unwrap();

    let map_tex: Texture2D = rl.load_texture_from_image(
        &thread,
        &map.render_to_image(),
    ).unwrap();

    let mut old_mouse_pos = rl.get_mouse_position();

    'mainloop: while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // helpful to compute mouse delta and get collision with widgets
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

        if d.gui_image_button(
                EXIT_BUTTON_RECT,
                None,
                &exit_button_tex
            ){
            break 'mainloop;
        }

        if test_button_obj.render(&mut d) {
            preview_window_obj.show();
        }

        greet_text_obj.render(&mut d, -greet_text_obj.text_width() / 2);

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

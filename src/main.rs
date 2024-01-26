mod chunks;

use raylib::prelude::*;
use raylib::ffi;
use raylib::ffi::KeyboardKey;
use raylib::core::text::measure_text;
use raylib::core::texture::Image;

use std::ffi::{CString};
use std::time::Duration;
use std::process::exit;

use chunks::Map;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
const TOOLBAR_HEIGHT: i32 = 40;

const EXIT_BUTTON_RECT: ffi::Rectangle = ffi::Rectangle{
    x: (WINDOW_WIDTH - 32) as f32, 
    y: 8.0, 
    height: 24.0, 
    width: 24.0,
};

const TEST_BUTTON_RECT: ffi::Rectangle = ffi::Rectangle{
    x: (WINDOW_WIDTH / 2 - 50) as f32, 
    y: (WINDOW_HEIGHT / 2 + 32) as f32, 
    height: 24.0, 
    width: 100.0,
};

const TOOLBAR_PAN_RECT: ffi::Rectangle = ffi::Rectangle{
    x: 0.0, 
    y: 0.0, 
    height: TOOLBAR_HEIGHT as f32,
    width: WINDOW_WIDTH as f32, 
};

fn exit_gui(d: &RaylibDrawHandle<'_>, code: i32) -> () {
    drop(d);
    unsafe { ffi::CloseWindow(); }
    exit(code);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .undecorated()
        .title("Bob's Raylib Tests")
        .build();

    rl.set_exit_key(None);

    let exit_button_image = Image::load_image("assets/EXIT_BUTTON.png")
        .unwrap();

    let exit_button_tex = rl
        .load_texture_from_image(&thread, &exit_button_image)
        .unwrap();

    let map = Map::new(1000.0, 1000.0, 10.0);

    let map_tex: Texture2D = rl.load_texture_from_image(
        &thread,
        &map.render_to_image(),
    ).unwrap();

    let mut state = false;
    let greet_text = "votai.";
    let greet_font_size: i32 = 20;
    let greet_size = measure_text(greet_text, greet_font_size);

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.gui_panel(
            TOOLBAR_PAN_RECT
        );

        if d.gui_image_button(
                EXIT_BUTTON_RECT,
                None,
                &exit_button_tex
            ){
            exit_gui(&d, 0);
        }

        d.gui_unlock();

        if d.gui_button(
                TEST_BUTTON_RECT,
                Some(&CString::new("TEST.").expect("CString::new failed")),
            ){
            state = true;
        }

        d.draw_text(
            greet_text,
            WINDOW_WIDTH / 2 - greet_size / 2,
            WINDOW_HEIGHT / 2 - 10,
            greet_font_size,
            Color::BLACK,
        );

        if state {
            if d.gui_window_box(
                Rectangle{
                    x: 0.0,
                    y: TOOLBAR_HEIGHT as f32,
                    width: 1002.0,
                    height: 1026.0,
                },
                Some(&CString::new("Preview").expect("CString::new failed")),
            ) || d.is_key_pressed(KeyboardKey::KEY_ESCAPE){
                state = false;
            }
            
            d.draw_texture(&map_tex, 1, TOOLBAR_HEIGHT + 24, Color::WHITE);
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

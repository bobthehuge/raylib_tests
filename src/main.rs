use raylib::prelude::*;
use raylib::ffi;
use raylib::core::text::measure_text;

use std::ffi::{CString};
use std::time::Duration;
use std::process::exit;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;

const EXIT_RECT: ffi::Rectangle = ffi::Rectangle{
    x: (WINDOW_WIDTH - 32) as f32, 
    y: 8.0, 
    height: 24.0, 
    width: 24.0,
};

const TOOLBAR_PAN_RECT: ffi::Rectangle = ffi::Rectangle{
    x: 0.0, 
    y: 0.0, 
    height: 40.0,
    width: WINDOW_WIDTH as f32, 
};

fn exit_myraygui(d: &RaylibDrawHandle<'_>, code: i32) -> () {
    drop(d);
    unsafe { ffi::CloseWindow(); }
    exit(code);
}

fn main() {
    println!("Hello, world!");

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .undecorated()
        .title("Bob's Raylib Tests")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GRAY);

        d.gui_panel(
            TOOLBAR_PAN_RECT
        );

        if d.gui_button(
                EXIT_RECT,
                Some(&CString::new("").expect("CString::new failed"))
            ){
            exit_myraygui(&d, 0);
        }


        let greet_text = "votai. ";
        let greet_font_size: i32 = 20;
        let greet_size = measure_text(greet_text, greet_font_size);
        d.draw_text(greet_text, 
            WINDOW_WIDTH / 2 - greet_size / 2, 
            WINDOW_HEIGHT / 2, 
            greet_font_size, 
            Color::BLACK
        );

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

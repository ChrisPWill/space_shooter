
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

pub mod types;
pub mod entity;

use std::cell::RefCell;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use sdl2_window::Sdl2Window;

fn main() {
    let (win_width, win_height) = (1280, 720);
    let mut window = Sdl2Window::new(
        OpenGL::_3_2,
        piston::window::WindowSettings {
            title: "Space Game".to_string(),
            size: [win_width, win_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    let window = RefCell::new(window);

    for e in piston::events(&window) {
    }
}

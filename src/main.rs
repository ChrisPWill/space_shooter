
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate sprite;
extern crate vecmath;
extern crate uuid;

pub mod entity;

use std::cell::RefCell;
use std::rc::Rc;
use opengl_graphics::{
    OpenGL,
};
use sdl2_window::Sdl2Window;

fn main() {
    let win_size = piston::window::Size{width: 1280, height: 720};
    let window_settings = piston::window::WindowSettings::new(
        "Space Game".to_string(),
        win_size
        ).fullscreen(false).exit_on_esc(true).samples(0);

    let window = Sdl2Window::new(
        OpenGL::_3_2,
        window_settings,
    );

    let window = Rc::new(RefCell::new(window));

    for _ in piston::events(window) {
    }
}


extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate sprite;
extern crate vecmath;
extern crate uuid;

pub mod entity;

use entity::{Drawable};

use std::cell::RefCell;
use std::rc::Rc;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
};
use sdl2_window::Sdl2Window;

fn main() {
    let win_size = piston::window::Size{width: 1280, height: 720};
    let window_settings = piston::window::WindowSettings::new(
        "Space Game".to_string(),
        win_size
        ).fullscreen(false).exit_on_esc(true).samples(0);
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        window_settings,
    );

    let ref mut gl = GlGraphics::new(opengl);
    let window = Rc::new(RefCell::new(window));
    let mut scene = sprite::Scene::new();
    let player = entity::Player::new(&mut scene);

    for e in piston::events(window) {
        use piston::event::*;

        scene.event(&e);

        if let Some(args) = e.render_args() {
            use graphics::*;
            player.draw(&mut scene, args);

            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
                scene.draw(c.transform, gl);
            });
        }
    }
}

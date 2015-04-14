
#![feature(slice_patterns)]

extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate sprite;
extern crate vecmath;
extern crate uuid;

pub mod entity;

use entity::{Drawable, MobileUnit};

use std::cell::RefCell;
use std::rc::Rc;

use piston::event::*;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
};
use sdl2_window::Sdl2Window;
use piston::input::Button;
use piston::input::keyboard::Key;

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

    let mut player = entity::Player::new(&mut scene);

    // left, up, right, down
    let mut moving = [false; 4];

    for e in window.events() {
        use piston::event::*;

        scene.event(&e);
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left  => {moving[0] = true;},
                Key::Up    => {moving[1] = true;},
                Key::Right => {moving[2] = true;},
                Key::Down  => {moving[3] = true;},
                _ => {},
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Left  => {moving[0] = false;},
                Key::Up    => {moving[1] = false;},
                Key::Right => {moving[2] = false;},
                Key::Down  => {moving[3] = false;},
                _ => {},

            }
        }
        if let Some(args) = e.update_args() {
            player.update(args.dt);
            // update horizontal movement
            match moving {
                [true , _, false, _] => {player.set_vel_x(-500.0);},
                [false, _, true , _] => {player.set_vel_x(500.0);},
                [_    , _, _    , _] => {player.set_vel_x(0.0);},
            }
            // update vertical movement
            match moving {
                [_, true , _, false] => {player.set_vel_y(500.0);},
                [_, false, _, true ] => {player.set_vel_y(-500.0);},
                [_, _    , _, _    ] => {player.set_vel_y(0.0);},
            }
        }
        if let Some(args) = e.render_args() {
            use graphics::*;
            player.draw(&mut scene, args);

            gl.draw(args.viewport(), |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
                scene.draw(c.transform, gl);
            });
        }
    }
}

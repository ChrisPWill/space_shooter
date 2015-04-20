use std::path::Path;
use std::rc::Rc;

use vecmath;
use sprite;

use piston::event;
use sprite::Sprite;
use opengl_graphics::Texture;
use uuid::Uuid;

/// A trait that provides a draw function that can be called from anything that
/// needs to be drawn.
pub trait Drawable {
    /// Draws a type to the scene. `render_args` provides the current drawable
    /// area while `scene` is the scene containing any sprites or textures that
    /// are the type has loaded and will draw to the renderer.
    fn draw(&self, scene: &mut sprite::Scene<Texture>, render_args: event::RenderArgs);
}

/// A trait that provides basic functions for updating position on a type.
pub trait MobileUnit {
    /// Update the position of the type based on the current velocity.
    ///
    /// `time_delta` provides the time elapsed since the last position update,
    /// allowing position to be updated in a manner independent from framerate.
    fn update_pos(&mut self, time_delta: f64);

    /// Updates the velocity in the x axis.
    /// 
    /// `velX` is in units/second
    fn set_vel_x(&mut self, velX: f64);
    /// Updates the velocity in the y axis.
    /// 
    /// `velY` is in units/second
    fn set_vel_y(&mut self, velY: f64);
}

/// The `Player` struct. Handles data and functionality that only the player
/// should have.
pub struct Player {
    /// The location of the `Player` relative to the origin (the centre-bottom)
    loc: vecmath::Vector2<f64>,
    /// The velocity of the `Player`
    vel: vecmath::Vector2<f64>,
    /// The sprite ID assigned to the sprite used for the current `Player` ship
    sprite: Uuid,
}

impl Player {
    /// Generates a new `Player`.
    ///
    /// This loads the player sprite and sets the player position to a reasonable
    /// default (the origin, centre-bottom of screen)
    pub fn new(scene: &mut sprite::Scene<Texture>) -> Player {
        // load sprite
        let path = Path::new("./assets/playerShip1_red.png");
        let tex  = Rc::new(
            match Texture::from_path(&path) {
                Ok(tex) => tex,
                Err(_)  => { panic!("couldn't load texture from path"); },
            }
            );
        let sprite = Sprite::from_texture(tex.clone());

        // add sprite to scene
        let id = scene.add_child(sprite);

        Player{loc: [0.0, 0.0], vel: [0.0, 0.0], sprite: id}
    }

    /// Updates the `Player` based on a provided time delta since the last update.
    ///
    /// `time_delta` is in seconds
    pub fn update(&mut self, time_delta: f64) {
        self.update_pos(time_delta);
    }

    /// Directly updates the position of the `Player`
    fn set_position(&mut self, x: f64, y: f64) {
        self.loc = [x, y];
    }
}

impl MobileUnit for Player {
    /// Updates the position of a `Player` based on its current velocity.
    ///
    /// This the takes `time_delta` (in seconds) since the last position update
    /// and updates the position avvording to currently stored velocity.
    fn update_pos(&mut self, time_delta: f64) {
        self.loc = vecmath::vec2_add(self.loc.clone(), 
            vecmath::vec2_scale(self.vel.clone(), time_delta));

        // Ensure ship stays within boundaries
        if self.loc[0] >  590.0 { let y = self.loc[1]; self.set_position( 590.0,     y); }
        if self.loc[0] < -590.0 { let y = self.loc[1]; self.set_position(-590.0,     y); }
        if self.loc[1] >  640.0 { let x = self.loc[0]; self.set_position(     x, 640.0); }
        if self.loc[1] <   40.0 { let x = self.loc[0]; self.set_position(     x,  40.0); }
    }

    /// Sets the velocity of a `Player` on the x axis.
    fn set_vel_x(&mut self, vel_x: f64) {
        self.vel[0] = vel_x;
    }

    /// Sets the velocity of a `Player` on the y axis.
    fn set_vel_y(&mut self, vel_y: f64) {
        self.vel[1] = vel_y;
    }
}

impl Drawable for Player {
    /// Draws the `Player` to the screen.
    fn draw(&self, scene: &mut sprite::Scene<Texture>, render_args: event::RenderArgs) {
        let mut sprite = scene.child_mut(self.sprite)
            .expect("couldn't retrieve sprite asset");

        let width_mult  = render_args.width as f64/1280.0;
        let height_mult = render_args.height as f64/720.0;
        sprite.set_position(render_args.width as f64/2.0 + (self.loc[0]*width_mult), 
                            render_args.height as f64 - (self.loc[1]*height_mult));
    }

}

pub struct Projectile {
    loc: vecmath::Vector2<f64>,
    vel: vecmath::Vector2<f64>,
    sprite: Uuid,
}

impl Projectile {
    /// Generates a new `Projectile`
    ///
    /// Takes a previously loaded `sprite_id` to be used as its sprite, in
    /// addition to an initial location (`loc`) and an initial y velocity (
    /// `y_vel`)
    pub fn new(sprite_id: Uuid, loc: vecmath::Vector2<f64>, y_vel: f64) 
        -> Projectile {
        Projectile{loc: loc, vel: [0.0, y_vel], sprite: sprite_id}
    }
    /// Updates the projectile based on the `time_delta` since the previous
    /// update. 
    ///
    /// `time_delta` is in seconds.
    pub fn update(&mut self, time_delta: f64) {
        self.update_pos(time_delta);
    }
}

impl MobileUnit for Projectile {
    fn update_pos(&mut self, time_delta: f64) {
        self.loc = vecmath::vec2_add(self.loc.clone(), 
            vecmath::vec2_scale(self.vel.clone(), time_delta));
    }

    fn set_vel_x(&mut self, vel_x: f64) {
        self.vel[0] = vel_x;
    }

    fn set_vel_y(&mut self, vel_y: f64) {
        self.vel[1] = vel_y;
    }
}

impl Drawable for Projectile {
    fn draw(&self, scene: &mut sprite::Scene<Texture>, render_args: event::RenderArgs) {
        let mut sprite = scene.child_mut(self.sprite)
            .expect("couldn't retrieve sprite asset");

        let width_mult  = render_args.width as f64/1280.0;
        let height_mult = render_args.height as f64/720.0;
        sprite.set_position(render_args.width as f64/2.0 + (self.loc[0]*width_mult), 
                            render_args.height as f64 - (self.loc[1]*height_mult));
    }
}

struct Turret {
    loc: vecmath::Vector2<f64>, // units relative to ship origin
    rot: f64, // radians clockwise from +x axis
}

impl Turret {
    /// Sets the direction towards which the turret is pointed.
    ///
    /// `r` is measured in radians clockwise from the positive x axis.
    fn set_rotation(&mut self, r: f64) {
        self.rot = r;
    }
}

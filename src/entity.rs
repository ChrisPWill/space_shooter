use std::path::Path;
use std::rc::Rc;

use vecmath;
use sprite;

use piston::event;
use sprite::Sprite;
use opengl_graphics::Texture;
use uuid::Uuid;

pub struct Player {
    loc: vecmath::Vector2<f64>,
    vel: vecmath::Vector2<f64>,
    sprite: Uuid,
}

impl Player {
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

    fn set_position(&mut self, x: f64, y: f64) {
        self.loc = [x, y];
    }
}

pub trait MobileUnit {
    fn update(&mut self, time_delta: f64);

    fn set_vel_x(&mut self, velX: f64);
    fn set_vel_y(&mut self, velY: f64);
}

impl MobileUnit for Player {
    fn update(&mut self, time_delta: f64) {
        self.loc = vecmath::vec2_add(self.loc.clone(), 
            vecmath::vec2_scale(self.vel.clone(), time_delta));

        // Ensure ship stays within boundaries
        if self.loc[0] >  640.0 { let y = self.loc[1]; self.set_position( 640.0,     y); }
        if self.loc[0] < -640.0 { let y = self.loc[1]; self.set_position(-640.0,     y); }
        if self.loc[1] >  600.0 { let x = self.loc[0]; self.set_position(     x, 600.0); }
        if self.loc[1] <    0.0 { let x = self.loc[0]; self.set_position(     x,   0.0); }
    }
    fn set_vel_x(&mut self, vel_x: f64) {
        self.vel[0] = vel_x;
    }
    fn set_vel_y(&mut self, vel_y: f64) {
        self.vel[1] = vel_y;
    }
}

pub trait Drawable {
    fn draw(&self, scene: &mut sprite::Scene<Texture>, render_args: event::RenderArgs);
}

impl Drawable for Player {
    fn draw(&self, scene: &mut sprite::Scene<Texture>, render_args: event::RenderArgs) {
        let mut sprite = scene.child_mut(self.sprite)
            .expect("couldn't retrieve sprite asset");

        let rect = sprite.bounding_box(); 
        let width_mult  = render_args.width as f64/1280.0;
        let height_mult = render_args.height as f64/720.0;
        sprite.set_position(render_args.width as f64/2.0 + (self.loc[0]*width_mult), 
                            render_args.height as f64 - (self.loc[1]*height_mult) - rect[3]);
    }

}

use std::path::Path;

use vecmath;
use piston::event;

use super::assets::AssetManager;

struct Player {
    loc: vecmath::Vector2<f64>,
    vel: vecmath::Vector2<f64>,
    sprite: u64,
}

impl Player {
    fn new(asset_manager: &mut AssetManager) -> Player {
        // load sprite
        let path = Path::new("./assets/playerShip1_red.png");
        let id = asset_manager.sprites.load_from_path(path).unwrap();

        Player{loc: [0.0, 0.0], vel: [0.0, 0.0], sprite: id}
    }
}

trait MobileUnit {
    fn update(&mut self, time_delta: i64);

    fn set_vel_x(&mut self, velX: f64);
    fn set_vel_y(&mut self, velY: f64);
}

impl MobileUnit for Player {
    fn update(&mut self, time_delta: i64) {
        self.loc = vecmath::vec2_add(self.loc.clone(), 
            vecmath::vec2_scale(self.vel.clone(), ((time_delta as f64)/1000.0)));
    }
    fn set_vel_x(&mut self, vel_x: f64) {
        self.vel[0] = vel_x;
    }
    fn set_vel_y(&mut self, vel_y: f64) {
        self.vel[1] = vel_y;
    }
}

trait Drawable {
    fn draw(&self, asset_manager: &mut AssetManager, render_args: event::RenderArgs);
}

impl Drawable for Player {
    fn draw(&self, asset_manager: &mut AssetManager, render_args: event::RenderArgs) {
        let mut sprite = asset_manager.sprites.get_mut(&self.sprite)
            .expect("couldn't retrieve sprite asset");

        let rect = sprite.bounding_box(); 
        sprite.set_position(render_args.width as f64/2.0 + self.loc[0], render_args.height as f64 - self.loc[1] - rect[3]);
    }

}

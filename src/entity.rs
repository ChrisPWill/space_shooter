use super::types::Vec2F;

struct Player {
    loc: Vec2F,
    vel: Vec2F,
}

trait MobileUnit {
    fn update(&mut self, time_delta: i64);

    fn set_vel_x(&mut self, velX: f64);
    fn set_vel_y(&mut self, velY: f64);
}

impl MobileUnit for Player {
    fn update(&mut self, time_delta: i64) {
        self.loc = self.loc.clone() + self.vel.clone()*((time_delta as f64)/1000.0);
    }
    fn set_vel_x(&mut self, vel_x: f64) {
        self.vel.x = vel_x;
    }
    fn set_vel_y(&mut self, vel_y: f64) {
        self.vel.y = vel_y;
    }
}

trait Drawable {
    fn draw(&self);
}

impl Drawable for Player {
    fn draw(&self) {
    }

    // Make textures some sort of static so they are only loaded once per
    // asset type.
    //
    //fn init_textures() {
    //}
}

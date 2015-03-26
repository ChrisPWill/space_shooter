struct Vec2F {
    x: f64,
    y: f64,
}

struct Player {
    loc: Vec2F,
    vel: Vec2F,
}

trait MobileUnit {
    fn set_vel_x(&mut self, velX: f64);
    fn set_vel_y(&mut self, velY: f64);
}

impl MobileUnit for Player {
    fn set_vel_x(&mut self, vel_x: f64) {
        self.vel.x = vel_x;
    }
    fn set_vel_y(&mut self, vel_y: f64) {
        self.vel.y = vel_y;
    }
}

fn main() {
    println!("Hello, world!");
}

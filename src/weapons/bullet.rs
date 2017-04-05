extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use piston::input::*;
use std::f64;
use vector::Vector;
use constants::bullet_constants::*;
use constants::color::*;

/// Bullets
pub struct Bullet {
    /// Position of the bullet
    pos: Vector,
    /// Velocity of the bullet
    vel: Vector,
    /// Rotation of the bullet
    rotation : f64,
    /// Whether the bullet is alive
    alive: bool,
    /// Whether the bullet is friendly
    friendly: bool
}

impl Bullet {
    /// Returns a bullet
    pub fn new(xpos: f64, ypos: f64, rot: f64, b: bool) -> Self {
        Bullet {
            pos: Vector::new(xpos, ypos),
            vel: Vector::new(VEL * rot.cos(), VEL * rot.sin()),
            rotation: rot,
            alive: true,
            friendly: b 
        }
    }

    /// Friendly bullets die once they hit the end of the screen
    fn friendly_mov(&mut self, args: &UpdateArgs, width: f64, height: f64) {
        // check x position
        if self.pos.x < 0.0 || self.pos.x > width || self.pos.y < 0.0 || self.pos.y > height {
            self.set_alive(false);
            return;
        }

        // update x and y
        self.pos.x += self.vel.x * VEL * args.dt;
        self.pos.y += self.vel.y * VEL * args.dt;
    }

    /// Enemy bullets bounce when they hit the end of the screen
    fn enemy_mov(&mut self, args: &UpdateArgs, width: f64, height: f64) {
        // make sure bullet doesnt leave the screen
        // check x position
        if self.pos.x < 0.0 || self.pos.x > width {
            self.vel.x *= -1.0;
        }

        // check y position
        if self.pos.y < 0.0 || self.pos.y > height {
            self.vel.y *= -1.0;
        }

        // update x and y
        self.pos.x += self.vel.x * VEL * args.dt;
        self.pos.y += self.vel.y * VEL * args.dt;
    }

    /// Draws the bullet
    pub fn draw(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;
        let square = rectangle::square(0.0, 0.0, SIDELENGTH);

        // get color of bullet
        let color = if self.get_friendly() {
            LIGHTBLUE
        } else {
            RED
        };

        // create transform matrix
        let transform = c.transform
            .trans(self.pos.x, self.pos.y)
            .rot_rad(self.rotation)
            .trans(-SIDELENGTH / 2.0, -SIDELENGTH / 2.0);

        // Draw a box rotating around the middle of the screen.
        rectangle(color, square, transform, gl);
    }

    /// Updates the bullet position and rotation
    pub fn update(&mut self, args: &UpdateArgs, dimensions: &[f64; 2]) {
        if self.get_friendly() {
            self.friendly_mov(args, dimensions[0], dimensions[1]);
        } else {
            self.enemy_mov(args, dimensions[0], dimensions[1]);
        }

        self.rotation = self.vel.y.atan2(self.vel.x);
    }

    /// Returns whether the bullet is alive
    pub fn get_alive(&self) -> bool {
        self.alive
    }

    /// Updates whether the bullet is alive
    pub fn set_alive(&mut self, b: bool) {
        self.alive = b;
    }

    /// Returns the bullet's x position
    pub fn get_x(&self) -> f64 {
        self.pos.x
    }

    /// Returns the bullet's y position
    pub fn get_y(&self) -> f64 {
        self.pos.y
    }

    /// Returns whether the bullet is friendly
    fn get_friendly(&self) -> bool {
        self.friendly
    }
}

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use opengl_graphics::glyph_cache::GlyphCache;
use rand::Rng;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::f64;
use vector::Vector;
use constants::boss_constants::*;
use weapons::bullet::Bullet;
use constants::color::*;

pub struct Boss {
    /// Position of the boss
    pos: Vector,
    /// Health of the boss
    health: u32,
    /// Cooldown for shooting
    cooldown: f64,
    /// Rotation of the boss
    rotation: f64,
}

impl Boss {
    /// Returns a boss
    pub fn new(x: f64, y: f64) -> Self {
        Boss {
            pos: Vector::new(x, y),
            health: STARTHEALTH,
            cooldown: rand::thread_rng().gen_range(0.0, COOLDOWN),
            rotation: rand::thread_rng().gen_range(0.0, 2.0 * f64::consts::PI),
        }
    }

    /// Check if the boss was hit by a bullet
    pub fn hit(&mut self, ref mut b: &mut Bullet) {
        // check the distance between bullet and boss
        let xdiff = self.pos.x - b.get_x();
        let ydiff = self.pos.y - b.get_y();
        let dist = (xdiff.powi(2) + ydiff.powi(2)).sqrt();

        // update health and kill bullet if boss is hit
        if self.get_alive() && dist < BOSSD / 2.0 - EPSILON {
            self.decrease_health();
            b.set_alive(false);
        }
    }

    /// Returns a Vector of Bullets
    pub fn shoot_bullets(&mut self) -> Vec<Bullet> {
        let mut bullets: Vec<Bullet> = Vec::new();
        for x in 0..5 {
            let rot = x as f64 * 2.0 * f64::consts::PI / TOTBULL+ self.rotation;
            bullets.push(Bullet::new(self.pos.x, self.pos.y, rot, true));
        }
        bullets
    }

    /// Update the boss position and velocity. Return a bullet if it is shooting.
    pub fn update(&mut self, args: &UpdateArgs) -> Option<Vec<Bullet>> {

        // check if boss can shoot
        if self.can_shoot() {
            self.update_cooldown(args.dt);
            return Some(self.shoot_bullets());
        }

        // update cooldown
        self.update_cooldown(args.dt);
        // update rotation
        self.update_rotation(args.dt);
        None
    }

    /// Draws the boss
    pub fn draw(&self, c: graphics::Context, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        use graphics::*;

        let circle = rectangle::square(0.0, 0.0, BOSSD);
        let square = rectangle::square(0.0, 0.0, GUND);

        // create transform matrix
        let transform = c.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-BOSSD / 2.0, -BOSSD / 2.0);

        // Draw the boss
        ellipse(GREEN, circle, transform, gl);

        // Draw where the bullets where go
        for x in 0..(TOTBULL as i64) {
            let rot = x as f64 * 2.0 * f64::consts::PI / TOTBULL + self.rotation;
            let transform2 = c.transform
                .trans(self.pos.x, self.pos.y)
                .rot_rad(rot)
                .trans(BOSSD / 2.0 - GUND / 2.0, -GUND / 2.0);
            rectangle(PINK, square, transform2, gl);
        }

        // display the health
        text(WHITE,
             FONTSIZE,
             format!("{}", self.get_health()).as_str(),
             glyph_cache,
             c.transform.trans(self.pos.x, self.pos.y),
             gl);
    }

    /// Returns whether the boss is alive
    pub fn get_alive(&self) -> bool {
        self.health != 0
    }

    /// Returns the health of the boss
    pub fn get_health(&self) -> u32 {
        self.health
    }

    /// Decreases the health of the boss
    fn decrease_health(&mut self) {
        self.health -= 1;
    }

    /// Updates the cooldown of shooting
    fn update_cooldown(&mut self, dt: f64) {
        if self.cooldown < 0.0 {
            self.cooldown = COOLDOWN;
        } else {
            self.cooldown -= dt;
        }
    }

    /// Updates the rotation of the boss
    fn update_rotation(&mut self, dt: f64) {
        // update rotation of the boss
        self.rotation += dt * ROTMULT ;
    }

    /// Returns whether the boss can shoot
    fn can_shoot(&self) -> bool {
        self.cooldown < 0.0
    }
}
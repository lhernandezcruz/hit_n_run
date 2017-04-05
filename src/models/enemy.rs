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
use constants::enemy_constants::*;
use weapons::bullet::Bullet;
use constants::color::*;

/// Enemy
pub struct Enemy {
    /// Position of the enemy
    pos: Vector,
    /// Desired position of the enemy
    desired_pos: Vector,
    /// Velocity of the enemy
    vel: Vector,
    /// Rotation of the player
    rotation: f64,
    /// Whether the enemy is moving forward
    forward: bool,
    /// Health of the enemy
    health: u32,
    /// Cooldown for shooting
    cooldown: f64
}

impl Enemy {
    /// Returns an enemy
    pub fn new(x: f64, y: f64, b: bool) -> Self {
        Enemy {
            pos: Vector::new(x, y),
            desired_pos: Vector::new(0.0, 0.0),
            vel: Vector::new(0.0, 0.0),
            rotation: 0.0,
            forward: b,
            health: STARTHEALTH,
            cooldown: COOLDOWN
        }
    }

    /// Check if enemy is hit by a bullet
    pub fn hit(&mut self, ref mut b: &mut Bullet) {
        // check the distance between bullet and enemy
        let xdiff = self.pos.x - b.get_x();
        let ydiff = self.pos.y - b.get_y();
        let dist = (xdiff.powi(2) + ydiff.powi(2)).sqrt();

        // update health and kill bullet if enemy is hit
        if self.get_alive() && dist < ENEMYR / 2.0 - EPSILON {
            self.decrease_health();
            b.set_alive(false);
        }
    }

    /// Updates the desired postion and rotation.
    pub fn desired_update(&mut self, desx: f64, desy: f64) {
        // update desired x position
        self.desired_pos.x = desx;
        self.desired_pos.y = desy;

        // update angle
        let xdiff = desx - self.pos.x;
        let ydiff = desy - self.pos.y;
        let mag = (xdiff.powi(2) + ydiff.powi(2)).sqrt();
        let unitx = xdiff / mag;
        let unity = ydiff / mag;

        // turn unit vector to radians
        self.rotation = unity.atan2(unitx);
    }

    /// Move the player to a new postion
    fn mov(&mut self, width: f64, height: f64) {
        // make sure player doesnt leave the screen
        // check x position
        if self.pos.x < 0.0 {
            self.pos.x += MOVEBACK;
        } else if self.pos.x > width {
            self.pos.x -= MOVEBACK;
        }

        // check y position
        if self.pos.y < 0.0 {
            self.pos.y += MOVEBACK;
        } else if self.pos.y > height {
            self.pos.y -= MOVEBACK;
        }

        // update x and y
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    /// Update the enemys position and velocity. Return a bullet if it is shooting.
    pub fn update(&mut self,
                  args: &UpdateArgs,
                  desx: f64,
                  desy: f64,
                  dimensions: &[f64; 2])
                  -> Option<Bullet> {

        // update desired postion and rotation
        self.desired_update(desx, desy);
        let dist = self.pos.dist(&self.desired_pos);

        // if the enemy is not at desired location keep moving
        if dist > ENEMYR && self.forward {
            self.vel.x = VEL * args.dt * self.rotation.cos();
            self.vel.y = VEL * args.dt * self.rotation.sin();
        } else {
            self.vel.reset();
        }

        // move enemy
        self.mov(dimensions[0], dimensions[1]);

        // check if enemy can shoot
        if self.can_shoot() {
            self.update_cooldown(args.dt);

            // has small error when shooting
            let rot = rand::thread_rng().gen_range(-SHOOTINGERR, SHOOTINGERR);
            return Some(Bullet::new(self.pos.x + ENEMYR / 2.0 * self.rotation.cos(),
                                    self.pos.y + ENEMYR / 2.0 * self.rotation.sin(),
                                    self.rotation + rot,
                                    true));
        }

        // update cooldown
        self.update_cooldown(args.dt);
        None
    }

    /// Draws the enemy
    pub fn draw(&self, c: graphics::Context, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        use graphics::*;

        let circle = rectangle::square(0.0, 0.0, ENEMYR);
        let square = rectangle::square(0.0, 0.0, GUNR);

        // create transform matrix
        let transform = c.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-ENEMYR / 2.0, -ENEMYR / 2.0);

        let transform2 = c.transform
            .trans(self.pos.x, self.pos.y)
            .rot_rad(self.rotation)
            .trans(ENEMYR / 2.0 - GUNR / 2.0, -GUNR / 2.0);

        // Draw a box rotating around the middle of the screen.
        ellipse(BLUE, circle, transform, gl);
        rectangle(PINK, square, transform2, gl);

        // display the health
        text(WHITE,
             FONTSIZE,
             format!("{}", self.get_health()).as_str(),
             glyph_cache,
             c.transform.trans(self.pos.x, self.pos.y),
             gl);
    }

    /// Returns whether the enemy is alive
    pub fn get_alive(&self) -> bool {
        self.health != 0
    }

    /// Returns whether the enemy can shoot
    fn can_shoot(&self) -> bool {
        self.cooldown < 0.0
    }

    /// Updates the cooldown of shooting
    fn update_cooldown(&mut self, dt: f64) {
        if self.cooldown < 0.0 {
            self.cooldown = COOLDOWN;
        } else {
            self.cooldown -= dt;
        }
    }

    /// Returns the health of the enemy
    fn get_health(&self) -> u32 {
        self.health
    }

    /// Decreases the health of the enemy
    fn decrease_health(&mut self) {
        self.health -= 1;
    }
}

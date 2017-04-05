extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;

use opengl_graphics::GlGraphics;
use piston::input::*;
use std::f64;
use vector::Vector;
use weapons::bullet::Bullet;
use constants::player_constants::*;
use constants::color::*;
use opengl_graphics::Texture;
use std::result::Result;

/// User controlled player
pub struct Player {
    /// Position of the player
    pos: Vector,
    /// Velocity of the player
    vel: Vector,
    /// Position where the player wants to be
    desired_pos: Vector,
    /// Rotation of the player
    rotation: f64,
    /// Health of the player
    health: u32,
    /// Whether the player is shooting
    is_shooting: bool,
    /// Cooldown of shooting (0 when the player can shoot)
    cooldown: f64,
    /// Texture for image of player
    texture: Result<Texture, String>,
}

impl Player {
    /// Returns a Player
    pub fn new(xpos: f64, ypos: f64) -> Self {
        Player {
            pos: Vector::new(xpos, ypos),
            vel: Vector::new(0.0, 0.0),
            desired_pos: Vector::new(xpos, ypos),
            rotation: 0.0,
            health: STARTHEALTH,
            is_shooting: false,
            cooldown: 0.0,
            texture: Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("player.png")),
        }
    }

    /// Check if player is hit by a bullet
    pub fn hit(&mut self, ref mut b: &mut Bullet) {
        // check the distance between bullet and player
        let xdiff = self.pos.x - b.get_x();
        let ydiff = self.pos.y - b.get_y();
        let dist = (xdiff.powi(2) + ydiff.powi(2)).sqrt();

        // update health and kill bullet if player is hit
        if self.get_alive() && dist < PLAYERR / 2.0 - EPSILON {
            self.decrease_health();
            b.set_alive(false);
        }
    }

    /// Updates the desired position and the rotation.
    pub fn desired_update(&mut self, mouse_x: f64, mouse_y: f64) {
        // update desired x position
        self.desired_pos.x = mouse_x;
        self.desired_pos.y = mouse_y;

        // update angle
        let xdiff = mouse_x - self.pos.x;
        let ydiff = mouse_y - self.pos.y;
        let mag = (xdiff.powi(2) + ydiff.powi(2)).sqrt();
        let unitx = xdiff / mag;
        let unity = ydiff / mag;

        // turn unit vector to radians
        self.rotation = unity.atan2(unitx);
    }

    /// Move the player based on its velocity
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

    /// Update the players position and velocity. Return a bullet if it is shooting.
    pub fn update(&mut self, args: &UpdateArgs, dimensions: &[f64; 2]) -> Option<Bullet> {
        // get distance to desired location
        let dist = self.pos.dist(&self.desired_pos);

        // if the player is not at desired location keep velocity else stop velocity
        if dist > 3.0 {
            self.vel.x = VEL * args.dt * self.rotation.cos();
            self.vel.y = VEL * args.dt * self.rotation.sin();
        } else {
            self.vel.reset();
        }

        // move player and reset velocity
        self.mov(dimensions[0], dimensions[1]);

        // check if player is shooting
        if self.get_shooting() {
            // update cooldown
            self.update_cooldown(args.dt);

            // return bullet
            if self.can_shoot() {
                return Some(Bullet::new(self.pos.x + PLAYERR / 2.0 * self.rotation.cos(),
                                        self.pos.y + PLAYERR / 2.0 * self.rotation.sin(),
                                        self.rotation,
                                        false));
            }
        }

        // no bullet shot
        None
    }

    /// Draws the player
    pub fn draw(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        let circle = rectangle::square(0.0, 0.0, PLAYERR);
        let square = rectangle::square(0.0, 0.0, GUNR);

        // draw image
        // create transform matrix
        let transform = c.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-PLAYERR / 2.0, -PLAYERR / 2.0);

        let transform2 = c.transform
            .trans(self.pos.x, self.pos.y)
            .rot_rad(self.rotation)
            .trans(PLAYERR / 2.0 - GUNR / 2.0, -GUNR / 2.0);

        // move this to a sprite class
        let transform3 = c.transform
            .trans(self.pos.x, self.pos.y)
            .rot_rad(self.rotation)
            .trans(-PLAYERR / 2.0, -PLAYERR / 2.0);

        // draw a circle rotating around the middle of the screen.
        ellipse(PINK, circle, transform, gl);
        rectangle(WHITE, square, transform2, gl);

        // check if we have an image for the player
        match self.texture {
            Ok(ref t) => image(t, transform3, gl),
            _ => {}
        }

    }

    /// Return the health
    pub fn get_health(&self) -> u32 {
        self.health
    }

    /// Increase the health
    pub fn increase_health(&mut self) {
        self.health += 1;
    }

    /// Decreases the health by 1
    pub fn decrease_health(&mut self) {
        self.health -= 1;
    }

    /// Return whether the player is alive or not
    pub fn get_alive(&self) -> bool {
        self.get_health() != 0
    }

    /// Reset the player's position and health.
    pub fn reset(&mut self, width: f64, height: f64) {
        self.pos.x = width / 2.0;
        self.pos.y = height / 2.0;
        self.health = STARTHEALTH;
    }

    /// Start shooting
    pub fn start_shooting(&mut self) {
        self.is_shooting = true;
    }

    /// Return whether the player is shooting
    fn get_shooting(&self) -> bool {
        self.is_shooting
    }

    /// Update the cooldown of shooting
    fn update_cooldown(&mut self, dt: f64) {
        // if it is less than 0 we reset it.
        if self.cooldown < 0.0 {
            self.cooldown = COOLDOWN;
        } else {
            self.cooldown -= dt;
        }
    }

    /// Stop shooting.
    pub fn stop_shooting(&mut self) {
        self.is_shooting = false;
        self.cooldown = 0.0;
    }

    /// Return whether the player can shoot.
    fn can_shoot(&mut self) -> bool {
        self.cooldown < 0.0
    }

    /// Return the x position of the player
    pub fn get_x(&self) -> f64 {
        self.pos.x
    }

    /// Return the y postion of the player
    pub fn get_y(&self) -> f64 {
        self.pos.y
    }
}
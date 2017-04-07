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
use models::enemy::Enemy;
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
    shots: u32,
    /// Cooldown between shots
    scooldown: f64,
    /// Cooldown between bursts
    bcooldown: f64,
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
            shots: STARTSHOTS,
            scooldown: 0.0,
            bcooldown: 0.0,
            texture: Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("player.png")),
        }
    }

    /// Check if the player collided with a plaer
    pub fn collide(&mut self, ref mut enemy: &mut Enemy) {
        let xdiff = self.pos.x - enemy.get_orb_x();
        let ydiff = self.pos.y - enemy.get_orb_y();
        let dist = (xdiff.powi(2) + ydiff.powi(2)).sqrt();

        // check if we crashed.
        if self.get_alive() && enemy.get_orb_active() && dist < PLAYERD / 2.0 {
            self.decrease_health();
            enemy.set_orb_active(false);
        }
    }
    /// Check if player is hit by a bullet
    pub fn hit(&mut self, ref mut b: &mut Bullet) {
        // check the distance between bullet and player
        let xdiff = self.pos.x - b.get_x();
        let ydiff = self.pos.y - b.get_y();
        let dist = (xdiff.powi(2) + ydiff.powi(2)).sqrt();

        // update health and kill bullet if player is hit
        if self.get_alive() && dist < PLAYERD / 2.0 - EPSILON {
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
        if dist > EPSILON {
            self.vel.x = VEL * args.dt * self.rotation.cos();
            self.vel.y = VEL * args.dt * self.rotation.sin();
        } else {
            self.vel.reset();
        }

        // move player and reset velocity
        self.mov(dimensions[0], dimensions[1]);

        // update cooldown
        self.update_cooldown(args.dt);

        // return bullet
        if self.can_shoot() && self.get_shooting() {
            self.scooldown = SHOTCOOLDOWN;
            self.shots -= 1;
            return Some(Bullet::new(self.pos.x + PLAYERD / 2.0 * self.rotation.cos(),
                                    self.pos.y + PLAYERD / 2.0 * self.rotation.sin(),
                                    self.rotation,
                                    false));
        }

        // no bullet shot
        None
    }

    /// Draws the player
    pub fn draw(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        // circle fot the body
        let circle = rectangle::square(0.0, 0.0, PLAYERD);
        // square for the gun
        let square = rectangle::square(0.0, 0.0, GUND);

        // create transform matrix for body and gun
        let bodytrans = c.transform
            .trans(self.pos.x, self.pos.y)
            .rot_rad(self.rotation)
            .trans(-PLAYERD / 2.0, -PLAYERD / 2.0);

        let guntrans = c.transform
            .trans(self.pos.x, self.pos.y)
            .rot_rad(self.rotation)
            .trans(PLAYERD / 2.0 - GUND / 2.0, -GUND / 2.0);

        // create transfrom matrix for cooldown bar
        let bartrans = c.transform.trans(self.pos.x - PLAYERD/2.0, self.pos.y);

        // draw a circle rotating around the middle of the screen.
        ellipse(PINK, circle, bodytrans, gl);
        rectangle(WHITE, square, guntrans, gl);

        // check if we have an image for the player
        match self.texture {
            Ok(ref t) => image(t, bodytrans, gl),
            _ => {}
        }

        if self.bcooldown <= 0.0 {
            // get length of bars and draw current shots bar and cooldown bar
            let slength = PLAYERD/(STARTSHOTS as f64) * (self.shots as f64);
            let clength = PLAYERD/(SHOTCOOLDOWN as f64) * (self.scooldown as f64);

            line(ORANGE, BARWIDTH, [0.0, SBARDIST, slength, SBARDIST], bartrans, gl);
            line(ANGEL, BARWIDTH, [0.0, CBARDIST, clength, CBARDIST], bartrans, gl);
        } else {
            // get length for bar and draw cooldown bar
            let blength = PLAYERD/(BURSTCOOLDOWN as f64) * (self.bcooldown);
            line(ANGEL, BARWIDTH, [0.0, BBARDIST, blength, BBARDIST], bartrans, gl);
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
        self.rotation = 0.0;
        self.bcooldown = 0.0;
        self.scooldown = 0.0;
        self.desired_pos.reset();
        self.vel.reset();
        self.shots = STARTSHOTS;
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
        // we ran out of burst shots. so now we update burst cooldown
        if self.shots == 0 {
            self.scooldown = 0.0;
            self.bcooldown = BURSTCOOLDOWN;
            self.shots = STARTSHOTS;
        }

        // check if we can shoot
        if self.bcooldown <= 0.0 {
            if self.scooldown <= 0.0 {
                // don't want our bar to be negative
                self.scooldown = 0.0;
            } else {
                self.scooldown -= dt;
            }
        } else {
            self.bcooldown -= dt;
        }
    }

    /// Stop shooting.
    pub fn stop_shooting(&mut self) {
        self.is_shooting = false;
    }

    /// Return whether the player can shoot.
    fn can_shoot(&mut self) -> bool {
        self.bcooldown <= 0.0 && self.scooldown <= 0.0
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

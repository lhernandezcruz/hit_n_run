extern crate rand;

use rand::Rng;
use vector::Vector;
use std::f64;
use constants::orb_constants::*;

/// Orbs
pub struct Orb {
    /// Whether the orb is active or not
    active: bool,
    /// The angle at which it is found
    theta: f64,
    /// The position of the angle
    pos: Vector,
    /// Cooldown of orb activity
    cooldown: f64
}

impl Orb {
    /// Return a new orb
    pub fn new(x: f64, y: f64) -> Self {
        // get the original position of the orb
        let rng_angle = rand::thread_rng().gen_range(0.0, 2.0);
        let posx = x + (ORBD*f64::consts::PI*rng_angle).cos();
        let posy = y + (ORBD*f64::consts::PI*rng_angle).sin();
        Orb {
            active: true,
            theta: rng_angle,
            pos: Vector::new(posx,posy),
            cooldown: COOLDOWN
        }
    }

    /// Return the x position of the orb
    pub fn get_x(&self) -> f64 {
        self.pos.x
    }

    /// Return the y position of the orb
    pub fn get_y(&self) -> f64 {
        self.pos.y
    }

    /// Update the orbs postition and angle
    pub fn update(&mut self, x: f64, y :f64, dt: f64) {
        self.theta += VEL*f64::consts::PI;

        // 2pi is the same as 0
        if (self.theta - 2.0*f64::consts::PI).abs() < EPSILON {
            self.theta = 0.0;
        }

        // update position
        self.pos.x = x + ORBD*self.theta.cos();
        self.pos.y = y + ORBD*self.theta.sin();

        // udpate active cooldown
        if !self.active {
            if self.cooldown < 0.0 {
                self.cooldown = COOLDOWN;
                self.set_active(true);
            } else {
                self.cooldown -= dt;
            }
        }
    }

    /// Set whether the orb is active or not
    pub fn set_active(&mut self, b: bool) {
        self.active = b;
    }

    /// Return whether the orb is active or not
    pub fn get_active(&self) -> bool {
        self.active
    }

    
}

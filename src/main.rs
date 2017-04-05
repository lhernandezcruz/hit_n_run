extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use opengl_graphics::glyph_cache::GlyphCache;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod game;
mod models;
mod constants;
mod vector;
mod weapons;

// use get width and height for game
use constants::sizes;

fn main() {
    // create opengl and window
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("HIT AND RUN",
                                                 [sizes::INITWIDTH, sizes::INITHEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // create gl graphics and game
    let mut gl = GlGraphics::new(opengl);
    let mut g = game::Game::new(sizes::INITWIDTH as f64, sizes::INITHEIGHT as f64);

    let mut glyph_cache = GlyphCache::new("assets/Roboto-Regular.ttf").unwrap();

    // run game
    g.run(&mut window, &mut gl, &mut glyph_cache);
}
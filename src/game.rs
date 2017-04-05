extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use rand::Rng;
use glutin_window::GlutinWindow as Window;

use constants::game_constants::*;
use constants::color::*;
use models::enemy::Enemy;
use models::player::Player;
use weapons::bullet::Bullet;

/// Contains Game State
pub struct Game {
    /// User controlled player. controlled with mouse and keyboard
    player: Player,
    /// Enemies that have to be killed
    enemies: Vec<Enemy>,
    /// Bullets shot by the player
    player_bullets: Vec<Bullet>,
    /// Bullets shot by the enemy
    enemy_bullets: Vec<Bullet>,
    /// Dimensions of the game screen [width,height]
    dimensions: [f64; 2],
    /// Score of the game.
    score: u32,
    /// Current level of the game.
    level: u32,
    /// Kills made during this level
    current_kills: u32,
    /// But did you die tho?
    game_over: bool
}

impl Game {
    /// Returns a game
    ///
    /// # Arguments
    ///
    /// * `width` - An integer that holds the width of the screen
    /// * `height` - An integer that holds the height of the screen
    pub fn new(width: f64, height: f64) -> Self {
        Game {
            // player starts out at center of screen
            player: Player::new(width / 2.0, height / 2.0),
            player_bullets: Vec::<Bullet>::new(),
            enemy_bullets: Vec::<Bullet>::new(),
            enemies: Vec::<Enemy>::new(),
            dimensions: [width, height],
            score: 0,
            level: 1,
            current_kills: 0,
            game_over: false,
        }

    }

    /// Updates all the things in the game
    ///
    /// # Arguments
    ///
    /// * `args` - An event that contains update information
    fn on_update(&mut self, args: &UpdateArgs) {
        // update player, check if a bullet was shot
        match self.player.update(args, &self.dimensions) {
            Some(bullet) => self.player_bullets.push(bullet),
            None => {}
        }

        // check for hit
        for bullet in &mut self.enemy_bullets {
            bullet.update(args, &self.dimensions);
            self.player.hit(bullet);
        }

        // check if we should end the game
        if !self.player.get_alive() {
            self.update_game_over(true);
        }

        for enemy in &mut self.enemies {
            // update enemy, check for bullet
            let shot = enemy.update(args,
                                    self.player.get_x(),
                                    self.player.get_y(),
                                    &self.dimensions);
            match shot {
                Some(bullet) => self.enemy_bullets.push(bullet),
                None => {}
            }
        }

        // check if enemy was hit
        for bullet in &mut self.player_bullets {
            bullet.update(args, &self.dimensions);
            for enemy in &mut self.enemies {
                enemy.hit(bullet);
            }
        }

        // remove bullets that are no longer alive
        self.player_bullets.retain(|e| e.get_alive());
        self.enemy_bullets.retain(|e| e.get_alive());

        // remove dead enemies
        let len = self.enemies.len();
        self.enemies.retain(|e| e.get_alive());

        // check if an enemy was killed
        if len != self.enemies.len() {
            // update stats
            self.player.increase_health();
            self.update_score();
            self.update_kills();

            // check if we have passed a level
            if self.new_level() {
                self.enemies.clear();
                self.add_enemy();
                self.update_level();
            } else {
                self.add_enemy();
                self.add_enemy();
            }

        }
    }

    /// Draws the gameboard
    fn on_draw(&self, args: &RenderArgs, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        use graphics::*;

        // draw viewport
        gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(BLACK, gl);

            // draw enemy bullets
            for bullet in &self.enemy_bullets {
                bullet.draw(c, gl);
            }

            // draw friendly bullets
            for bullet in &self.player_bullets {
                bullet.draw(c, gl);
            }

            // draw enemies
            for enemy in &self.enemies {
                enemy.draw(c, gl, glyph_cache);
            }

            // draw player
            self.player.draw(c, gl);

            // display score and health
            text(WHITE,
                 FONTSIZE,
                 format!("Score: {} | Health: {} | Level: {} | Level Kills: {}",
                         self.get_score(),
                         self.player.get_health(),
                         self.get_level(),
                         self.get_kills())
                     .as_str(),
                 glyph_cache,
                 c.transform.trans(15.0, 15.0),
                 gl);

            // draw reset screen
            if !self.player.get_alive() {
                text(ORANGE,
                     50,
                     format!("RIP. R to Reset").as_str(),
                     glyph_cache,
                     c.transform.trans(15.0, 75.0),
                     gl);
            }

        });

    }

    /// Adds enemies to the game. Enemies spawn randomly in the map
    fn add_enemy(&mut self) {
        // get random x and y locations
        let x = rand::thread_rng().gen_range(0.0, self.dimensions[0]);
        let y = rand::thread_rng().gen_range(0.0, self.dimensions[1]);

        // 1 / 5 chance of being able to move forward
        let random_num = rand::thread_rng().gen_range(1, 5);
        if random_num == 1 {
            self.enemies.push(Enemy::new(x, y, true));
        } else {
            self.enemies.push(Enemy::new(x, y, false));
        }
    }

    /// Updates the size of the window when it is resized
    fn on_resize(&mut self, new_dimensions: &[u32; 2]) {
        self.dimensions[0] = new_dimensions[0] as f64;
        self.dimensions[1] = new_dimensions[1] as f64;
    }

    /// Updates player desired location when mouse is moved
    fn on_mouse_mov(&mut self, motion: &[f64; 2]) {
        self.player.desired_update(motion[0], motion[1]);
    }

    /// Updates things in the game when a key is pressed
    /// # Arguments
    /// * `key` - the key that was pressed
    ///
    /// # Remarks
    ///
    /// Space is used to start shoot.
    /// R is used to reset game
    fn on_key_press(&mut self, key: Key) {
        match key {
            Key::Space => {
                if !self.get_game_over() {
                    self.player.start_shooting();
                }
            }
            Key::R => {
                self.reset();
            }
            _ => {}
        }
    }

    /// Resets the game
    fn reset(&mut self) {
        self.enemies.clear();
        self.player_bullets.clear();
        self.enemy_bullets.clear();
        self.score = 0;
        self.level = 1;
        self.current_kills = 0;
        self.player.reset(self.dimensions[0], self.dimensions[1]);
        self.add_enemy();
        self.update_game_over(false);
    }

    /// Called when a key is released
    /// # Arguments
    /// *  `key` - the key that is released
    /// # Remarks
    /// When spacebar is released the player stops shooting
    fn on_key_release(&mut self, key: Key) {
        match key {
            Key::Space => {
                self.player.stop_shooting();
            }
            _ => {}
        }
    }

    /// Runs the game
    /// # Arguments
    /// * `window` - Wiindow that displays that game
    /// * `gl`     - Graphics used to draw on window
    /// * `glyph_cache - Used for font
    pub fn run(&mut self,
               mut window: &mut Window,
               mut gl: &mut GlGraphics,
               mut glyph_cache: &mut GlyphCache) {

        // check for events and set the fps
        let mut events = Events::new(EventSettings::new());
        events.set_ups(FPS);

        // add an enemy to start off with
        self.add_enemy();

        while let Some(e) = events.next(window) {

            // make sure the game isnt over
            if !self.get_game_over() {
                // check mouse location
                if let Some(m) = e.mouse_cursor_args() {
                    self.on_mouse_mov(&m);
                }

                // on key release
                if let Some(k) = e.release_args() {
                    match k {
                        Button::Keyboard(key) => self.on_key_release(key),
                        _ => {}
                    }
                }

                // upon resize we change dimensions of game
                if let Some(r) = e.resize_args() {
                    self.on_resize(&r);
                }

                // on update
                if let Some(r) = e.update_args() {
                    self.on_update(&r);
                }
            };

            // on key press
            if let Some(k) = e.press_args() {
                match k {
                    Button::Keyboard(key) => self.on_key_press(key),
                    _ => {}
                }
            }

            // upon render we draw things
            if let Some(u) = e.render_args() {
                self.on_draw(&u, &mut gl, &mut glyph_cache);
            }
        }

    }

    /// Returns the current score
    fn get_score(&self) -> u32 {
        self.score
    }

    /// Returns the current level
    fn get_level(&self) -> u32 {
        self.level
    }

    /// Returns the amount of kills in the current level
    fn get_kills(&self) -> u32 {
        self.current_kills
    }

    /// Returns whether we have moved to new level
    fn new_level(&self) -> bool {
        self.current_kills == self.level
    }

    /// Updates the score of the game
    fn update_score(&mut self) {
        self.score += 1;
    }

    /// Update the amount of kills in the current level
    fn update_kills(&mut self) {
        self.current_kills += 1;
    }

    /// Update the level and reset current_kills
    fn update_level(&mut self) {
        self.level += 1;
        self.current_kills = 0;
    }

    /// Sets the value of game over
    fn update_game_over(&mut self, b: bool) {
        self.game_over = b;
    }

    /// Returns whether the game is over
    fn get_game_over(&mut self) -> bool {
        self.game_over
    }
}
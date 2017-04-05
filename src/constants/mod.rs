/// Contains colors that are used in the game
pub mod color {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const LIGHTBLUE: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const PINK: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
}

/// Contains initial game screen sizes
pub mod sizes {
    /// Initial width of the screen
    pub const INITWIDTH: u32 = 600;
    /// Initial height of the screen
    pub const INITHEIGHT: u32 = 600;
}

/// Contains player constants
pub mod player_constants {
    /// Radius of the player
    pub const PLAYERR: f64 = 50.0;
    /// Radius of the gun
    pub const GUNR: f64 = 10.0;
    /// How much to move back when player hits the end of screen
    pub const MOVEBACK: f64 = 15.0;
    /// Velocity of player
    pub const VEL: f64 = 250.0;
    /// Bullet must be this amount inside to count as a hit
    pub const EPSILON: f64 = 0.25;
    /// Cooldown for shooting
    pub const COOLDOWN: f64 = 0.75;
    /// Starting health for player
    pub const STARTHEALTH: u32 = 30;
}

/// Contains enemy constants
pub mod enemy_constants {
    /// How much to move back when hitting the wall
    pub const MOVEBACK: f64 = 15.0;
    /// Radius of the enemy
    pub const ENEMYR: f64 = 50.0;
    /// Radius of the gun
    pub const GUNR: f64 = 10.0;
    /// Velocity of the enemy
    pub const VEL: f64 = 50.0;
    /// Starting health of enemy
    pub const STARTHEALTH: u32 = 6;
    /// Bullet collision error
    pub const EPSILON: f64 = 0.1;
    /// Cooldown for shooting
    pub const COOLDOWN: f64 = 1.5;
    /// Fontsize for health
    pub const FONTSIZE: u32 = 15;
    /// Starting health
    pub const SHOOTINGERR: f64 = 0.1;
}

/// Contains bullet constants
pub mod bullet_constants {
    /// Velocity of bullet
    pub const VEL: f64 = 25.0;
    /// Sidelength of bullet
    pub const SIDELENGTH: f64 = 10.0;
}

/// Contains game constants
pub mod game_constants {
    /// FPS for game
    pub const FPS: u64 = 60;
    /// Font size for score
    pub const FONTSIZE: u32 = 15;
}

/// Contains colors that are used in the game
pub mod color {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const LIGHTBLUE: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const PINK: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
    pub const ANGEL: [f32; 4 ] = [0.5,0.5,1.0,0.5];
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
    /// Diameter of the player
    pub const PLAYERD: f64 = 50.0;
    /// Diameter of the gun
    pub const GUND: f64 = 10.0;
    /// How much to move back when player hits the end of screen
    pub const MOVEBACK: f64 = 15.0;
    /// Velocity of player
    pub const VEL: f64 = 250.0;
    /// Bullet must be this amount inside to count as a hit
    pub const EPSILON: f64 = 0.25;
    /// Cooldown between bursts
    pub const BURSTCOOLDOWN: f64 = 1.0;
    /// Cooldown between shots
    pub const SHOTCOOLDOWN: f64 = 0.25;
    /// Staring amount of shots
    pub const STARTSHOTS: u32 = 6;
    /// Starting health for player
    pub const STARTHEALTH: u32 = 30;
    /// Distance of shot amount bar from body
    pub const SBARDIST: f64 = 50.0;
    /// Distance of shot cooldown bar from body
    pub const CBARDIST: f64 = 60.0;
    /// Distance of burst cooldown bar from body
    pub const BBARDIST: f64 = 50.0;
    /// Width of the bar
    pub const BARWIDTH: f64 = 1.0;
}

/// Contains enemy constants
pub mod enemy_constants {
    /// How much to move back when hitting the wall
    pub const MOVEBACK: f64 = 15.0;
    /// Diameter of the enemy
    pub const ENEMYD: f64 = 50.0;
    /// Side length of the gun
    pub const GUND: f64 = 10.0;
    /// Velocity of the enemy
    pub const VEL: f64 = 50.0;
    /// Starting health of enemy
    pub const STARTHEALTH: u32 = 5;
    /// Bullet collision error
    pub const EPSILON: f64 = 0.1;
    /// Cooldown for shooting
    pub const COOLDOWN: f64 = 1.5;
    /// Fontsize for health
    pub const FONTSIZE: u32 = 15;
    /// Starting health
    pub const SHOOTINGERR: f64 = 0.1;
    /// Diameter of the actual orb 
    pub const ORBD: f64 = 10.0;
}

/// Contains orb constants
pub mod orb_constants {
    /// Diameter of the orb orbit
    pub const ORBD: f64 = 50.0;
    /// Vel multiplier for rotation
    pub const VEL: f64 = 0.01;
    /// Used for reseting the angle
    pub const EPSILON: f64 = 1e-10;
    /// Cooldown for when it is active
    pub const COOLDOWN: f64 = 0.75;
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

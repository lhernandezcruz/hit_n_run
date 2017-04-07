# Hit and Run
Simple game made with Rust using piston. This game was written as a project for a class as a way to learn Rust.

# Build and Run
![Image of game](images/screenshot1.JPG?raw=true "Image of Hit and Run")

Windows 10: Easiest method is to just download the game from https://github.com/lhernandezcruz/hit_n_run/releases and run the executable.
If Rust and Cargo is installed then downloading the files and running the command 'cargo run' should run the game. 

Note: Game has only been tested on Windows 10.

# Gameplay
## Controls
* SPACE - Shoot a bullet
* Mouse - Player follows mouse
* R     - Reset the game

## Objective
The objective of the game is to kill as many enemies as possible without dying. Shooting an enemy adds one health point and killing an enemy adds one point to the score and current level kills. Enemies have their health displayed on them and shoot at intervals; they are also circled by orbs that do damage to the player. Getting shot will take one health away from the player, getting to 0 health means you have died an need restart the game. In order to move to the next level, the player must kill as many enemies as the current level. Upon moving levels the enemies reset and one new enemy will respawn. The player has a cooldown system for shooting. They player can shoot a certain amount before a longer cooldown must be waited out. 

GLHF

# Dependenciess
### Game engine dependencies
* piston = "0.31.1" 
* piston2d-graphics = "0.21.1"
* pistoncore-glutin_window = "0.35.0"
* piston2d-opengl_graphics = "0.40.0"

### RNG and Path dependencies
* rand = "0.3"
* find_folder = "0.3.0"

Run "cargo rustdoc -- --no-defaults --passes "collapse-docs" --passes "unindent-comments" --passes strip-priv-imports" in order to create the documentation.
# Authors
* Luis Hernandez Cruz - lhernandezcruz@g.hmc.edu

# Acknowledgments
Thank you to all other developers who have made games with piston your work was very helpfull in starting this game.

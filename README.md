# Hit and Run
Simple game made with Rust using piston. This game was written as a project for a class as a way to learn Rust.


# Build and Run
![Image of game](images/screenshot1.JPG?raw=true "Image of Hit and Run")
Assuming you have Rust and Cargo installed. Just download the files and run the command 'cargo run'. 

Note: Game has only been tested on Windows 10.

# Gameplay
Kill enemies to get points. Enemies have their health displayed on them. Kill as many enemies as the current level to move onto the next level.
* SPACE - Shoot a bullet
* Mouse - Player follows mouse
* R     - Reset the game

# Dependencies
### Game engine dependencies
* piston = "0.31.1" 
* piston2d-graphics = "0.21.1"
* pistoncore-glutin_window = "0.35.0"
* piston2d-opengl_graphics = "0.40.0"

### RNG and Path dependecies
* rand = "0.3"
* find_folder = "0.3.0"

# Authors
* Luis Hernandez Cruz

# Acknowledgments
Thank you to all other games that have been made with piston your work was very helpfull in starting this game.
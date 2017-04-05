/// Vector
pub struct Vector {
    /// X component
    pub x: f64,
    /// Y component
    pub y: f64,
}

impl Vector {
    /// Returns a vector
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x: x, y: y }
    }

    /// Resest the x and y components to 0
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    /// Returns the distance between two vector
    pub fn dist(&self, other: &Vector) -> f64 {
        let xdiff = self.x - other.x;
        let ydiff = self.y - other.y;
        (xdiff.powi(2) + ydiff.powi(2)).sqrt()
    }
}
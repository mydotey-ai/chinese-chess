#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    
    pub fn is_valid(&self) -> bool {
        self.x < 9 && self.y < 10
    }
}
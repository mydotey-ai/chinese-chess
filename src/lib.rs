pub mod board {
    pub mod coordinate;
    pub mod square;
    pub use modl::Board;
    pub mod modl;
}

pub mod pieces;

pub mod game {
    pub mod rules;
    pub mod state;
    pub use modl::Game;
    pub mod modl;
}

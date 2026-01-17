use crate::board::coordinate::Coordinate;
use crate::board::modl::Board;
use crate::board::square::Color;
use crate::pieces::Piece;

#[derive(Debug, Clone)]
pub struct King {
    color: Color,
}

impl Piece for King {
    fn new(color: Color) -> Self {
        Self { color }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, from: Coordinate, _board: &Board) -> Vec<Coordinate> {
        let mut moves = Vec::new();

        // King moves one square in any direction within palace
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dx, dy) in delta.iter() {
            let new_x = (from.x as isize + dx) as usize;
            let new_y = (from.y as isize + dy) as usize;
            let coord = Coordinate::new(new_x, new_y);

            if Self::is_in_palace(coord, self.color) && coord.is_valid() {
                moves.push(coord);
            }
        }

        moves
    }
}

impl King {
    fn is_in_palace(coord: Coordinate, color: Color) -> bool {
        let (x_valid, y_valid) = match color {
            Color::Red => (3 <= coord.x && coord.x <= 5, coord.y <= 2),
            Color::Black => (3 <= coord.x && coord.x <= 5, 7 <= coord.y && coord.y <= 9),
        };

        x_valid && y_valid
    }
}

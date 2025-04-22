use crate::grid::GridPosition;

#[derive(Clone, Copy, Debug)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetrominoType {
    pub fn shape(&self) -> &'static [GridPosition]{
        match self {
            TetrominoType::O => &[
                GridPosition{x: 0, y: 0},
                GridPosition{x: 1, y: 0},
                GridPosition{x: 0, y: 1},
                GridPosition{x: 1, y: 1},
            ],
            TetrominoType::I => &[
                GridPosition{x: 0, y: 0},
                GridPosition{x: 1, y: 0},
                GridPosition{x: 2, y: 0},
                GridPosition{x: 3, y: 0},
            ],
            TetrominoType::J => &[
                GridPosition{x: 0, y: 0},
                GridPosition{x: 1, y: 0},
                GridPosition{x: 2, y: 0},
                GridPosition{x: 2, y: 1},
            ],
            TetrominoType::L => &[
                GridPosition{x: 0, y: 0},
                GridPosition{x: 1, y: 0},
                GridPosition{x: 2, y: 0},
                GridPosition{x: 0, y: 1},
            ],
            TetrominoType::S => &[
                GridPosition{x: 0, y: 0},
                GridPosition{x: 1, y: 0},
                GridPosition{x: 1, y: 1},
                GridPosition{x: 2, y: 1},
            ],
            TetrominoType::T => &[
                GridPosition{x: 0, y: 0},
                GridPosition{x: 1, y: 0},
                GridPosition{x: 2, y: 0},
                GridPosition{x: 1, y: 1},
            ],
            TetrominoType::Z => &[
                GridPosition{x: 0, y: 0},
                GridPosition{x: 1, y: 0},
                GridPosition{x: 1, y: 1},
                GridPosition{x: 2, y: 1},
            ],
        }
    }
}
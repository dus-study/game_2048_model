mod no_empty_error;

use rand::prelude::*;
pub use no_empty_error::NoEmptyError;

pub const BOARD_SIZE: usize = 4;

pub type BoardElement = u8;

pub type ArrayBoard = [BoardElement; BOARD_SIZE * BOARD_SIZE];
pub type ArrayBoardIndex = [usize; BOARD_SIZE * BOARD_SIZE];

// The board is represented as a matrix defined as an array of arrays
pub type MatrixBoard = [[BoardElement; BOARD_SIZE]; BOARD_SIZE];

pub enum Directions {
    Up,
    Right,
    Down,
    Left,
}

pub trait Model: From<MatrixBoard> + From<ArrayBoard> {
    fn new() -> Self;

    fn slide(&mut self, direction: Directions);

    fn random<R: Rng>(&mut self, rng: &mut R) -> Result<(), NoEmptyError>;

    fn as_matrix(&self) -> MatrixBoard;

    fn as_array(&self) -> ArrayBoard;
}

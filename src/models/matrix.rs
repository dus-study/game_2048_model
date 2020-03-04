#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use rand::prelude::*;

use crate::base::*;

/// Implements the 2048 game model with the board defined as an array of arrays 
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    board: MatrixBoard,
}

impl Matrix {
    fn move_left(&mut self) {
        for row in 0..4 {
            let mut first_empty: Option<usize> = None;
            let mut potential_merge: Option<usize> = None;
            for col in 0..4 {
                let value = self.board[row][col];
    
                if let Some(p_ind) = potential_merge {
                    let p_value = self.board[row][p_ind];
                    if p_value == value {
                        self.board[row][p_ind] += value;
                        self.board[row][col] = 0;
                        first_empty = Some(col);
                    }
                }
    
                let value = self.board[row][col];
    
                if value == 0 && first_empty == None {
                    first_empty = Some(col);
                } else if value != 0 {
                    if let Some(target) = first_empty {
                        self.board[row][target] = value;
                        self.board[row][col] = 0;
                        first_empty = Some(target + 1);
                        potential_merge = Some(target);
                    } else {
                        potential_merge = Some(col);
                    }
                }
            }
        }
    }
}

impl From<MatrixBoard> for Matrix {
    /// ```
    /// use game_2048_model::models::{Model, Matrix};
    /// 
    /// let input = [
    ///  [0,1,1,0],
    ///  [1,2,2,1],
    ///  [1,2,2,1],
    ///  [0,1,1,0]
    /// ];
    /// 
    /// let game = Matrix::from(input);
    /// 
    /// assert_eq!(game.as_matrix(), input);
    /// ```
    /// 
    fn from(board: MatrixBoard) -> Self {
        Matrix {
            board: board
        }
    }
}

impl From<ArrayBoard> for Matrix {
    /// ```
    /// use game_2048_model::models::{Model, Matrix};
    /// 
    /// let input = [
    ///     0,1,1,0,
    ///     1,2,2,1,
    ///     1,2,2,1,
    ///     0,1,1,0
    /// ];
    /// 
    /// let game = Matrix::from(input);
    /// 
    /// assert_eq!(game.as_array(), input);
    /// ```
    /// 
    fn from(board: ArrayBoard) -> Self {
        // TODO: Convert to macro
        Matrix {
            board: [
                [board[0], board[1], board[2], board[3]],
                [board[4], board[5], board[6], board[7]],
                [board[8], board[9], board[10], board[11]],
                [board[12], board[13], board[14], board[15]]
            ]
        }
    }
}

impl Model for Matrix {
    /// Create a new instance of the game board filled with zeros
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048_model::models::{Model, Matrix};
    /// 
    /// let game = Matrix::new();
    /// ```
    /// 
    fn new() -> Matrix {
        Matrix {
            board: [[0; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    // / Slides all non-empty elements towards the choosen direction
    // / 
    // / # Examples
    // / 
    // / ```
    // / use game_2048_model::prelude::*;
    // / use rand::thread_rng;
    // / 
    // / let mut game = Matrix::new();
    // / game.from_array([
    // /     4,0,4,2,
    // /     0,0,1,1,
    // /     4,8,8,16,
    // /     2,2,2,2
    // / ]);
    // / game.slide(Directions::Left);
    // / 
    // / assert_eq!(game.to_array(), [
    // /     8,2,0,0,
    // /     2,0,0,0,
    // /     4,16,16,0,
    // /     4,4,0,0
    // / ]);
    // / ```
    // / 
    fn slide(&mut self, direction: Directions) {
        match direction {
            Directions::Up => (), // TODO: Implement this
            Directions::Right => (),
            Directions::Down => (),
            Directions::Left => (),
        }
    }

    fn random<R: Rng>(&mut self, rng: &mut R) {
        
    }

    /// Converts the game model to a matrix as an array of arrays
    /// 
    /// ```
    /// use game_2048_model::models::{Model, Matrix};
    /// 
    /// let input = [
    ///  [0,1,1,0],
    ///  [1,2,2,1],
    ///  [1,2,2,1],
    ///  [0,1,1,0]
    /// ];
    /// 
    /// let game = Matrix::from(input);
    /// 
    /// assert_eq!(game.as_matrix(), input);
    /// ```
    /// 
    fn as_matrix(&self) -> MatrixBoard {
        self.board
    }

    /// Converts the game model to an array
    /// 
    /// ```
    /// use game_2048_model::models::{Model, Matrix};
    /// 
    /// let input = [
    ///     0,1,1,0,
    ///     1,2,2,1,
    ///     1,2,2,1,
    ///     0,1,1,0
    /// ];
    /// 
    /// let game = Matrix::from(input);
    /// 
    /// assert_eq!(game.as_array(), input);
    /// ```
    /// 
    fn as_array(&self) -> ArrayBoard {
        // TODO: Convert to macro
        [
            self.board[0][0], self.board[0][1], self.board[0][2], self.board[0][3],
            self.board[1][0], self.board[1][1], self.board[1][2], self.board[1][3],
            self.board[2][0], self.board[2][1], self.board[2][2], self.board[2][3],
            self.board[3][0], self.board[3][1], self.board[3][2], self.board[3][3]
        ]
    }
}
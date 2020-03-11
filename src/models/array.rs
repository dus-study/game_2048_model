#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use rand::prelude::*;

use crate::base::*;

/// Implements the 2048 game model with the board defined as an array 
#[derive(Debug, Copy, Clone)]
pub struct ArrayModel {
    board: ArrayBoard,
}

const UP_INDEX: ArrayBoardIndex = [
    0, 4, 8, 12,
    1, 5, 9, 13,
    2, 6, 10, 14,
    3, 7, 11, 15
];

const RIGHT_INDEX: ArrayBoardIndex = [
    3, 2, 1, 0,
    7, 6, 5, 4,
    11, 10, 9, 8,
    15, 14, 13, 12
];

const DOWN_INDEX: ArrayBoardIndex = [
    12, 8, 4, 0,
    13, 9, 5, 1,
    14, 10, 6, 2,
    15, 11, 7, 3
];

pub const LEFT_INDEX: ArrayBoardIndex = [
    0, 1, 2, 3,
    4, 5, 6, 7,
    8, 9, 10, 11,
    12, 13, 14, 15
];

impl ArrayModel {
    /// Used to shift non-empty elements towards one of the four sides.
    /// 
    /// This is a private method not intended to be used directly.
    /// The method allways shifts towards the left, the index defines what
    /// the method considers left.
    /// 
    /// # Arguments
    /// 
    /// * `array` - The board to shift
    /// * `index` - Defines in what direction the method acts.
    /// 
    fn shift(array: &mut ArrayBoard, index: ArrayBoardIndex) {
        for outer_i in (0..16).step_by(4) {
            let mut movable: Option<usize> = None;
            for inner_i in outer_i..(outer_i + 4) {
                let ind = index[inner_i];
                let value = array[ind as usize];
                if let Some(move_to) = movable {
                    if value != 0 && inner_i != move_to {
                        array[index[move_to]] = value;
                        array[ind] = 0;
                        movable = Some(move_to + 1);
                    }
                } else if value == 0 {
                    movable = Some(inner_i);
                }
            }
        }
    }

    /// Used to merge elements towards one of the four sides.
    /// 
    /// This is a private method not intended to be used directly.
    /// The method allways merge towards the left, the index defines what
    /// the method considers left.
    /// 
    /// # Arguments
    /// 
    /// * `array` - The board to shift
    /// * `index` - Defines in what direction the method acts.
    /// 
    fn merge(array: &mut ArrayBoard, index: ArrayBoardIndex) {
        for outer_i in (0..16).step_by(4) {
            let mut mergeable: Option<usize> = None;
            for inner_i in outer_i..(outer_i + 4) {
                let ind = index[inner_i];
                let value = array[ind as usize];
                
                if value == 0 {
                    break;
                }
                
                if let Some(merge_to) = mergeable {
                    let prev_ind = index[merge_to];
                    let prev_value = array[prev_ind];
                    
                    if value == prev_value && merge_to + 1 == inner_i {
                        array[prev_ind] += value;
                        array[ind] = 0;
                        mergeable = None;
                    } else {
                        mergeable = Some(inner_i);
                    }
                } else {
                    mergeable = Some(inner_i);
                }
            }
        }
    }

    // TODO: check if change has occured
}

impl From<MatrixBoard> for ArrayModel {
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
        // TODO: Implement macro
        ArrayModel {
            board: [
                board[0][0], board[0][1], board[0][2], board[0][3],
                board[1][0], board[1][1], board[1][2], board[1][3],
                board[2][0], board[2][1], board[2][2], board[2][3],
                board[3][0], board[3][1], board[3][2], board[3][3]
            ]
        }
    }
}

impl From<ArrayBoard> for ArrayModel {
    /// Sets the board state based on the given array
    /// 
    /// # Examples
    /// ```
    /// use game_2048_model::models::{Model, ArrayModel};
    /// 
    /// let input = [
    ///     0,1,1,0,
    ///     1,2,2,1,
    ///     1,2,2,1,
    ///     0,1,1,0
    /// ];
    /// 
    /// let game = ArrayModel::from(input);
    /// 
    /// assert_eq!(game.as_array(), input);
    /// ```
    /// 
    fn from(board: ArrayBoard) -> Self {
        ArrayModel {
            board: board
        }
    }
}

impl Model for ArrayModel {
    /// Create a new instance of the game board filled with zeros
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048_model::models::{Model, ArrayModel};
    /// 
    /// let game = ArrayModel::new();
    /// ```
    /// 
    fn new() -> ArrayModel {
        ArrayModel {
            board: [0; BOARD_SIZE * BOARD_SIZE],
        }
    }

    /// Slide and merge the numbers towards a direction
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048_model::models::{ArrayModel, Directions, Model};
    /// use rand::thread_rng;
    /// 
    /// let mut game = ArrayModel::from([
    ///     2,1,16,2,
    ///     4,1,8,2,
    ///     0,0,8,2,
    ///     4,0,4,2
    /// ]);
    /// game.slide(Directions::Down);
    /// 
    /// assert_eq!(game.as_array(), [
    ///     0,0,0,0,
    ///     0,0,16,0,
    ///     2,0,16,4,
    ///     8,2,4,4
    /// ]);
    /// ```
    ///
    fn slide(&mut self, direction: Directions) {
        match direction {
            Directions::Up => {
                ArrayModel::shift(&mut self.board, UP_INDEX);
                ArrayModel::merge(&mut self.board, UP_INDEX);
                ArrayModel::shift(&mut self.board, UP_INDEX);
            },
            Directions::Right => {
                ArrayModel::shift(&mut self.board, RIGHT_INDEX);
                ArrayModel::merge(&mut self.board, RIGHT_INDEX);
                ArrayModel::shift(&mut self.board, RIGHT_INDEX);
            },
            Directions::Down => {
                ArrayModel::shift(&mut self.board, DOWN_INDEX);
                ArrayModel::merge(&mut self.board, DOWN_INDEX);
                ArrayModel::shift(&mut self.board, DOWN_INDEX);
            },
            Directions::Left => {
                ArrayModel::shift(&mut self.board, LEFT_INDEX);
                ArrayModel::merge(&mut self.board, LEFT_INDEX);
                ArrayModel::shift(&mut self.board, LEFT_INDEX);
            },
        }
    }

    /// Add a number to a random empty square.
    /// 
    /// A square is considered empty if it contains a 0.
    /// There is a 90% chance of the number added being a 2 and a 10% chance of it being a 4.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048_model::models::{Model, ArrayModel};
    /// use rand::thread_rng;
    /// 
    /// let mut game = ArrayModel::new();
    /// let mut rng = thread_rng();
    /// game.random(&mut rng);
    /// ```
    /// 
    fn random<R: Rng>(&mut self, rng: &mut R) {
        let square = self.board.choose_weighted_mut(rng, |item| if *item == 0 { 1 } else { 0 }).unwrap();
        let new_value = if rng.gen_range(0, 10) > 8 { 4 } else { 2 };
        *square = new_value;
    }

    /// Converts the game model to a matrix as an array of arrays
    /// 
    /// ```
    /// use game_2048_model::models::{Model, ArrayModel};
    /// 
    /// let input = [
    ///  [0,1,1,0],
    ///  [1,2,2,1],
    ///  [1,2,2,1],
    ///  [0,1,1,0]
    /// ];
    /// 
    /// let game = ArrayModel::from(input);
    /// 
    /// assert_eq!(game.as_matrix(), input);
    /// ```
    /// 
    fn as_matrix(&self) -> MatrixBoard {
        // TODO: Convert to macro
        [
            [self.board[0], self.board[1], self.board[2], self.board[3]],
            [self.board[4], self.board[5], self.board[6], self.board[7]],
            [self.board[8], self.board[9], self.board[10], self.board[11]],
            [self.board[12], self.board[13], self.board[14], self.board[15]]
        ]
    }

    /// Returns the board in array form
    /// 
    /// # Examples
    /// ```
    /// use game_2048_model::models::{Model, ArrayModel};
    /// 
    /// let input = [
    ///     0,1,1,0,
    ///     1,2,2,1,
    ///     1,2,2,1,
    ///     0,1,1,0
    /// ];
    /// 
    /// let game = ArrayModel::from(input);
    /// 
    /// assert_eq!(game.as_array(), input);
    /// ```
    /// 
    fn as_array(&self) -> ArrayBoard {
        self.board
    }
}

#[cfg(test)]
mod tests {
    use super::{ArrayModel, Directions, Model};

    mod new {
        use super::{ArrayModel, Model};

        #[test]
        fn initalize_with_board_empty() {
            let game = ArrayModel::new();
            assert_eq!(game.as_array(), [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }
    }

    mod random {
        use super::{ArrayModel, Model};
        use rand::rngs::mock::StepRng;
        use rand::{SeedableRng};
        use rand::rngs::StdRng;

        #[test]
        fn updates_a_zero_square() {
            let mut game = ArrayModel::new();
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            game.random(&mut rng);
            assert_eq!(game.as_array(), [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }

        #[test]
        fn ignores_non_zero_squares() {            
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            let mut game = ArrayModel::from([64,32,16,8,0,0,0,0,0,0,0,0,0,0,0,0]);
            game.random(&mut rng);
            assert_eq!(game.as_array(), [64,32,16,8,2,0,0,0,0,0,0,0,0,0,0,0]);
        }

        #[test]
        fn sets_2_with_90_procent_chans() {
            let mut game = ArrayModel::new();
            let seed = [
                64, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ];
            let mut rng: StdRng = SeedableRng::from_seed(seed);
            game.random(&mut rng);
            assert_eq!(game.as_array(), [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }

        #[test]
        fn sets_4_with_10_procent_chance() {
            let mut game = ArrayModel::new();
            // This seed causes the fake randomness to repeatedly fulfil this test,
            // that is set a 4 in the first element in the array by randomly generating a 9.
            let seed = [
                15, 118, 207, 76, 243, 48, 181, 38,
                199, 222, 147, 175, 48, 222, 181, 31,
                31, 65, 195, 28, 223, 56, 54, 166,
                169, 133, 246, 52, 86, 197, 228, 114
            ];
            let mut rng: StdRng = SeedableRng::from_seed(seed);
            game.random(&mut rng);
            assert_eq!(game.as_array(), [4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }
    }

    mod slide_up {
        use super::{ArrayModel, Directions, Model};

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::from([
                2,4,8,0,
                2,0,0,0,
                0,4,0,0,
                0,0,8,0
            ]);
            
            let expected = [
                4,8,16,0,
                0,0,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);
            
            assert_eq!(game.as_array(), expected, "Did not properly join equal squares");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::from([
                4,2,0,0,
                4,2,0,0,
                2,2,0,0,
                2,2,0,0
            ]);
            
            let expected = [
                8,4,0,0,
                4,4,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);
            
            assert_eq!(game.as_array(), expected, "Did not properly join multiple same row equal squares");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::from([
                2,4,8,0,
                4,0,0,0,
                0,8,0,0,
                0,0,16,0
            ]);
            
            let expected = [
                2,4,8,0,
                4,8,16,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);
            
            assert_eq!(game.as_array(), expected, "Joined unequal squares");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::from([
                2,2,4,0,
                2,2,2,0,
                2,4,2,0,
                2,0,0,0
            ]);
            
            let expected = [
                4,4,4,0,
                4,4,4,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);
            
            assert_eq!(game.as_array(), expected, "Joined multiple times.");
        }
    }

    mod move_right {
        use super::{ArrayModel, Directions, Model};

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::from([
                0,0,2,2,
                0,4,0,4,
                8,0,0,8,
                0,0,0,0
            ]);
            
            let expected = [
                0,0,0,4,
                0,0,0,8,
                0,0,0,16,
                0,0,0,0
            ];

            game.slide(Directions::Right);
            
            assert_eq!(game.as_array(), expected, "Did not properly join equal squares");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::from([
                2,2,4,4,
                2,2,2,2,
                0,0,0,0,
                0,0,0,0
            ]);
            
            let expected = [
                0,0,4,8,
                0,0,4,4,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Right);
            
            assert_eq!(game.as_array(), expected, "Did not properly join multiple same row equal squares");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::from([
                0,0,4,2,
                0,8,0,4,
                16,0,0,8,
                0,0,0,0
            ]);
            
            let expected = [
                0,0,4,2,
                0,0,8,4,
                0,0,16,8,
                0,0,0,0
            ];

            game.slide(Directions::Right);
            
            assert_eq!(game.as_array(), expected, "Joined unequal squares");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::from([
                2,2,2,2,
                0,4,2,2,
                0,2,2,4,
                0,0,0,0
            ]);
            
            let expected = [
                0,0,4,4,
                0,0,4,4,
                0,0,4,4,
                0,0,0,0
            ];

            game.slide(Directions::Right);
            
            assert_eq!(game.as_array(), expected, "Joined multiple times.");
        }
    }

    mod slide_down {
        use super::{ArrayModel, Directions, Model};

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::from([
                0,0,8,0,
                0,4,0,0,
                2,0,0,0,
                2,4,8,0
            ]);
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                0,0,0,0,
                4,8,16,0
            ];

            game.slide(Directions::Down);
            
            assert_eq!(game.as_array(), expected, "Did not properly join equal squares");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::from([
                2,2,0,0,
                2,2,0,0,
                4,2,0,0,
                4,2,0,0
            ]);
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                4,4,0,0,
                8,4,0,0
            ];

            game.slide(Directions::Down);
            
            assert_eq!(game.as_array(), expected, "Did not properly join multiple same row equal squares");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::from([
                0,0,16,0,
                0,8,0,0,
                4,0,0,0,
                2,4,8,0
            ]);
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                4,8,16,0,
                2,4,8,0
            ];

            game.slide(Directions::Down);
            
            assert_eq!(game.as_array(), expected, "Joined unequal squares");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::from([
                2,0,0,0,
                2,4,2,0,
                2,2,2,0,
                2,2,4,0
            ]);
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                4,4,4,0,
                4,4,4,0
            ];

            game.slide(Directions::Down);
            
            assert_eq!(game.as_array(), expected, "Joined multiple times.");
        }
    }

    mod slide_left {
        use super::{ArrayModel, Directions, Model};

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::from([
                2,2,0,0,
                4,0,4,0,
                8,0,0,8,
                0,0,0,0
            ]);
            
            let expected = [
                4,0,0,0,
                8,0,0,0,
                16,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);
            
            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Did not properly join equal squares. (0 square gap)");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Did not properly join equal squares. (1 square gap)");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Did not properly join equal squares. (2 square gap)");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::from([
                4,4,2,2,
                2,2,2,2,
                0,0,0,0,
                0,0,0,0
            ]);
            
            let expected = [
                8,4,0,0,
                4,4,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);
            
            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Did not properly join multiple same row equal squares. (Two distinct pairs)");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Did not properly join multiple same row equal squares. (Two identical pairs)");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Unexpected square modification");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::from([
                2,4,0,0,
                4,0,8,0,
                8,0,0,16,
                0,0,0,0
            ]);
            
            let expected = [
                2,4,0,0,
                4,8,0,0,
                8,16,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);
            
            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Joined unequal squares. (0 square gap)");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Joined unequal squares. (1 square gap)");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Joined unequal squares. (2 square gap)");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::from([
                2,2,2,2,
                2,2,4,0,
                4,2,2,0,
                0,0,0,0
            ]);
            
            let expected = [
                4,4,0,0,
                4,4,0,0,
                4,4,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);
            
            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Joined multiple times.");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Joined multiple times.");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Joined multiple times.");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }
    }
}
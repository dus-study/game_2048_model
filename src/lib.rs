#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
//! This crate implements different data models for the game 2048

use rand::prelude::*;

// Type of the board elements
type BoardElement = u8;

// The board is represented as an array
type ArrayBoard = [BoardElement; 16];
type ArrayBoardIndex = [usize; 16];

/// Implements the 2048 game model with the board defined as an array 
#[derive(Debug)]
pub struct ArrayModel {
    board: ArrayBoard,
}

impl ArrayModel {
    /// Create a new instance of the game board filled with zeros
    ///
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// 
    /// let game = ArrayModel::new();
    /// ```
    /// 
    pub fn new() -> ArrayModel {
        ArrayModel {
            board: [0; 16]
        }
    }

    /// Sets the board state based on the given array
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// 
    /// let mut game = ArrayModel::new();
    /// let array = [
    ///     1,2,2,1,
    ///     2,4,4,2,
    ///     2,4,4,2,
    ///     1,2,2,1
    /// ];
    /// game.from_array(array);
    /// assert_eq!(game.to_array(), array);
    /// ```
    /// 
    pub fn from_array(&mut self, input: ArrayBoard) {
        self.board = input;
    }

    /// Returns the board in array form
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// 
    /// let mut game = ArrayModel::new();
    /// let array = [
    ///     1,2,2,1,
    ///     2,4,4,2,
    ///     2,4,4,2,
    ///     1,2,2,1
    /// ];
    /// game.from_array(array);
    /// assert_eq!(game.to_array(), array);
    /// ```
    /// 
    pub fn to_array(self) -> ArrayBoard {
        self.board
    } 

    /// Add a number to a random empty square.
    /// 
    /// A square is considered empty if it contains a 0.
    /// There is a 90% chance of the number added being a 2 and a 10% chance of it being a 4.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// use rand::thread_rng;
    /// 
    /// let mut game = ArrayModel::new();
    /// let mut rng = thread_rng();
    /// game.add_to_random_empty(&mut rng);
    /// ```
    /// 
    pub fn add_to_random_empty<R: Rng>(&mut self, rng: &mut R) {
        let square = self.board.choose_weighted_mut(rng, |item| if *item == 0 { 1 } else { 0 }).unwrap();
        let new_value = if rng.gen_range(0, 10) > 5 { 4 } else { 2 };
        *square = new_value;
    }

    /// Move and merge numbers towards the left
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// use rand::thread_rng;
    /// 
    /// let mut game = ArrayModel::new();
    /// game.from_array([
    ///     4,0,4,2,
    ///     0,0,1,1,
    ///     4,8,8,16,
    ///     2,2,2,2
    /// ]);
    /// game.move_left();
    /// 
    /// assert_eq!(game.to_array(), [
    ///     8,2,0,0,
    ///     2,0,0,0,
    ///     4,16,16,0,
    ///     4,4,0,0
    /// ]);
    /// ```
    /// 
    pub fn move_left(&mut self) {
        let index = [
            0, 1, 2, 3,
            4, 5, 6, 7,
            8, 9, 10, 11,
            12, 13, 14, 15
        ];
        ArrayModel::shift(&mut self.board, index);
        ArrayModel::merge(&mut self.board, index);
        ArrayModel::shift(&mut self.board, index);
    }

    /// Move and merge numbers towards the right
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// use rand::thread_rng;
    /// 
    /// let mut game = ArrayModel::new();
    /// game.from_array([
    ///     2,4,0,4,
    ///     1,1,0,0,
    ///     16,8,8,4,
    ///     2,2,2,2
    /// ]);
    /// game.move_right();
    /// 
    /// assert_eq!(game.to_array(), [
    ///     0,0,2,8,
    ///     0,0,0,2,
    ///     0,16,16,4,
    ///     0,0,4,4
    /// ]);
    /// ```
    /// 
    pub fn move_right(&mut self) {
        let index = [
            3, 2, 1, 0,
            7, 6, 5, 4,
            11, 10, 9, 8,
            15, 14, 13, 12
        ];
        ArrayModel::shift(&mut self.board, index);
        ArrayModel::merge(&mut self.board, index);
        ArrayModel::shift(&mut self.board, index);
    }

    /// Move and merge numbers towards the top
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// use rand::thread_rng;
    /// 
    /// let mut game = ArrayModel::new();
    /// game.from_array([
    ///     4,0,4,2,
    ///     0,0,8,2,
    ///     4,1,8,2,
    ///     2,1,16,2
    /// ]);
    /// game.move_up();
    /// 
    /// assert_eq!(game.to_array(), [
    ///     8,2,4,4,
    ///     2,0,16,4,
    ///     0,0,16,0,
    ///     0,0,0,0
    /// ]);
    /// ```
    /// 
    pub fn move_up(&mut self) {
        let index = [
            0, 4, 8, 12,
            1, 5, 9, 13,
            2, 6, 10, 14,
            3, 7, 11, 15
        ];
        ArrayModel::shift(&mut self.board, index);
        ArrayModel::merge(&mut self.board, index);
        ArrayModel::shift(&mut self.board, index);
    }

    /// Move and merge numbers towards the top
    /// 
    /// # Examples
    /// 
    /// ```
    /// use game_2048::ArrayModel;
    /// use rand::thread_rng;
    /// 
    /// let mut game = ArrayModel::new();
    /// game.from_array([
    ///     2,1,16,2,
    ///     4,1,8,2,
    ///     0,0,8,2,
    ///     4,0,4,2
    /// ]);
    /// game.move_down();
    /// 
    /// assert_eq!(game.to_array(), [
    ///     0,0,0,0,
    ///     0,0,16,0,
    ///     2,0,16,4,
    ///     8,2,4,4
    /// ]);
    /// ```
    ///
    pub fn move_down(&mut self) {
        let index = [
            12, 8, 4, 0,
            13, 9, 5, 1,
            14, 10, 6, 2,
            15, 11, 7, 3
        ];
        ArrayModel::shift(&mut self.board, index);
        ArrayModel::merge(&mut self.board, index);
        ArrayModel::shift(&mut self.board, index);
    }
    
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

#[cfg(test)]
mod tests {
    use super::ArrayModel;

    #[cfg(test)]
    mod new {
        use super::ArrayModel;

        #[test]
        fn initalize_with_board_empty() {
            let game = ArrayModel::new();
            assert_eq!(game.board, [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }
    }

    #[cfg(test)]
    mod add_to_random_empty {
        use super::ArrayModel;
        use rand::rngs::mock::StepRng;
        use rand::{SeedableRng};
        use rand::rngs::StdRng;

        #[test]
        fn updates_a_zero_square() {
            let mut game = ArrayModel::new();
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            game.add_to_random_empty(&mut rng);
            assert_eq!(game.board, [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }

        #[test]
        fn ignores_non_zero_squares() {
            let mut game = ArrayModel::new();
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            game.board = [64,32,16,8,0,0,0,0,0,0,0,0,0,0,0,0];
            game.add_to_random_empty(&mut rng);
            assert_eq!(game.board, [64,32,16,8,2,0,0,0,0,0,0,0,0,0,0,0]);
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
            game.add_to_random_empty(&mut rng);
            assert_eq!(game.board, [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }

        #[test]
        fn sets_4_with_10_procent_chans() {
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
            game.add_to_random_empty(&mut rng);
            assert_eq!(game.board, [4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }
    }

    #[cfg(test)]
    mod move_left {
        use super::ArrayModel;

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,2,0,0,
                4,0,4,0,
                8,0,0,8,
                0,0,0,0
            ];
            
            let expected = [
                4,0,0,0,
                8,0,0,0,
                16,0,0,0,
                0,0,0,0
            ];

            game.move_left();
            
            assert_eq!(game.board[0 .. 4], expected[0 .. 4], "Did not properly join equal squares. (0 square gap)");
            assert_eq!(game.board[4 .. 8], expected[4 .. 8], "Did not properly join equal squares. (1 square gap)");
            assert_eq!(game.board[8 .. 12], expected[8 .. 12], "Did not properly join equal squares. (2 square gap)");
            assert_eq!(game.board[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                4,4,2,2,
                2,2,2,2,
                0,0,0,0,
                0,0,0,0
            ];
            
            let expected = [
                8,4,0,0,
                4,4,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.move_left();
            
            assert_eq!(game.board[0 .. 4], expected[0 .. 4], "Did not properly join multiple same row equal squares. (Two distinct pairs)");
            assert_eq!(game.board[4 .. 8], expected[4 .. 8], "Did not properly join multiple same row equal squares. (Two identical pairs)");
            assert_eq!(game.board[8 .. 12], expected[8 .. 12], "Unexpected square modification");
            assert_eq!(game.board[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,4,0,0,
                4,0,8,0,
                8,0,0,16,
                0,0,0,0
            ];
            
            let expected = [
                2,4,0,0,
                4,8,0,0,
                8,16,0,0,
                0,0,0,0
            ];

            game.move_left();
            
            assert_eq!(game.board[0 .. 4], expected[0 .. 4], "Joined unequal squares. (0 square gap)");
            assert_eq!(game.board[4 .. 8], expected[4 .. 8], "Joined unequal squares. (1 square gap)");
            assert_eq!(game.board[8 .. 12], expected[8 .. 12], "Joined unequal squares. (2 square gap)");
            assert_eq!(game.board[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,2,2,2,
                2,2,4,0,
                4,2,2,0,
                0,0,0,0
            ];
            
            let expected = [
                4,4,0,0,
                4,4,0,0,
                4,4,0,0,
                0,0,0,0
            ];

            game.move_left();
            
            assert_eq!(game.board[0 .. 4], expected[0 .. 4], "Joined multiple times.");
            assert_eq!(game.board[4 .. 8], expected[4 .. 8], "Joined multiple times.");
            assert_eq!(game.board[8 .. 12], expected[8 .. 12], "Joined multiple times.");
            assert_eq!(game.board[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }
    }

    #[cfg(test)]
    mod move_right {
        use super::ArrayModel;

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                0,0,2,2,
                0,4,0,4,
                8,0,0,8,
                0,0,0,0
            ];
            
            let expected = [
                0,0,0,4,
                0,0,0,8,
                0,0,0,16,
                0,0,0,0
            ];

            game.move_right();
            
            assert_eq!(game.board, expected, "Did not properly join equal squares");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,2,4,4,
                2,2,2,2,
                0,0,0,0,
                0,0,0,0
            ];
            
            let expected = [
                0,0,4,8,
                0,0,4,4,
                0,0,0,0,
                0,0,0,0
            ];

            game.move_right();
            
            assert_eq!(game.board, expected, "Did not properly join multiple same row equal squares");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                0,0,4,2,
                0,8,0,4,
                16,0,0,8,
                0,0,0,0
            ];
            
            let expected = [
                0,0,4,2,
                0,0,8,4,
                0,0,16,8,
                0,0,0,0
            ];

            game.move_right();
            
            assert_eq!(game.board, expected, "Joined unequal squares");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,2,2,2,
                0,4,2,2,
                0,2,2,4,
                0,0,0,0
            ];
            
            let expected = [
                0,0,4,4,
                0,0,4,4,
                0,0,4,4,
                0,0,0,0
            ];

            game.move_right();
            
            assert_eq!(game.board, expected, "Joined multiple times.");
        }
    }

    #[cfg(test)]
    mod move_up {
        use super::ArrayModel;

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,4,8,0,
                2,0,0,0,
                0,4,0,0,
                0,0,8,0
            ];
            
            let expected = [
                4,8,16,0,
                0,0,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.move_up();
            
            assert_eq!(game.board, expected, "Did not properly join equal squares");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                4,2,0,0,
                4,2,0,0,
                2,2,0,0,
                2,2,0,0
            ];
            
            let expected = [
                8,4,0,0,
                4,4,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.move_up();
            
            assert_eq!(game.board, expected, "Did not properly join multiple same row equal squares");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,4,8,0,
                4,0,0,0,
                0,8,0,0,
                0,0,16,0
            ];
            
            let expected = [
                2,4,8,0,
                4,8,16,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.move_up();
            
            assert_eq!(game.board, expected, "Joined unequal squares");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,2,4,0,
                2,2,2,0,
                2,4,2,0,
                2,0,0,0
            ];
            
            let expected = [
                4,4,4,0,
                4,4,4,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.move_up();
            
            assert_eq!(game.board, expected, "Joined multiple times.");
        }
    }

    #[cfg(test)]
    mod move_down {
        use super::ArrayModel;

        #[test]
        fn join_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                0,0,8,0,
                0,4,0,0,
                2,0,0,0,
                2,4,8,0
            ];
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                0,0,0,0,
                4,8,16,0
            ];

            game.move_down();
            
            assert_eq!(game.board, expected, "Did not properly join equal squares");
        }

        #[test]
        fn join_multiple_equal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,2,0,0,
                2,2,0,0,
                4,2,0,0,
                4,2,0,0
            ];
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                4,4,0,0,
                8,4,0,0
            ];

            game.move_down();
            
            assert_eq!(game.board, expected, "Did not properly join multiple same row equal squares");
        }

        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                0,0,16,0,
                0,8,0,0,
                4,0,0,0,
                2,4,8,0
            ];
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                4,8,16,0,
                2,4,8,0
            ];

            game.move_down();
            
            assert_eq!(game.board, expected, "Joined unequal squares");
        }

        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = ArrayModel::new();
            
            game.board = [
                2,0,0,0,
                2,4,2,0,
                2,2,2,0,
                2,2,4,0
            ];
            
            let expected = [
                0,0,0,0,
                0,0,0,0,
                4,4,4,0,
                4,4,4,0
            ];

            game.move_down();
            
            assert_eq!(game.board, expected, "Joined multiple times.");
        }
    }
}
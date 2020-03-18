#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use rand::prelude::*;

use crate::base::*;

/// Implements the 2048 game model with the board defined as an array of arrays
#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    board: MatrixBoard,
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
        Matrix { board: board }
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
                [board[12], board[13], board[14], board[15]],
            ],
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
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
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
    // /     2,0,2,1,
    // /     0,0,1,1,
    // /     2,3,3,4,
    // /     1,1,1,1
    // / ]);
    // / game.slide(Directions::Left);
    // /
    // / assert_eq!(game.to_array(), [
    // /     3,1,0,0,
    // /     1,0,0,0,
    // /     2,4,4,0,
    // /     2,2,0,0
    // / ]);
    // / ```
    // /
    fn slide(&mut self, direction: Directions) {
        match direction {
            Directions::Up => self.slide_up(),
            Directions::Right => self.slide_right(),
            Directions::Down => self.slide_down(),
            Directions::Left => self.slide_left(),
        }
    }

    fn random<R: Rng>(&mut self, rng: &mut R) -> Result<(), NoEmptyError> {
        let max: usize = self
            .as_array()
            .iter()
            .fold(0, |acc, x| acc + if *x == 0 { 1 } else { 0 });

        if max == 0 {
            return Err(NoEmptyError);
        }

        let ind: usize = rng.gen_range(0, max);

        let mut cur_ind = 0;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col] == 0 {
                    if cur_ind == ind {
                        self.board[row][col] = if rng.gen_range(0, 10) > 8 { 2 } else { 1 };
                        return Ok(());
                    } else {
                        cur_ind += 1;
                    }
                }
            }
        }

        Err(NoEmptyError)
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
            self.board[0][0],
            self.board[0][1],
            self.board[0][2],
            self.board[0][3],
            self.board[1][0],
            self.board[1][1],
            self.board[1][2],
            self.board[1][3],
            self.board[2][0],
            self.board[2][1],
            self.board[2][2],
            self.board[2][3],
            self.board[3][0],
            self.board[3][1],
            self.board[3][2],
            self.board[3][3],
        ]
    }
}

impl Matrix {
    fn slide_up(&mut self) {
        for col in 0..4 {
            let mut first_empty: Option<usize> = None;
            let mut potential_merge: Option<usize> = None;
            for row in 0..4 {
                let value = self.board[row][col];

                if let Some(p_ind) = potential_merge {
                    let p_value = self.board[p_ind][col];
                    if p_value == value {
                        self.board[p_ind][col] += 1;
                        self.board[row][col] = 0;
                        first_empty = Some(row);
                        potential_merge = None;
                    }
                }

                let value = self.board[row][col];

                if value == 0 && first_empty == None {
                    first_empty = Some(row);
                } else if value != 0 {
                    if let Some(target) = first_empty {
                        self.board[target][col] = value;
                        self.board[row][col] = 0;
                        first_empty = Some(target + 1);
                        potential_merge = Some(target);
                    } else {
                        potential_merge = Some(row);
                    }
                }
            }
        }
    }

    fn slide_right(&mut self) {
        for row in 0..4 {
            let mut first_empty: Option<usize> = None;
            let mut potential_merge: Option<usize> = None;
            for col in (0..4).rev() {
                let value = self.board[row][col];

                if let Some(p_ind) = potential_merge {
                    let p_value = self.board[row][p_ind];
                    if p_value == value {
                        self.board[row][p_ind] += 1;
                        self.board[row][col] = 0;
                        first_empty = Some(col);
                        potential_merge = None;
                    }
                }

                let value = self.board[row][col];

                if value == 0 && first_empty == None {
                    first_empty = Some(col);
                } else if value != 0 {
                    if let Some(target) = first_empty {
                        self.board[row][target] = value;
                        self.board[row][col] = 0;
                        first_empty = Some(target - 1);
                        potential_merge = Some(target);
                    } else {
                        potential_merge = Some(col);
                    }
                }
            }
        }
    }

    fn slide_down(&mut self) {
        for col in 0..4 {
            let mut first_empty: Option<usize> = None;
            let mut potential_merge: Option<usize> = None;
            for row in (0..4).rev() {
                let value = self.board[row][col];

                if let Some(p_ind) = potential_merge {
                    let p_value = self.board[p_ind][col];
                    if p_value == value {
                        self.board[p_ind][col] += 1;
                        self.board[row][col] = 0;
                        first_empty = Some(row);
                        potential_merge = None;
                    }
                }

                let value = self.board[row][col];

                if value == 0 && first_empty == None {
                    first_empty = Some(row);
                } else if value != 0 {
                    if let Some(target) = first_empty {
                        self.board[target][col] = value;
                        self.board[row][col] = 0;
                        first_empty = Some(target - 1);
                        potential_merge = Some(target);
                    } else {
                        potential_merge = Some(row);
                    }
                }
            }
        }
    }

    fn slide_left(&mut self) {
        for row in 0..4 {
            let mut first_empty: Option<usize> = None;
            let mut potential_merge: Option<usize> = None;
            for col in 0..4 {
                let value = self.board[row][col];

                if let Some(p_ind) = potential_merge {
                    let p_value = self.board[row][p_ind];
                    if p_value == value {
                        self.board[row][p_ind] += 1;
                        self.board[row][col] = 0;
                        first_empty = Some(col);
                        potential_merge = None;
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

#[cfg(test)]
mod tests {
    use super::{Directions, Matrix, Model};

    mod new {
        use super::{Matrix, Model};

        #[test]
        fn initalize_with_board_empty() {
            let game = Matrix::new();
            assert_eq!(
                game.as_array(),
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            );
        }
    }

    mod random {
        use super::{Matrix, Model};
        use rand::rngs::mock::StepRng;
        use rand::rngs::StdRng;
        use rand::SeedableRng;

        #[test]
        fn updates_a_zero_square() {
            let mut game = Matrix::new();
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            assert_eq!(game.random(&mut rng).is_ok(), true);
            assert_eq!(
                game.as_array(),
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            );
        }

        #[test]
        fn ignores_non_zero_squares() {
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            let mut game = Matrix::from([6, 5, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(game.random(&mut rng).is_ok(), true);
            assert_eq!(
                game.as_array(),
                [6, 5, 4, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            );
        }

        #[test]
        fn sets_1_with_90_procent_chans() {
            let mut game = Matrix::new();
            let seed = [
                64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ];
            let mut rng: StdRng = SeedableRng::from_seed(seed);
            assert_eq!(game.random(&mut rng).is_ok(), true);
            assert_eq!(
                game.as_array(),
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            );
        }

        #[ignore]
        #[test]
        fn sets_2_with_10_procent_chance() {
            unimplemented!();
            // TODO: test not working
            // let mut game = Matrix::new();
            // // This seed causes the fake randomness to repeatedly fulfil this test,
            // // that is set a 4 in the first element in the array by randomly generating a 9.
            // let seed = [
            //     15, 118, 207, 76, 243, 48, 181, 38,
            //     199, 222, 147, 175, 48, 222, 181, 31,
            //     31, 65, 195, 28, 223, 56, 54, 166,
            //     169, 133, 246, 52, 86, 197, 228, 114
            // ];
            // let mut rng: StdRng = SeedableRng::from_seed(seed);
            // assert_eq!(game.random(&mut rng).is_ok(), true);
            // assert_eq!(game.as_array(), [4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        }

        #[rustfmt::skip]
        #[test]
        fn returns_no_empty_error_on_full_board() {
            let mut game = Matrix::from([
                1,1,1,1,
                1,1,1,1,
                1,1,1,1,
                1,1,1,1
            ]);
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            assert_eq!(game.random(&mut rng).is_err(), true);
        }

        #[rustfmt::skip]
        #[test]
        fn no_changes_on_no_empty_error() {
            let mut game = Matrix::from([
                1,1,1,1,
                1,1,1,1,
                1,1,1,1,
                1,1,1,1
            ]);
            // TODO: Replace StepRng with StdRng and SeedableRng.
            let mut rng = StepRng::new(2, 1);
            assert_eq!(game.random(&mut rng).is_err(), true);
            assert_eq!(game.as_array(), [
                1,1,1,1,
                1,1,1,1,
                1,1,1,1,
                1,1,1,1
            ]);
        }
    }

    mod slide_up {
        use super::{Directions, Matrix, Model};

        #[rustfmt::skip]
        #[test]
        fn join_equal_squares() {
            let mut game = Matrix::from([
                1,2,3,0,
                1,0,0,0,
                0,2,0,0,
                0,0,3,0
            ]);

            let expected = [
                2,3,4,0,
                0,0,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);

            assert_eq!(game.as_array(), expected, "Did not properly join equal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn join_multiple_equal_squares() {
            let mut game = Matrix::from([
                2,1,0,0,
                2,1,0,0,
                1,1,0,0,
                1,1,0,0
            ]);

            let expected = [
                3,2,0,0,
                2,2,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);

            assert_eq!(game.as_array(), expected, "Did not properly join multiple same row equal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = Matrix::from([
                1,2,3,0,
                2,0,0,0,
                0,3,0,0,
                0,0,4,0
            ]);

            let expected = [
                1,2,3,0,
                2,3,4,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);

            assert_eq!(game.as_array(), expected, "Joined unequal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = Matrix::from([
                1,1,2,0,
                1,1,1,0,
                1,2,1,0,
                1,0,0,0
            ]);

            let expected = [
                2,2,2,0,
                2,2,2,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Up);

            assert_eq!(game.as_array(), expected, "Joined multiple times.");
        }
    }

    mod slide_right {
        use super::{Directions, Matrix, Model};

        #[rustfmt::skip]
        #[test]
        fn join_equal_squares() {
            let mut game = Matrix::from([
                0,0,1,1,
                0,2,0,2,
                3,0,0,3,
                0,0,0,0
            ]);

            let expected = [
                0,0,0,2,
                0,0,0,3,
                0,0,0,4,
                0,0,0,0
            ];

            game.slide(Directions::Right);

            assert_eq!(game.as_array(), expected, "Did not properly join equal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn join_multiple_equal_squares() {
            let mut game = Matrix::from([
                1,1,2,2,
                1,1,1,1,
                0,0,0,0,
                0,0,0,0
            ]);

            let expected = [
                0,0,2,3,
                0,0,2,2,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Right);

            assert_eq!(game.as_array(), expected, "Did not properly join multiple same row equal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = Matrix::from([
                0,0,2,1,
                0,3,0,2,
                4,0,0,3,
                0,0,0,0
            ]);

            let expected = [
                0,0,2,1,
                0,0,3,2,
                0,0,4,3,
                0,0,0,0
            ];

            game.slide(Directions::Right);

            assert_eq!(game.as_array(), expected, "Joined unequal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = Matrix::from([
                1,1,1,1,
                0,2,1,1,
                0,1,1,2,
                0,0,0,0
            ]);

            let expected = [
                0,0,2,2,
                0,0,2,2,
                0,0,2,2,
                0,0,0,0
            ];

            game.slide(Directions::Right);

            assert_eq!(game.as_array(), expected, "Joined multiple times.");
        }
    }

    mod slide_down {
        use super::{Directions, Matrix, Model};

        #[rustfmt::skip]
        #[test]
        fn join_equal_squares() {
            let mut game = Matrix::from([
                0,0,3,0,
                0,2,0,0,
                1,0,0,0,
                1,2,3,0
            ]);

            let expected = [
                0,0,0,0,
                0,0,0,0,
                0,0,0,0,
                2,3,4,0
            ];

            game.slide(Directions::Down);

            assert_eq!(game.as_array(), expected, "Did not properly join equal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn join_multiple_equal_squares() {
            let mut game = Matrix::from([
                1,1,0,0,
                1,1,0,0,
                2,1,0,0,
                2,1,0,0
            ]);

            let expected = [
                0,0,0,0,
                0,0,0,0,
                2,2,0,0,
                3,2,0,0
            ];

            game.slide(Directions::Down);

            assert_eq!(game.as_array(), expected, "Did not properly join multiple same row equal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = Matrix::from([
                0,0,4,0,
                0,3,0,0,
                2,0,0,0,
                1,2,3,0
            ]);

            let expected = [
                0,0,0,0,
                0,0,0,0,
                2,3,4,0,
                1,2,3,0
            ];

            game.slide(Directions::Down);

            assert_eq!(game.as_array(), expected, "Joined unequal squares");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = Matrix::from([
                1,0,0,0,
                1,2,1,0,
                1,1,1,0,
                1,1,2,0
            ]);

            let expected = [
                0,0,0,0,
                0,0,0,0,
                2,2,2,0,
                2,2,2,0
            ];

            game.slide(Directions::Down);

            assert_eq!(game.as_array(), expected, "Joined multiple times.");
        }
    }

    mod slide_left {
        use super::{Directions, Matrix, Model};

        #[rustfmt::skip]
        #[test]
        fn join_equal_squares() {
            let mut game = Matrix::from([
                1,1,0,0,
                2,0,2,0,
                3,0,0,3,
                0,0,0,0
            ]);

            let expected = [
                2,0,0,0,
                3,0,0,0,
                4,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);

            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Did not properly join equal squares. (0 square gap)");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Did not properly join equal squares. (1 square gap)");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Did not properly join equal squares. (2 square gap)");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[rustfmt::skip]
        #[test]
        fn join_multiple_equal_squares() {
            let mut game = Matrix::from([
                2,2,1,1,
                1,1,1,1,
                0,0,0,0,
                0,0,0,0
            ]);

            let expected = [
                3,2,0,0,
                2,2,0,0,
                0,0,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);

            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Did not properly join multiple same row equal squares. (Two distinct pairs)");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Did not properly join multiple same row equal squares. (Two identical pairs)");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Unexpected square modification");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_unequal_squares() {
            let mut game = Matrix::from([
                1,2,0,0,
                2,0,3,0,
                3,0,0,4,
                0,0,0,0
            ]);

            let expected = [
                1,2,0,0,
                2,3,0,0,
                3,4,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);

            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Joined unequal squares. (0 square gap)");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Joined unequal squares. (1 square gap)");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Joined unequal squares. (2 square gap)");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }

        #[rustfmt::skip]
        #[test]
        fn do_not_join_multiple_pairs_of_squares() {
            let mut game = Matrix::from([
                1,1,1,1,
                1,1,2,0,
                2,1,1,0,
                0,0,0,0
            ]);

            let expected = [
                2,2,0,0,
                2,2,0,0,
                2,2,0,0,
                0,0,0,0
            ];

            game.slide(Directions::Left);

            assert_eq!(game.as_array()[0 .. 4], expected[0 .. 4], "Joined multiple times. 1");
            assert_eq!(game.as_array()[4 .. 8], expected[4 .. 8], "Joined multiple times. 2");
            assert_eq!(game.as_array()[8 .. 12], expected[8 .. 12], "Joined multiple times. 3");
            assert_eq!(game.as_array()[12 .. 16], expected[12 .. 16], "Unexpected square modification");
        }
    }
}

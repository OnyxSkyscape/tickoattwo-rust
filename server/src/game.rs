#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Horizontal,
    Vertical,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FieldState {
    None,
    OccupiedByOne(Player),
    Both,
}

type Board = [[FieldState; 3]; 3];

#[derive(Debug)]
pub struct Game {
    board: Board,
    current_player: Player,
    previous_move: Option<(u8, u8)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[FieldState::None; 3]; 3],
            current_player: Player::Horizontal,
            previous_move: None,
        }
    }

    pub fn make_move(&mut self, coords: (u8, u8)) -> Result<(), &str> {
        // Out of bound check
        if coords.0 >= 3 || coords.1 >= 3 {
            return Err("Invalid coordinates");
        }

        // Check if field was occupied the in last round
        if self.previous_move.is_some() && coords == self.previous_move.unwrap() {
            return Err("Invalid move: placed in last round");
        }

        self.previous_move = Some(coords);

        let field = &mut self.board[coords.0 as usize][coords.1 as usize];

        // Try to occupy field
        *field = match field {
            FieldState::None => FieldState::OccupiedByOne(self.current_player),
            FieldState::OccupiedByOne(player) => {
                if *player == self.current_player {
                    return Err("Invalid move: already placed");
                }
                FieldState::Both
            }
            FieldState::Both => return Err("Invalid move"),
        };

        // Set next player
        self.current_player = match self.current_player {
            Player::Horizontal => Player::Vertical,
            Player::Vertical => Player::Horizontal,
        };

        Ok(())
    }

    pub fn check_win(&self) -> Option<Player> {
        // Check rows
        for row in self.board.iter() {
            if row.iter().all(|&x| x == FieldState::Both) {
                return Some(self.current_player);
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] == FieldState::Both
                && self.board[1][col] == FieldState::Both
                && self.board[2][col] == FieldState::Both
            {
                return Some(self.current_player);
            }
        }

        // Check diagonals
        if self.board[0][0] == FieldState::Both
            && self.board[1][1] == FieldState::Both
            && self.board[2][2] == FieldState::Both
            || self.board[0][2] == FieldState::Both
                && self.board[1][1] == FieldState::Both
                && self.board[2][0] == FieldState::Both
        {
            return Some(self.current_player);
        }

        // No winning combination found
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_win() {
        let mut game = Game::new();

        // Test no winner
        assert_eq!(game.check_win(), None);

        // Test horizontal win
        game.board = [
            [FieldState::Both, FieldState::Both, FieldState::Both],
            [FieldState::None, FieldState::None, FieldState::None],
            [FieldState::None, FieldState::None, FieldState::None],
        ];
        game.current_player = Player::Horizontal;
        assert_eq!(game.check_win(), Some(Player::Horizontal));

        // Test vertical win
        game.board = [
            [FieldState::None, FieldState::None, FieldState::None],
            [FieldState::Both, FieldState::Both, FieldState::Both],
            [FieldState::Both, FieldState::Both, FieldState::Both],
        ];
        game.current_player = Player::Vertical;
        assert_eq!(game.check_win(), Some(Player::Vertical));

        // Test diagonal win
        game.board = [
            [FieldState::Both, FieldState::None, FieldState::None],
            [FieldState::None, FieldState::Both, FieldState::None],
            [FieldState::None, FieldState::None, FieldState::Both],
        ];
        game.current_player = Player::Horizontal;
        assert_eq!(game.check_win(), Some(Player::Horizontal));
    }

    // Test invalid coordinates
    #[test]
    fn test_make_move_bounds() {
        let mut game = Game::new();

        assert_eq!(game.make_move((3, 0)), Err("Invalid coordinates"));
    }

    // Test invalid move: placed in last round
    #[test]
    fn test_make_move_last_round() {
        let mut game = Game::new();

        game.previous_move = Some((0u8, 0u8));
        assert_eq!(
            game.make_move((0, 0)),
            Err("Invalid move: placed in last round")
        );
    }

    // Test invalid move: already placed
    #[test]
    fn test_make_move_already_placed() {
        let mut game = Game::new();

        // Test valid move
        assert_eq!(game.make_move((0, 1)), Ok(()));
        assert_eq!(
            game.board[0][1],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test valid move
        assert_eq!(game.make_move((1, 1)), Ok(()));
        assert_eq!(
            game.board[1][1],
            FieldState::OccupiedByOne(Player::Vertical)
        );

        // Test invalid move: already placed
        assert_eq!(game.make_move((0, 1)), Err("Invalid move: already placed"));
    }

    // Test valid move
    #[test]
    fn test_make_move_valid() {
        let mut game = Game::new();

        // Test valid move
        assert_eq!(game.make_move((0, 1)), Ok(()));
        assert_eq!(
            game.board[0][1],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test valid move
        assert_eq!(game.make_move((1, 1)), Ok(()));
        assert_eq!(
            game.board[1][1],
            FieldState::OccupiedByOne(Player::Vertical)
        );

        // Test valid move
        assert_eq!(game.make_move((1, 2)), Ok(()));
        assert_eq!(
            game.board[1][2],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test valid move
        assert_eq!(game.make_move((2, 2)), Ok(()));
        assert_eq!(
            game.board[2][2],
            FieldState::OccupiedByOne(Player::Vertical)
        );
    }

    // Test valid move
    #[test]
    fn test_make_move_valid_both() {
        let mut game = Game::new();

        // Test valid move
        assert_eq!(game.make_move((0, 1)), Ok(()));
        assert_eq!(
            game.board[0][1],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test valid move
        assert_eq!(game.make_move((1, 1)), Ok(()));
        assert_eq!(
            game.board[1][1],
            FieldState::OccupiedByOne(Player::Vertical)
        );

        // Test valid move
        assert_eq!(game.make_move((1, 2)), Ok(()));
        assert_eq!(
            game.board[1][2],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test valid move both
        assert_eq!(game.make_move((0, 1)), Ok(()));
        assert_eq!(game.board[0][1], FieldState::Both);
    }

    // Test invalid move
    #[test]
    fn test_make_move_invalid_both() {
        let mut game = Game::new();

        // Test valid move
        assert_eq!(game.make_move((0, 1)), Ok(()));
        assert_eq!(
            game.board[0][1],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test valid move
        assert_eq!(game.make_move((1, 1)), Ok(()));
        assert_eq!(
            game.board[1][1],
            FieldState::OccupiedByOne(Player::Vertical)
        );

        // Test valid move
        assert_eq!(game.make_move((1, 2)), Ok(()));
        assert_eq!(
            game.board[1][2],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test valid move both
        assert_eq!(game.make_move((0, 1)), Ok(()));
        assert_eq!(game.board[0][1], FieldState::Both);

        // Test valid move
        assert_eq!(game.make_move((2, 1)), Ok(()));
        assert_eq!(
            game.board[2][1],
            FieldState::OccupiedByOne(Player::Horizontal)
        );

        // Test invalid move both
        assert_eq!(game.make_move((0, 1)), Err("Invalid move"));
    }
}

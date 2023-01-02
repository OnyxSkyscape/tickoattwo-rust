#[derive(Copy, Clone, PartialEq)]
enum Player {
    Player1,
    Player2,
}

#[derive(Copy, Clone, PartialEq)]
enum FieldState {
    None,
    OccupiedByOne(Player),
    Both,
}

type Board = [[FieldState; 3]; 3];

struct Game {
    board: Board,
    current_player: Player,
    previous_move: (usize, usize),
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[FieldState::None; 3]; 3],
            current_player: Player::Player1,
            previous_move: (3, 3),
        }
    }

    fn make_move(&mut self, current_player: Player, coords: (usize, usize)) -> Result<(), &str> {
        if coords.0 >= 3 || coords.0 < 0 || coords.1 >= 3 || coords.1 < 0 {
            return Err("Invalid coordinates");
        }
        let field = &mut self.board[coords.0][coords.1];
        *field = match field {
            FieldState::None => FieldState::OccupiedByOne(current_player),
            FieldState::OccupiedByOne(player) => {
                if *player == current_player {
                    return Err("Invalid move");
                }
                FieldState::Both
            }
            FieldState::Both => return Err("Invalid move"),
        };
        Ok(())
    }

    fn check_win(&self) -> bool {
        // Check rows
        for row in self.board.iter() {
            if row.iter().all(|&x| x == FieldState::Both) {
                return true;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] == FieldState::Both
                && self.board[1][col] == FieldState::Both
                && self.board[2][col] == FieldState::Both
            {
                return true;
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
            return true;
        }

        // No winning combination found
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_win() {
        let mut game = Game::new();

        // Test no winner
        assert_eq!(game.check_win(), false);

        // Test horizontal win
        game.board = [
            [FieldState::Both, FieldState::Both, FieldState::Both],
            [FieldState::None, FieldState::None, FieldState::None],
            [FieldState::None, FieldState::None, FieldState::None],
        ];
        assert_eq!(game.check_win(), true);

        // Test vertical win
        game.board = [
            [FieldState::None, FieldState::None, FieldState::None],
            [FieldState::Both, FieldState::Both, FieldState::Both],
            [FieldState::Both, FieldState::Both, FieldState::Both],
        ];
        assert_eq!(game.check_win(), true);

        // Test diagonal win
        game.board = [
            [FieldState::Both, FieldState::None, FieldState::None],
            [FieldState::None, FieldState::Both, FieldState::None],
            [FieldState::None, FieldState::None, FieldState::Both],
        ];
        assert_eq!(game.check_win(), true);
    }

    #[test]
    fn test_make_move() {
        let mut game = Game::new();

        // Test making a valid move
        assert!(game.make_move(Players::Player1, (0, 0)).is_ok());

        // Test making an invalid move because the coordinates are out of bounds
        assert!(game.make_move(Players::Player1, (3, 3)).is_err());
        assert!(game.make_move(Players::Player1, (0, 3)).is_err());
        assert!(game.make_move(Players::Player1, (3, 0)).is_err());
        assert!(game.make_move(Players::Player1, (4, 0)).is_err());

        // Test making a move on an occupied field
        assert!(game.make_move(Players::Player1, (0, 0)).is_err());
        assert!(game.make_move(Players::Player2, (0, 0)).is_err());
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    Horizontal,
    Vertical,
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
            current_player: Player::Horizontal,
            previous_move: (3, 3),
        }
    }

    fn make_move(&mut self, coords: (u8, u8)) -> Result<(), &str> {
        // Out of bound check
        if coords.0 >= 3 || coords.0 < 0 || coords.1 >= 3 || coords.1 < 0 {
            return Err("Invalid coordinates");
        }

        // Check if field was occupied the in last round
        if coords == self.previous_move {
            return Err("Invalid move: placed in last round");
        }

        let field = &mut self.board[coords.0][coords.1];

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

    fn check_win(&self) -> Option<Player> {
        // Check rows
        for row in self.board.iter() {
            if row.iter().all(|&x| x == FieldState::Both) {
                return self.current_player;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] == FieldState::Both
                && self.board[1][col] == FieldState::Both
                && self.board[2][col] == FieldState::Both
            {
                return self.current_player;
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
            return self.current_player;
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
        let player_horizontal = Player::Horizontal;
        let player_vertical = Player::Vertical;

        // Test invalid coordinates
        assert_eq!(
            game.make_move(player_horizontal, (3, 0)),
            Err("Invalid coordinates")
        );
        assert_eq!(
            game.make_move(player_horizontal, (0, 3)),
            Err("Invalid coordinates")
        );
        assert_eq!(
            game.make_move(player_horizontal, (0, 4)),
            Err("Invalid coordinates")
        );
        assert_eq!(
            game.make_move(player_horizontal, (4, 0)),
            Err("Invalid coordinates")
        );
        assert_eq!(
            game.make_move(player_horizontal, (3, 3)),
            Err("Invalid coordinates")
        );
        assert_eq!(
            game.make_move(player_horizontal, (4, 4)),
            Err("Invalid coordinates")
        );
        assert_eq!(
            game.make_move(player_horizontal, (5, 5)),
            Err("Invalid coordinates")
        );
        assert_eq!(
            game.make_move(player_horizontal, (100, 100)),
            Err("Invalid coordinates")
        );

        // Test invalid move: placed in last round
        game.previous_move = Some((0, 0));
        assert_eq!(
            game.make_move(player_horizontal, (0, 0)),
            Err("Invalid move: placed in last round")
        );

        // Test valid move
        assert_eq!(game.make_move(player_horizontal, (0, 1)), Ok(()));
        assert_eq!(
            game.board[0][1],
            FieldState::OccupiedByOne(player_horizontal)
        );

        // Test invalid move: already placed
        assert_eq!(
            game.make_move(player_horizontal, (0, 1)),
            Err("Invalid move: already placed")
        );
        assert_eq!(
            game.make_move(player_vertical, (0, 1)),
            Err("Invalid move: already placed")
        );
        assert_eq!(
            game.board[0][1],
            FieldState::OccupiedByOne(player_horizontal)
        );

        // Test valid move: occupied by other player
        assert_eq!(game.make_move(player_vertical, (1, 1)), Ok(()));
        assert_eq!(game.board[1][1], FieldState::OccupiedByOne(player_vertical));

        // Test invalid move: both players have placed
        assert_eq!(
            game.make_move(player_horizontal, (1, 1)),
            Err("Invalid move")
        );
        assert_eq!(game.board[1][1], FieldState::OccupiedByOne(player_vertical));
    }
}

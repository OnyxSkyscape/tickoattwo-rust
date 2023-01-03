use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::game::Game;
use crate::user::User;

pub struct Backend {
    waiting: Option<Arc<Mutex<User>>>,
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            waiting: None,
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn user_join(&mut self, user: Arc<Mutex<User>>) {
        if let Some(waiting_user) = &self.waiting {
            let waiting = Arc::clone(waiting_user);
            self.waiting = None;
            self.start_game(user, waiting);
        } else {
            self.waiting = Some(user);
        }
    }

    pub fn user_leave(&mut self, _user: User) {}

    fn start_game(&mut self, player1: Arc<Mutex<User>>, player2: Arc<Mutex<User>>) {
        let game = Game::new();
        let game_id = Uuid::new_v4();
        self.games.lock().unwrap().insert(game_id.clone(), game);
        player1.lock().unwrap().game = Some(game_id.clone());
        player2.lock().unwrap().game = Some(game_id.clone());
    }
}

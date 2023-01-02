use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::game::Game;
use crate::user::User;

pub struct Backend {
    waiting: Arc<Mutex<Option<User>>>,
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            waiting: Arc::new(Mutex::new(None)),
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn user_join(&mut self, user: User) {
        let arc = Arc::clone(&self.waiting);
        let mut waiting_user = arc.lock().unwrap();
        if waiting_user.is_some() {
            let new_player = waiting_user.as_ref().unwrap().clone();
            *waiting_user = None;
            self.start_game(user, new_player);
        } else {
            *waiting_user = Some(user);
        }
    }

    pub fn user_leave(&mut self, _user: User) {}

    fn start_game(&mut self, mut player1: User, mut player2: User) {
        let game = Game::new();
        let game_id = Uuid::new_v4();
        self.games.lock().unwrap().insert(game_id.clone(), game);
        player1.game = game_id.clone();
        player2.game = game_id.clone();
    }
}

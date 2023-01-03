use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::game::Game;
use crate::user::User;

pub struct Backend {
    queue: Option<SocketAddr>,
    users: Arc<Mutex<HashMap<SocketAddr, User>>>,
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            queue: None,
            users: Arc::new(Mutex::new(HashMap::new())),
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn user_join(&mut self, new_user_id: &SocketAddr) {
        let mut users = self.users.lock().unwrap();

        let new_user = User::new();
        users.insert(new_user_id.clone(), new_user);

        std::mem::drop(users);

        if let Some(queue_user_id) = self.queue {
            self.queue = None;
            self.start_game(&queue_user_id, new_user_id);
        } else {
            self.queue = Some(new_user_id.clone());
        }
    }

    pub fn user_leave(&mut self, _user_id: &SocketAddr) {}

    fn start_game(&mut self, user_id1: &SocketAddr, user_id2: &SocketAddr) {
        let game = Game::new();
        let game_id = Uuid::new_v4();
        self.games.lock().unwrap().insert(game_id.clone(), game);

        // Assign users to game
        let mut users = self.users.lock().unwrap();
        let user1 = users.get_mut(user_id1).unwrap();
        user1.game = Some(game_id.clone());
        let user2 = users.get_mut(user_id2).unwrap();
        user2.game = Some(game_id.clone());
    }
}

use log::{debug, info};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::game::Game;
use crate::user::User;
use tickoattwo::packet::{Event, Packet};

pub struct Backend {
    // Single slot waiting room
    queue: Option<SocketAddr>,

    // User store
    users: Arc<Mutex<HashMap<SocketAddr, User>>>,

    // Game state store
    games: Arc<Mutex<HashMap<Uuid, (Game, SocketAddr, SocketAddr)>>>,
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

    pub fn user_leave(&mut self, user_id: &SocketAddr) {
        let mut users = self.users.lock().unwrap();
        let mut games = self.games.lock().unwrap();

        if Some(user_id) == self.queue.as_ref() {
            self.queue = None;
            debug!("Removed user from queue: {}", user_id);
        }

        if let Some(user) = users.remove(user_id) {
            debug!("Removed user: {}", user_id);
            if let Some(game_id) = &user.game {
                if let Some(game) = games.remove(game_id) {
                    debug!("Removed game: {}", game_id);
                    let mut other_player: Option<SocketAddr> = None;

                    if &game.1 == user_id {
                        other_player = Some(game.2);
                    }

                    if &game.2 == user_id {
                        other_player = Some(game.1);
                    }

                    if let Some(other_player) = &other_player {
                        users.remove(other_player);
                        debug!("Removed other player: {}", other_player);
                    }
                }
            }
        }
    }

    fn start_game(&mut self, user_id1: &SocketAddr, user_id2: &SocketAddr) {
        let game = Game::new();
        let game_id = Uuid::new_v4();
        self.games
            .lock()
            .unwrap()
            .insert(game_id.clone(), (game, user_id1.clone(), user_id2.clone()));

        info!("Started new game: {} ({}, {})", game_id, user_id1, user_id2);

        // Assign users to game
        let mut users = self.users.lock().unwrap();
        let user1 = users.get_mut(user_id1).unwrap();
        user1.game = Some(game_id.clone());
        let user2 = users.get_mut(user_id2).unwrap();
        user2.game = Some(game_id.clone());
    }

    pub fn dispatch_event(&mut self, event: Event, user_id: &SocketAddr) -> Option<Packet> {
        debug!("Received event: {:?} ({})", event, user_id);

        match event {
            Event::Nickname(username) => {
                let mut users = self.users.lock().unwrap();
                let user = users.get_mut(user_id).unwrap();
                user.username = username;
            }
        }

        None
    }
}

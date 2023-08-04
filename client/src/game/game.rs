use std::{rc::Rc, cell::RefCell, f32::consts::PI, collections::HashMap};

use macroquad::prelude::*;
use matchbox_socket::{WebRtcSocket, PeerState, PeerId};
use async_executor::LocalExecutor;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::{Player, Weapon, Map, Collidable};
use crate::{GameObject, Resources};

const MATCHBOX_ADDR: &str = "ws://localhost:3536/";

#[derive(Serialize, Deserialize)]
struct PlayerState {
    remote_player_key: Uuid,
    x: f32,
    y: f32
}

type GameObjectRc = Rc<RefCell<dyn GameObject>>;
type CollidableRc = Rc<RefCell<dyn Collidable>>;

pub struct Game<'a> {
    local_player_key: Uuid,
    entities: HashMap<Uuid, GameObjectRc>,
    collidables: Vec<CollidableRc>,
    socket: WebRtcSocket,
    executor: LocalExecutor<'a>
}

impl<'a> Game<'a>  {
    pub fn new() -> Self {
        let local_player_key = Uuid::new_v4();
        let (socket, executor) = Self::connect();

        let mut game = Self {
            local_player_key,
            entities: HashMap::new(),
            collidables: Vec::new(),
            socket,
            executor
        };

        game.create_player(local_player_key, 160., 160.);

        game
    }

    fn connect() -> (WebRtcSocket, LocalExecutor<'a>) {
        info!("Constructing socket...");
        let (socket, message_loop) = WebRtcSocket::new_unreliable(MATCHBOX_ADDR);
        let executor = LocalExecutor::new();
        executor.spawn(message_loop).detach();
        (socket, executor)
    }

    fn broadcast_local_player_state(&mut self) {
        let game_obj = self.entities[&self.local_player_key].borrow();
        let Player { position, .. } = unsafe { game_obj.downcast::<Player>() };
        let [x, y] = position.borrow().to_array();
        let local_player_state = bincode::serialize(&PlayerState {
            remote_player_key: self.local_player_key,
            x,
            y,
        }).expect("Failed to serialize RemotePlayerState").into_boxed_slice();
        let peer_ids: Vec<PeerId> = self.socket.connected_peers().into_iter().collect();

        for peer_id in peer_ids {
            self.socket.send(local_player_state.clone(), peer_id);
        }
    }

    fn receive_remote_player_states(&mut self) {
        for (_, packet) in self.socket.receive() {
            let PlayerState { remote_player_key, x, y } = bincode::deserialize(&packet).unwrap();

            if let Some(entity) = self.entities.get(&remote_player_key) {
                let game_obj = entity.borrow_mut();
                let player = unsafe { game_obj.downcast::<Player>() };
                player.set_position(x, y);
            } else {
                self.create_player(remote_player_key, x, y);
            }
        }
    }

    fn create_player(&mut self, player_key: Uuid, x: f32, y: f32) {
        let pos = Rc::new(RefCell::new(Vec2::new(x, y)));
        // TODO: Load weapon attributes from a JSON file
        let weapon = Weapon::new(pos.clone(), Vec2::new(30., 10.), 2. * PI / 180.);
        let player = Rc::new(RefCell::new(Player::new(pos.clone(), weapon))); 
        self.entities.insert(player_key, player.clone());
        self.collidables.push(player.clone());
    }

    // TODO: optimize detect_collisions using spatial partitioning for better performance
    // (currently O(n^2))
    fn detect_collisions(&mut self) {
        for collidable in self.collidables.iter_mut() {
            collidable.borrow_mut().set_is_colliding(false);
        }

        for i in 0..self.collidables.len() {
            for j in i + 1..self.collidables.len() {
                let mut collidable_i = self.collidables[i].borrow_mut();
                let mut collidable_j = self.collidables[j].borrow_mut();

                if collidable_i.overlaps(collidable_j.hitbox()) {
                    collidable_i.set_is_colliding(true);
                    collidable_j.set_is_colliding(true);
                }
            }
        }
    }
}

impl<'a> GameObject for Game<'a> {
    fn update(&mut self, map: &Map) -> Option<Box<dyn GameObject>> {
        self.executor.try_tick();

        for (_, new_state) in self.socket.update_peers() {
            match new_state {
                PeerState::Connected => info!("peer connected"),
                PeerState::Disconnected => info!("peer disconnected"),
            }
        }

        self.detect_collisions();
        self.broadcast_local_player_state();
        self.receive_remote_player_states();

        self.entities.get_mut(&self.local_player_key).expect("Player not found").borrow_mut().update(map)
    }

    fn draw(&self, resources: &Resources, map: &Map) {
        map.draw_tiles("Tile Layer 1");
        self.entities.iter().for_each(|(_, entity)| entity.borrow().draw(resources, map));
        map.draw_tiles("Tile Layer 2");
    }
}

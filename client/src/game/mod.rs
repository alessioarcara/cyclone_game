pub mod map;
pub mod player;
pub mod weapon;
pub mod geometry;

use std::{collections::HashMap, str::FromStr};

use macroquad::prelude::info;
use matchbox_socket::{WebRtcSocket, PeerState, PeerId};
use async_executor::LocalExecutor;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use self::{player::Player, weapon::Weapon, geometry::Collidable};
use super::{GameObject, Resources, Map};

const MATCHBOX_ADDR: &str = "ws://localhost:3536/";

#[derive(Serialize, Deserialize)]
struct PlayerState {
    player_key: String,
    x: f32,
    y: f32
}

pub struct Game<'a> {
    player_key: Uuid,
    entities: HashMap<Uuid, Player>,
    socket: WebRtcSocket,
    executor: LocalExecutor<'a>
}

impl<'a> Game<'a>  {
    pub fn new() -> Self {
        let player_key = Uuid::new_v4();
        let entities: HashMap<Uuid, Player> = HashMap::new();
        let (socket, executor) = Self::connect();

        let mut game = Self {
            player_key,
            entities,
            socket,
            executor
        };

        game.spawn_player(160., 160., player_key);

        game
    }

    fn connect() -> (WebRtcSocket, LocalExecutor<'a>) {
        info!("Constructing socket...");
        let (socket, message_loop) = WebRtcSocket::new_unreliable(MATCHBOX_ADDR);
        let executor = LocalExecutor::new();
        executor.spawn(message_loop).detach();
        (socket, executor)
    }

    fn send_stuff(&mut self) {
        let Player { position, .. } = self.entities.get(&self.player_key).unwrap();
        let [x, y] = position.borrow().to_array();
        let player_state = bincode::serialize(&PlayerState {
            player_key: self.player_key.to_string(),
            x,
            y,
        }).unwrap().into_boxed_slice();
        let peer_ids: Vec<PeerId> = self.socket.connected_peers().into_iter().collect();

        for peer_id in peer_ids {
            self.socket.send(player_state.clone(), peer_id);
        }
    }

    fn receive_stuff(&mut self) {
        for (_, packet) in self.socket.receive() {
            let PlayerState { player_key, x, y }: PlayerState = bincode::deserialize(&packet).unwrap();
            let uuid = Uuid::from_str(&player_key).unwrap();
            let player = self.spawn_player(0., 0., uuid);
            let mut position = player.position.borrow_mut();
            position.x = x; 
            position.y = y;
        }
    }


    fn spawn_player(&mut self, x: f32, y: f32, uuid: Uuid) -> &Player {
        let player = self.entities.entry(uuid).or_insert(Player::new(x, y));
        player
    }

    fn detect_collisions(&mut self) {
        for entity in self.entities.values_mut() {
            entity.is_colliding = false;
        }

        let mut entities_vec: Vec<_> = self.entities.values_mut().collect();

        for i in 0..entities_vec.len() {
            for j in i + 1..entities_vec.len() {
                if entities_vec[i].overlaps(entities_vec[j]) {
                    entities_vec[i].is_colliding = true;
                    entities_vec[j].is_colliding = true;
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
        self.send_stuff();
        self.receive_stuff();

        self.entities.get_mut(&self.player_key).unwrap().update(map)
    }

    fn draw(&self, resources: &Resources, map: &Map) {
        map.draw_tiles("Tile Layer 1");
        self.entities.iter().for_each(|(_, entity)| entity.draw(resources, map));
        map.draw_tiles("Tile Layer 2");
    }
}

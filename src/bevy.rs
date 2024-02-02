use bevy_ecs::prelude::*;

#[derive(Debug, Component, Resource)]
pub struct ClientInfo {
    pub client_id: String,
    pub client_secret: String,
}

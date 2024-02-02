use bevy_ecs::prelude::*;

#[derive(Debug, Component, Resource)]
pub struct ClientInfo {
    pub id: String,
    pub secret: String,
}

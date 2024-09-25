use crate::ws_client::packet::dto::{move_direction::MoveDirection, rotation::Rotation};

pub enum AgentResponse {
    TankMovement {
        direction: MoveDirection,
    },
    TankRotation {
        tank_rotation: Option<Rotation>,
        turret_rotation: Option<Rotation>,
    },
    TankShoot,
}

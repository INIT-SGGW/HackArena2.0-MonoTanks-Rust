use super::rotation::Rotation;
use super::move_direction::MoveDirection;

/// Represents the various responses an agent can have in the system.
pub enum AgentResponse {
    /// Represents a movement action for a tank.
    ///
    /// ### Fields
    /// - `direction`: The direction in which the tank should move.
    TankMovement { direction: MoveDirection },

    /// Represents a rotation action for a tank.
    ///
    /// ### Fields
    /// - `tank_rotation`: The optional rotation of the tank's body.
    /// - `turret_rotation`: The optional rotation of the tank's turret.
    TankRotation {
        tank_rotation: Option<Rotation>,
        turret_rotation: Option<Rotation>,
    },

    /// Represents a shooting action for a tank.
    ///
    /// This variant indicates that the tank's turret should shoot a bullet.
    TankShoot,

    /// Represents a pass action, where the agent chooses to do nothing.
    /// It is useful when the agent want to wait on a site or when it is dead.
    ResponsePass,
}

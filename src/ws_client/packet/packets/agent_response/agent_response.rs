use super::ability_type::AbilityType;
use super::rotation::Rotation;
use super::move_direction::MoveDirection;

/// Represents the various responses an agent can have in the system.
pub enum AgentResponse {
    /// Represents a movement action for a tank.
    ///
    /// ### Fields
    /// - `direction`: The direction in which the tank should move.
    Movement { direction: MoveDirection },

    /// Represents a rotation action for a tank.
    ///
    /// ### Fields
    /// - `tank_rotation`: The optional rotation of the tank's body.
    /// - `turret_rotation`: The optional rotation of the tank's turret.
    Rotation {
        tank_rotation: Option<Rotation>,
        turret_rotation: Option<Rotation>,
    },

    /// Represents the use of an ability by the tank.
    ///
    /// ### Fields
    /// - `ability_type`: The type of ability to use.
    AbilityUse { ability_type: AbilityType },

    /// Represents a pass action, where the agent chooses to do nothing.
    /// It is useful when the agent want to wait on a site or when it is dead.
    Pass,
}

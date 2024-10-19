use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Warning {
    PlayerAlreadyMadeActionWarning,

    MissingGameStateIdWarning,

    SlowResponseWarning,

    ActionIgnoredDueToDeadWarning,

    CustomWarning { message: String },
}

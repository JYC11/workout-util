use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum LeverVariation {
    Tuck,
    AdvancedTuck,
    Straddle,
    OneLeg,
    HalfLay,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum Grip {
    Pronated,
    Supinated,
    Neutral,
    GymnasticsRing,
    Floor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum GripWidth {
    Wide,
    Shoulder,
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
pub enum Equipment {
    LowParallettes,
    HighParallettes,
    Bench,
    Dumbbells,
    Barbell,
    SmithMachine,
    GymnasticsRings,
    PullUpBar,
    DipBar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
pub enum Band {
    Yellow,
    Red,
    Black,
    Purple,
    Green,
}

// data models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum PushOrPull {
    Push,
    Pull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum DynamicOrStatic {
    Dynamic,
    Static,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum StraightOrBentArm {
    Straight,
    Bent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum SquatOrHinge {
    Squat,
    Hinge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum UpperOrLower {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum CompoundOrIsolation {
    Compound,
    Isolation,
}

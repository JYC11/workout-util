use std::fmt;
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

impl fmt::Display for LeverVariation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LeverVariation::Tuck => write!(f, "Tuck"),
            LeverVariation::AdvancedTuck => write!(f, "Advanced Tuck"),
            LeverVariation::Straddle => write!(f, "Straddle"),
            LeverVariation::OneLeg => write!(f, "One Leg"),  // Fixed spacing
            LeverVariation::HalfLay => write!(f, "Half Lay"),
            LeverVariation::Full => write!(f, "Full"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum Grip {
    Pronated,
    Supinated,
    Neutral,
    GymnasticsRing,
    Floor,
    Mixed,
}

impl fmt::Display for Grip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grip::Pronated => write!(f, "Pronated Grip"),
            Grip::Supinated => write!(f, "Supinated Grip"),
            Grip::Neutral => write!(f, "Neutral Grip"),
            Grip::GymnasticsRing => write!(f, "Gymnastics Ring Grip"),
            Grip::Floor => write!(f, "Floor Grip"),
            Grip::Mixed => write!(f, "Mixed Grip"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
pub enum GripWidth {
    Wide,
    Shoulder,
    Narrow,
}

impl fmt::Display for GripWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GripWidth::Wide => write!(f, "Wide"),
            GripWidth::Shoulder => write!(f, "Shoulder Width"),
            GripWidth::Narrow => write!(f, "Narrow"),
        }
    }
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

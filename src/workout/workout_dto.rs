use crate::workout::enums::{Band, Equipment};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutExerciseReq {
    pub workout_id: u32,
    pub exercise_id: u32,
    pub code: String,
    pub sets_target: u8,
    pub reps_or_seconds_target: u8,
    pub working_weight: u16,
    pub rest_period_seconds: u8,
    pub tempo: String,
    pub emom: bool,
    pub equipments: Vec<Equipment>,
    pub bands: Vec<Band>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutExerciseRes {
    pub id: u32,
    pub workout_id: u32,
    pub exercise_id: u32,
    pub code: String,
    pub sets_target: u8,
    pub reps_or_seconds_target: u8,
    pub working_weight: u16,
    pub rest_period_seconds: u8,
    pub tempo: String,
    pub equipments: Vec<Equipment>,
    pub bands: Vec<Band>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutReq {
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutRes {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

use crate::core::enums::{Band, Equipment};
use crate::core::workout::workout_entity::WorkoutExerciseEntity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutExerciseReq {
    pub workout_id: u32,
    pub name: String,
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
    pub name: String,
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

impl WorkoutExerciseRes {
    pub fn from_entity(entity: WorkoutExerciseEntity) -> WorkoutExerciseRes {
        Self {
            id: entity.id,
            workout_id: entity.workout_id,
            name: entity.name,
            code: entity.code,
            sets_target: entity.sets_target,
            reps_or_seconds_target: entity.reps_or_seconds_target,
            working_weight: entity.working_weight,
            rest_period_seconds: entity.rest_period_seconds,
            tempo: entity.tempo,
            equipments: entity.equipments.0,
            bands: entity.bands.0,
            description: entity.description,
        }
    }
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

#[derive(Clone)]
pub struct WorkoutsFilterReq {
    pub name: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
}

impl Default for WorkoutsFilterReq {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            active: None,
        }
    }
}

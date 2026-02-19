use crate::enums::{Band, Equipment};
use crate::workout::workout_entity::{WorkoutEntity, WorkoutExerciseEntity};

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
    pub emom: bool,
    pub equipments: Vec<Equipment>,
    pub bands: Vec<Band>,
    pub description: Option<String>,
}

pub trait RestMinuteAndSeconds {
    fn rest_minutes_and_seconds(&self) -> String;
}

impl RestMinuteAndSeconds for WorkoutExerciseRes {
    fn rest_minutes_and_seconds(&self) -> String {
        format!("{}m {}s rest", self.rest_period_seconds / 60, self.rest_period_seconds % 60)
    }
}

impl WorkoutExerciseRes {
    pub fn target(&self) -> String {
        format!(
            "{}x{} @ {}",
            self.sets_target, self.reps_or_seconds_target, self.working_weight
        )
    }
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
            emom: entity.emom,
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

impl WorkoutRes {
    pub fn from_entity(workout_entity: WorkoutEntity) -> WorkoutRes {
        Self {
            id: workout_entity.id,
            name: workout_entity.name,
            description: workout_entity.description,
            active: workout_entity.active,
        }
    }
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

pub fn default_workout_req() -> WorkoutReq {
    WorkoutReq {
        name: String::new(),
        description: None,
        active: true,
    }
}

pub fn default_exercise_req(workout_id: u32) -> WorkoutExerciseReq {
    WorkoutExerciseReq {
        workout_id,
        name: String::new(),
        code: String::new(),
        sets_target: 3,
        reps_or_seconds_target: 10,
        working_weight: 0,
        rest_period_seconds: 60,
        tempo: "0000".to_string(),
        emom: false,
        equipments: vec![],
        bands: vec![],
        description: None,
    }
}

pub fn workout_to_req(res: &WorkoutRes) -> WorkoutReq {
    WorkoutReq {
        name: res.name.clone(),
        description: res.description.clone(),
        active: res.active,
    }
}

pub fn exercise_res_to_req(res: &WorkoutExerciseRes) -> WorkoutExerciseReq {
    WorkoutExerciseReq {
        workout_id: res.workout_id,
        name: res.name.clone(),
        code: res.code.clone(),
        sets_target: res.sets_target,
        reps_or_seconds_target: res.reps_or_seconds_target,
        working_weight: res.working_weight,
        rest_period_seconds: res.rest_period_seconds,
        tempo: res.tempo.clone(),
        emom: false,
        equipments: res.equipments.clone(),
        bands: res.bands.clone(),
        description: res.description.clone(),
    }
}

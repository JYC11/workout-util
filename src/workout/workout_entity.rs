use crate::db::pagination_support::HasId;
use crate::enums::{Band, Equipment};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::types::Json;
use sqlx::FromRow;

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow, Deserialize)]
pub struct WorkoutExerciseEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub workout_id: u32,           // fk to Workout
    pub name: String,
    pub code: String, // A1, A2, B1, B2 ... input by user
    pub sets_target: u8,
    pub reps_or_seconds_target: u8,
    pub working_weight: u16,
    pub rest_period_seconds: u8,
    pub tempo: String,
    pub emom: bool,
    pub equipments: Json<Vec<Equipment>>,
    pub bands: Json<Vec<Band>>,
    pub description: Option<String>,
}
// WorkoutEntity -> WorkoutExerciseEntity, 1:many
// ExerciseLibraryEntry -> WorkoutExerciseEntity, 1:many

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow, Deserialize)]
pub struct WorkoutEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

impl HasId for WorkoutEntity {
    fn id(&self) -> u32 {
        self.id
    }
}

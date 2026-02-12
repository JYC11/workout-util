use crate::core::enums::{Band, Equipment};
use chrono::{DateTime, Utc};
use sqlx::types::Json;
use sqlx::{FromRow, Sqlite, Transaction};

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct WorkoutExerciseEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub workout_id: u32,           // fk to Workout
    pub exercise_id: u32,          // fk to ExerciseLibraryEntry
    pub code: String,              // A1, A2, B1, B2 ...
    pub sets_target: u8,
    pub reps_or_seconds_target: u8,
    pub working_weight: u8,
    pub rest_period_seconds: u8,
    pub tempo: String,
    pub equipments: Json<Vec<Equipment>>,
    pub bands: Json<Vec<Band>>,
    pub description: Option<String>,
}
// WorkoutEntity -> WorkoutExerciseEntity, 1:many
// ExerciseLibraryEntry -> WorkoutExerciseEntity, 1:many

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutExerciseReq {
    pub workout_id: u32,
    pub exercise_id: u32,
    pub code: String,
    pub sets_target: u8,
    pub reps_or_seconds_target: u8,
    pub working_weight: u8,
    pub rest_period_seconds: u8,
    pub tempo: String,
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
    pub working_weight: u8,
    pub rest_period_seconds: u8,
    pub tempo: String,
    pub equipments: Vec<Equipment>,
    pub bands: Vec<Band>,
    pub description: Option<String>,
}

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct WorkoutEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub workout_plan_id: u32,      // fk to WorkoutPlanEntity
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutReq {
    pub workout_plan_id: u32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutRes {
    pub id: u32,
    pub workout_plan_id: u32,
    pub name: String,
    pub description: Option<String>,
}

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct WorkoutPlanEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub name: String,
    pub description: Option<String>,
    pub currently_using: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutPlanReq {
    pub name: String,
    pub description: Option<String>,
    pub currently_using: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutPlanRes {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub currently_using: bool,
}

// WorkoutPlanEntity -> WorkoutEntity, 1:many

pub fn create_workout_plan(
    tx: &mut Transaction<'_, Sqlite>,
    req: WorkoutPlanReq,
) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn create_workout(tx: &mut Transaction<'_, Sqlite>, req: WorkoutReq) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn create_workout_exercise(
    tx: &mut Transaction<'_, Sqlite>,
    req: WorkoutExerciseReq,
) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn update_workout_plan(
    tx: &mut Transaction<'_, Sqlite>,
    id: u32,
    req: WorkoutPlanReq,
) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn update_workout(
    tx: &mut Transaction<'_, Sqlite>,
    id: u32,
    req: WorkoutReq,
) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn update_workout_exercise(
    tx: &mut Transaction<'_, Sqlite>,
    id: u32,
    req: WorkoutExerciseReq,
) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn delete_workout_plan(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn delete_workout(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    // prevent deleting workouts that are in use by workout logs
    // prevent deleting workout that are used by workout plans
    Ok(())
}

pub fn delete_workout_exercise(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    // prevent deleting exercises that are in use by workouts and that have logs
    Ok(())
}

pub fn get_one_workout_plan(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn get_one_workout(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn get_one_workout_exercise(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn paginate_workout_plan(tx: &Transaction<Sqlite>) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn paginate_one_workout(tx: &Transaction<Sqlite>) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn paginate_one_workout_exercise(tx: &Transaction<Sqlite>) -> Result<(), String> {
    // TODO
    Ok(())
}

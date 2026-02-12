use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, Sqlite, Transaction};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct WorkoutLogEntity {
    pub id: u32,
    pub workout_id: u32,           // fk to WorkoutEntity
    pub workout_exercise_id: u32,  // fk to WorkoutExerciseEntity
    pub workout_log_group_id: u32, // fk to WorkoutLogGroupEntity
    pub set_number: u8,
    pub rep_number_or_seconds: u8,
    pub weight: u8,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutLogReq {
    pub workout_id: u32,
    pub workout_exercise_id: u32,
    pub workout_log_group_id: u32,
    pub set_number: u8,
    pub rep_number_or_seconds: u8,
    pub weight: u8,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkoutLogRes {
    pub id: u32,
    pub workout_id: u32,
    pub workout_exercise_id: u32,
    pub workout_log_group_id: u32,
    pub set_number: u8,
    pub rep_number_or_seconds: u8,
    pub weight: u8,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct WorkoutLogGroupEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub date: NaiveDate,           // should be some kinda Date
    pub notes: Option<String>,
}
// WorkoutLogGroupEntity -> WorkoutLogEntity 1:many

pub fn create_log_group(tx: &Transaction<Sqlite>) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn create_log(tx: &mut Transaction<'_, Sqlite>, req: WorkoutLogReq) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn delete_log_group(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn delete_log(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn paginate_logs(tx: &Transaction<Sqlite>) -> Result<(), String> {
    // TODO
    Ok(())
}

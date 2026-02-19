use crate::db::pagination_support::HasId;
use chrono::NaiveDate;
use sqlx::FromRow;

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

pub struct WorkoutLogFilterReq {
    pub workout_date_gte: Option<NaiveDate>,
    pub workout_date_lte: Option<NaiveDate>,
    pub workout_name: Option<String>,
    pub workout_exercise_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct WorkoutLogDetailRes {
    pub id: u32,
    pub workout_log_group_id: u32,
    pub workout_date: NaiveDate,
    pub workout_id: u32,
    pub workout_name: String,
    pub workout_exercise_id: u32,
    pub workout_exercise_name: String,
    pub set_number: u8,
    pub rep_number_or_seconds: u8,
    pub weight: u8,
    pub description: Option<String>,
}

impl HasId for WorkoutLogDetailRes {
    fn id(&self) -> u32 {
        self.id
    }
}

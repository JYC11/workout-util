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

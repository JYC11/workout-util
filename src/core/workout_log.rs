use crate::context::AppContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub struct WorkoutLogGroupEntity {
    pub id: u32,
    pub created_at: String, // should be some kinda DateTime
    pub date: String,       // should be some kinda Date
}
// WorkoutLogGroupEntity -> WorkoutLogEntity 1:many

pub fn create_log_group(app_context: &AppContext) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn create_log(app_context: &AppContext, req: WorkoutLogReq) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn delete_log_group(app_context: &AppContext, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn delete_log(app_context: &AppContext, id: u32) -> Result<(), String> {
    // TODO
    Ok(())
}

pub fn paginate_logs(app_context: &AppContext) -> Result<(), String> {
    // TODO
    Ok(())
}

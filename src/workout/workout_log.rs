use crate::workout::workout_log_dto::{WorkoutLogReq, WorkoutLogRes};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{Executor, FromRow, Sqlite, Transaction};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct WorkoutLogGroupEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub date: NaiveDate,           // should be some kinda Date
    pub notes: Option<String>,
}
// WorkoutLogGroupEntity -> WorkoutLogEntity 1:many

// --- WORKOUT LOG GROUP ---

pub async fn create_log_group(
    tx: &mut Transaction<'_, Sqlite>,
    date: NaiveDate,
    notes: Option<String>,
) -> Result<u32, String> {
    let created_at = Utc::now();

    let result =
        sqlx::query(r#"INSERT INTO workout_log_groups (created_at, date, notes) VALUES (?, ?, ?)"#)
            .bind(created_at)
            .bind(date)
            .bind(notes)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Failed to create log group: {}", e))?;

    Ok(result.last_insert_rowid() as u32)
}

pub async fn delete_log_group(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    // Will fail if any workout_log references it (ON DELETE RESTRICT)
    let result = sqlx::query("DELETE FROM workout_log_groups WHERE id = ?")
        .bind(id)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Failed to delete log group (may be in use): {}", e))?;

    if result.rows_affected() == 0 {
        return Err("Log group not found".to_string());
    }

    Ok(())
}

// TODO need to join logs
pub async fn get_one_log_group<'e, E: Executor<'e, Database = Sqlite>>(
    executor: E,
    id: u32,
) -> Result<WorkoutLogGroupEntity, String> {
    sqlx::query_as::<_, WorkoutLogGroupEntity>("SELECT * FROM workout_log_groups WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Log group not found".to_string())
}

// --- WORKOUT LOG ---

pub async fn create_log(
    tx: &mut Transaction<'_, Sqlite>,
    req: WorkoutLogReq,
) -> Result<u32, String> {
    let result = sqlx::query(
        r#"INSERT INTO workout_logs (
            workout_id, workout_exercise_id, workout_log_group_id,
            set_number, rep_number_or_seconds, weight, description
        ) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(req.workout_id)
    .bind(req.workout_exercise_id)
    .bind(req.workout_log_group_id)
    .bind(req.set_number)
    .bind(req.rep_number_or_seconds)
    .bind(req.weight)
    .bind(req.description)
    .execute(&mut **tx)
    .await
    .map_err(|e| format!("Failed to create workout log: {}", e))?;

    Ok(result.last_insert_rowid() as u32)
}

pub async fn delete_log(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    let result = sqlx::query("DELETE FROM workout_logs WHERE id = ?")
        .bind(id)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Failed to delete workout log: {}", e))?;

    if result.rows_affected() == 0 {
        return Err("Workout log not found".to_string());
    }

    Ok(())
}

// TODO need to join exercises and workout
pub async fn get_one_log<'e, E: Executor<'e, Database = Sqlite>>(
    executor: E,
    id: u32,
) -> Result<WorkoutLogRes, String> {
    let entity: WorkoutLogEntity = sqlx::query_as("SELECT * FROM workout_logs WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Workout log not found".to_string())?;

    Ok(WorkoutLogRes {
        id: entity.id,
        workout_id: entity.workout_id,
        workout_exercise_id: entity.workout_exercise_id,
        workout_log_group_id: entity.workout_log_group_id,
        set_number: entity.set_number,
        rep_number_or_seconds: entity.rep_number_or_seconds,
        weight: entity.weight,
        description: entity.description,
    })
}

pub async fn paginate_logs(tx: &mut Transaction<'_, Sqlite>) -> Result<(), String> {
    // TODO
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{IN_MEMORY_DB_URL, init_db};
    use crate::workout::enums::{
        Band, CompoundOrIsolation, DynamicOrStatic, Equipment, UpperOrLower,
    };
    use sqlx::SqlitePool;

    async fn setup_db() -> SqlitePool {
        init_db(IN_MEMORY_DB_URL).await
    }

    // Helper: Create minimal exercise for FK
    async fn create_dummy_exercise(tx: &mut Transaction<'_, Sqlite>) -> u32 {
        sqlx::query(
            r#"INSERT INTO exercise_library (
                name, dynamic_or_static, upper_or_lower, compound_or_isolation
            ) VALUES (?, ?, ?, ?)"#,
        )
        .bind("Dummy Ex")
        .bind(DynamicOrStatic::Dynamic)
        .bind(UpperOrLower::Upper)
        .bind(CompoundOrIsolation::Compound)
        .execute(&mut **tx)
        .await
        .unwrap();
        1 // first auto-increment ID
    }

    // Helper: Create full workout plan → workout → workout_exercise chain
    async fn create_workout_exercise(tx: &mut Transaction<'_, Sqlite>) -> (u32, u32, u32) {
        // (plan_id, workout_id, exercise_id)
        let exercise_id = create_dummy_exercise(tx).await;

        let plan_id = sqlx::query(
            "INSERT INTO workout_plans (created_at, name, description, currently_using) VALUES (?, ?, ?, ?)"
        )
            .bind(chrono::Utc::now())
            .bind("Test Plan")
            .bind(Option::<String>::None)
            .bind(false)
            .execute(&mut **tx)
            .await
            .unwrap()
            .last_insert_rowid() as u32;

        let workout_id = sqlx::query(
            "INSERT INTO workouts (created_at, workout_plan_id, name, description) VALUES (?, ?, ?, ?)"
        )
            .bind(chrono::Utc::now())
            .bind(plan_id)
            .bind("Test Workout")
            .bind(Option::<String>::None)
            .execute(&mut **tx)
            .await
            .unwrap()
            .last_insert_rowid() as u32;

        sqlx::query(
            r#"INSERT INTO workout_exercises (
                created_at, workout_id, exercise_id, code,
                sets_target, reps_or_seconds_target, working_weight,
                rest_period_seconds, tempo, emom, equipments, bands, description
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(chrono::Utc::now())
        .bind(workout_id)
        .bind(exercise_id)
        .bind("A1")
        .bind(3u8)
        .bind(10u8)
        .bind(50u8)
        .bind(60u8)
        .bind("2010")
        .bind(false)
        .bind(sqlx::types::Json(vec![Equipment::Barbell]))
        .bind(sqlx::types::Json(vec![Band::Yellow]))
        .bind(Option::<String>::None)
        .execute(&mut **tx)
        .await
        .unwrap();

        let workout_exercise_id = 1u32; // first one

        (plan_id, workout_id, workout_exercise_id)
    }

    #[tokio::test]
    async fn test_log_group_crud() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        let today = Utc::now().naive_utc().date();

        // Create
        let group_id = create_log_group(&mut tx, today, Some("Morning session".to_string()))
            .await
            .expect("Failed to create log group");

        // Get
        let group = get_one_log_group(&mut *tx, group_id)
            .await
            .expect("Failed to get log group");
        assert_eq!(group.date, today);
        assert_eq!(group.notes, Some("Morning session".to_string()));

        // Delete
        delete_log_group(&mut tx, group_id)
            .await
            .expect("Failed to delete log group");

        assert!(get_one_log_group(&mut *tx, group_id).await.is_err());

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_workout_log_crud() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        let (_plan_id, workout_id, workout_exercise_id) = create_workout_exercise(&mut tx).await;

        let today = Utc::now().naive_utc().date();
        let group_id = create_log_group(&mut tx, today, None).await.unwrap();

        // Create log
        let log_req = WorkoutLogReq {
            workout_id,
            workout_exercise_id,
            workout_log_group_id: group_id,
            set_number: 2,
            rep_number_or_seconds: 8,
            weight: 95,
            description: Some("Felt strong".to_string()),
        };

        let log_id = create_log(&mut tx, log_req)
            .await
            .expect("Failed to create workout log");

        // Get
        let log = get_one_log(&mut *tx, log_id)
            .await
            .expect("Failed to get workout log");
        assert_eq!(log.workout_id, workout_id);
        assert_eq!(log.workout_exercise_id, workout_exercise_id);
        assert_eq!(log.workout_log_group_id, group_id);
        assert_eq!(log.set_number, 2);
        assert_eq!(log.weight, 95);
        assert_eq!(log.description, Some("Felt strong".to_string()));

        // Delete
        delete_log(&mut tx, log_id)
            .await
            .expect("Failed to delete workout log");

        assert!(get_one_log(&mut *tx, log_id).await.is_err());

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_cannot_delete_log_group_with_logs() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        let (_plan_id, workout_id, workout_exercise_id) = create_workout_exercise(&mut tx).await;
        let today = Utc::now().naive_utc().date();
        let group_id = create_log_group(&mut tx, today, None).await.unwrap();

        // Create a log referencing the group
        let log_req = WorkoutLogReq {
            workout_id,
            workout_exercise_id,
            workout_log_group_id: group_id,
            set_number: 1,
            rep_number_or_seconds: 10,
            weight: 80,
            description: None,
        };
        create_log(&mut tx, log_req).await.unwrap();

        // Attempt to delete group → should fail
        let result = delete_log_group(&mut tx, group_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("may be in use"));

        tx.commit().await.unwrap();
    }
}

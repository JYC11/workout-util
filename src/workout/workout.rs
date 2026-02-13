use crate::db::pagination_support::PaginationParams;
use crate::workout::enums::{Band, Equipment};
use crate::workout::workout_dto::{WorkoutExerciseReq, WorkoutExerciseRes, WorkoutReq, WorkoutRes};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::types::Json;
use sqlx::{Executor, FromRow, Sqlite, Transaction};

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow, Deserialize)]
pub struct WorkoutExerciseEntity {
    pub id: u32,
    pub created_at: DateTime<Utc>, // should be some kinda DateTime
    pub workout_id: u32,           // fk to Workout
    pub exercise_id: u32,          // fk to ExerciseLibraryEntry
    pub code: String,              // A1, A2, B1, B2 ...
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

// --- WORKOUT ---

pub async fn create_workout(
    tx: &mut Transaction<'_, Sqlite>,
    req: WorkoutReq,
) -> Result<u32, String> {
    let created_at = Utc::now();

    let result = sqlx::query(
        r#"
        INSERT INTO workouts (created_at, name, description, active)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(created_at)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.active)
    .execute(&mut **tx)
    .await
    .map_err(|e| format!("Failed to create workout: {}", e))?;

    let id = result.last_insert_rowid() as u32;
    Ok(id)
}

pub async fn update_workout(
    tx: &mut Transaction<'_, Sqlite>,
    id: u32,
    req: WorkoutReq,
) -> Result<(), String> {
    let result = sqlx::query(
        r#"
        UPDATE workouts
        SET name = ?, description = ?, active = ?
        WHERE id = ?
        "#,
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.active)
    .bind(id)
    .execute(&mut **tx)
    .await
    .map_err(|e| format!("Failed to update workout: {}", e))?;

    if result.rows_affected() == 0 {
        return Err("Workout not found".to_string());
    }

    Ok(())
}

pub async fn delete_workout(tx: &mut Transaction<'_, Sqlite>, id: u32) -> Result<(), String> {
    let result = sqlx::query("DELETE FROM workouts WHERE id = ?")
        .bind(id)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Failed to delete workout (may be in use): {}", e))?;

    if result.rows_affected() == 0 {
        return Err("Workout not found".to_string());
    }

    Ok(())
}

// TODO needs to join workout exercises and exercise library
pub async fn get_one_workout<'e, E: Executor<'e, Database = Sqlite>>(
    executor: E,
    id: u32,
) -> Result<WorkoutRes, String> {
    let row: WorkoutEntity = sqlx::query_as("SELECT * FROM workouts WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Workout not found".to_string())?;

    Ok(WorkoutRes {
        id: row.id,
        name: row.name,
        description: row.description,
        active: row.active,
    })
}

pub fn paginate_workouts<'e, E: Executor<'e, Database = Sqlite>>(
    executor: E,
    pagination_params: PaginationParams,
) -> Result<(), String> {
    // TODO
    Ok(())
}

// --- WORKOUT EXERCISE ---

pub async fn create_workout_exercise(
    tx: &mut Transaction<'_, Sqlite>,
    req: WorkoutExerciseReq,
) -> Result<u32, String> {
    let created_at = Utc::now();

    let result = sqlx::query(
        r#"
        INSERT INTO workout_exercises (
            created_at, workout_id, exercise_id, code,
            sets_target, reps_or_seconds_target, working_weight,
            rest_period_seconds, tempo, emom, equipments, bands, description
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(created_at)
    .bind(req.workout_id)
    .bind(req.exercise_id)
    .bind(&req.code)
    .bind(req.sets_target)
    .bind(req.reps_or_seconds_target)
    .bind(req.working_weight)
    .bind(req.rest_period_seconds)
    .bind(&req.tempo)
    .bind(&req.emom)
    .bind(Json(req.equipments))
    .bind(Json(req.bands))
    .bind(&req.description)
    .execute(&mut **tx)
    .await
    .map_err(|e| format!("Failed to create workout exercise: {}", e))?;

    let id = result.last_insert_rowid() as u32;
    Ok(id)
}

pub async fn update_workout_exercise(
    tx: &mut Transaction<'_, Sqlite>,
    id: u32,
    req: WorkoutExerciseReq,
) -> Result<(), String> {
    let result = sqlx::query(
        r#"
        UPDATE workout_exercises
        SET workout_id = ?, exercise_id = ?, code = ?,
            sets_target = ?, reps_or_seconds_target = ?, working_weight = ?,
            rest_period_seconds = ?, tempo = ?, emom = ?, equipments = ?,
            bands = ?, description = ?
        WHERE id = ?
        "#,
    )
    .bind(req.workout_id)
    .bind(req.exercise_id)
    .bind(&req.code)
    .bind(req.sets_target)
    .bind(req.reps_or_seconds_target)
    .bind(req.working_weight)
    .bind(req.rest_period_seconds)
    .bind(&req.tempo)
    .bind(req.emom)
    .bind(Json(req.equipments))
    .bind(Json(req.bands))
    .bind(&req.description)
    .bind(id)
    .execute(&mut **tx)
    .await
    .map_err(|e| format!("Failed to update workout exercise: {}", e))?;

    if result.rows_affected() == 0 {
        return Err("Workout exercise not found".to_string());
    }

    Ok(())
}

pub async fn delete_workout_exercise(
    tx: &mut Transaction<'_, Sqlite>,
    id: u32,
) -> Result<(), String> {
    let result = sqlx::query("DELETE FROM workout_exercises WHERE id = ?")
        .bind(id)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Failed to delete workout exercise (may be in use): {}", e))?;

    if result.rows_affected() == 0 {
        return Err("Workout exercise not found".to_string());
    }

    Ok(())
}

// TODO join to exercise library to get exercise name
pub async fn get_one_workout_exercise<'e, E: Executor<'e, Database = Sqlite>>(
    executor: E,
    id: u32,
) -> Result<WorkoutExerciseRes, String> {
    let row: WorkoutExerciseEntity = sqlx::query_as("SELECT * FROM workout_exercises WHERE id = ?")
        .bind(id)
        .fetch_optional(executor)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Workout exercise not found".to_string())?;

    Ok(WorkoutExerciseRes {
        id: row.id,
        workout_id: row.workout_id,
        exercise_id: row.exercise_id,
        code: row.code,
        sets_target: row.sets_target,
        reps_or_seconds_target: row.reps_or_seconds_target,
        working_weight: row.working_weight,
        rest_period_seconds: row.rest_period_seconds,
        tempo: row.tempo,
        equipments: row.equipments.0,
        bands: row.bands.0,
        description: row.description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{IN_MEMORY_DB_URL, init_db};
    use crate::workout::enums::{Band, Equipment};
    use sqlx::SqlitePool;

    async fn setup_db() -> SqlitePool {
        init_db(IN_MEMORY_DB_URL).await
    }

    fn mock_workout_req(name: &str) -> WorkoutReq {
        WorkoutReq {
            name: name.to_string(),
            description: Some(format!("{} notes", name)),
            active: true,
        }
    }

    fn mock_workout_exercise_req(
        workout_id: u32,
        exercise_id: u32,
        code: &str,
    ) -> WorkoutExerciseReq {
        WorkoutExerciseReq {
            workout_id,
            exercise_id,
            code: code.to_string(),
            sets_target: 4,
            reps_or_seconds_target: 8,
            working_weight: 100,
            rest_period_seconds: 90,
            tempo: "2010".to_string(),
            emom: false,
            equipments: vec![Equipment::Barbell],
            bands: vec![Band::Black],
            description: Some("Standard set".to_string()),
        }
    }

    // --- WORKOUT TESTS ---

    #[tokio::test]
    async fn test_workout_crud() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        // Create workout
        let workout_id = create_workout(&mut tx, mock_workout_req("Upper Body A"))
            .await
            .expect("Failed to create workout");

        // Get
        let workout = get_one_workout(&mut *tx, workout_id)
            .await
            .expect("Failed to get workout");
        assert_eq!(workout.name, "Upper Body A");

        // Update
        let mut updated_req = mock_workout_req("Lower Body A");
        updated_req.description = Some("Leg day!".to_string());
        update_workout(&mut tx, workout_id, updated_req)
            .await
            .expect("Failed to update workout");

        let updated_workout = get_one_workout(&mut *tx, workout_id)
            .await
            .expect("Failed to get updated workout");
        assert_eq!(updated_workout.name, "Lower Body A");
        assert_eq!(updated_workout.description, Some("Leg day!".to_string()));

        // Delete
        delete_workout(&mut tx, workout_id)
            .await
            .expect("Failed to delete workout");

        assert!(get_one_workout(&mut *tx, workout_id).await.is_err());

        tx.commit().await.unwrap();
    }

    // --- WORKOUT EXERCISE TESTS ---

    #[tokio::test]
    async fn test_workout_exercise_crud() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        // Setup dependencies
        let workout_id = create_workout(&mut tx, mock_workout_req("Test Workout"))
            .await
            .unwrap();

        // For exercise_id, we'll assume exercise ID 1 exists (or create one)
        // Since this test focuses on workout_exercise, we can insert a minimal exercise
        // directly into exercise_library to avoid dependency on other modules
        sqlx::query(
            r#"INSERT INTO exercise_library (
                name, dynamic_or_static, upper_or_lower, compound_or_isolation
            ) VALUES (?, ?, ?, ?)"#,
        )
        .bind("Dummy Pushup")
        .bind(crate::workout::enums::DynamicOrStatic::Dynamic)
        .bind(crate::workout::enums::UpperOrLower::Upper)
        .bind(crate::workout::enums::CompoundOrIsolation::Compound)
        .execute(&mut *tx)
        .await
        .unwrap();
        let exercise_id = 1; // first autoincrement ID

        // Create
        let ex_id = create_workout_exercise(
            &mut tx,
            mock_workout_exercise_req(workout_id, exercise_id, "A1"),
        )
        .await
        .expect("Failed to create workout exercise");

        // Get
        let ex = get_one_workout_exercise(&mut *tx, ex_id)
            .await
            .expect("Failed to get workout exercise");
        assert_eq!(ex.code, "A1");
        assert_eq!(ex.workout_id, workout_id);
        assert_eq!(ex.exercise_id, exercise_id);
        assert_eq!(ex.equipments, vec![Equipment::Barbell]);
        assert_eq!(ex.bands, vec![Band::Black]);
        assert_eq!(ex.tempo, "2010");

        // Update
        let mut updated_req = mock_workout_exercise_req(workout_id, exercise_id, "A2");
        updated_req.sets_target = 5;
        updated_req.working_weight = 120;
        updated_req.equipments = vec![Equipment::Dumbbells];
        updated_req.bands = vec![];
        updated_req.description = None;

        update_workout_exercise(&mut tx, ex_id, updated_req)
            .await
            .expect("Failed to update workout exercise");

        let updated_ex = get_one_workout_exercise(&mut *tx, ex_id)
            .await
            .expect("Failed to get updated exercise");
        assert_eq!(updated_ex.code, "A2");
        assert_eq!(updated_ex.sets_target, 5);
        assert_eq!(updated_ex.working_weight, 120);
        assert_eq!(updated_ex.equipments, vec![Equipment::Dumbbells]);
        assert!(updated_ex.bands.is_empty());
        assert_eq!(updated_ex.description, None);

        // Delete
        delete_workout_exercise(&mut tx, ex_id)
            .await
            .expect("Failed to delete workout exercise");

        assert!(get_one_workout_exercise(&mut *tx, ex_id).await.is_err());

        tx.commit().await.unwrap();
    }

    // --- INTEGRITY TESTS ---

    #[tokio::test]
    async fn test_cannot_delete_workout_with_exercises() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        let workout_id = create_workout(&mut tx, mock_workout_req("Protected Workout"))
            .await
            .unwrap();

        // Insert dummy exercise
        sqlx::query(
            r#"INSERT INTO exercise_library (name, dynamic_or_static, upper_or_lower, compound_or_isolation)
               VALUES ('Test Ex', 'Dynamic', 'Upper', 'Compound')"#,
        )
            .execute(&mut *tx)
            .await
            .unwrap();

        // Create a workout exercise → now workout is referenced
        create_workout_exercise(&mut tx, mock_workout_exercise_req(workout_id, 1, "A1"))
            .await
            .unwrap();

        // Attempt to delete workout → should fail due to RESTRICT
        let result = delete_workout(&mut tx, workout_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("may be in use"));

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_cannot_delete_workout_exercise_with_logs() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        let workout_id = create_workout(&mut tx, mock_workout_req("Logged Workout"))
            .await
            .unwrap();

        sqlx::query(
            r#"INSERT INTO exercise_library (name, dynamic_or_static, upper_or_lower, compound_or_isolation)
               VALUES ('Log Ex', 'Dynamic', 'Upper', 'Compound')"#,
        )
            .execute(&mut *tx)
            .await
            .unwrap();

        let ex_id =
            create_workout_exercise(&mut tx, mock_workout_exercise_req(workout_id, 1, "A1"))
                .await
                .unwrap();

        // Create a log group
        let now = Utc::now().format("%Y-%m-%d").to_string();
        let log_group_id: i64 = sqlx::query(
            "INSERT INTO workout_log_groups (created_at, date, notes) VALUES (?, ?, ?)",
        )
        .bind(Utc::now())
        .bind(&now)
        .bind("Test session")
        .execute(&mut *tx)
        .await
        .unwrap()
        .last_insert_rowid();

        // Create a log entry referencing the workout exercise
        sqlx::query(
            r#"INSERT INTO workout_logs (
                workout_id, workout_exercise_id, workout_log_group_id,
                set_number, rep_number_or_seconds, weight, description
            ) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(workout_id)
        .bind(ex_id)
        .bind(log_group_id as u32)
        .bind(1u8)
        .bind(8u8)
        .bind(100u32)
        .bind("Completed")
        .execute(&mut *tx)
        .await
        .unwrap();

        // Now try to delete the workout exercise → should fail
        let result = delete_workout_exercise(&mut tx, ex_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("may be in use"));

        tx.commit().await.unwrap();
    }
}

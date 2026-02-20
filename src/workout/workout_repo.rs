use crate::db::pagination_support::{
    PaginationParams, PaginationRes, get_cursors, keyset_paginate,
};
use crate::workout::workout_dto::{
    WorkoutExerciseReq, WorkoutExerciseRes, WorkoutReq, WorkoutRes, WorkoutsFilterReq,
};
use crate::workout::workout_entity::{WorkoutEntity, WorkoutExerciseEntity};
use chrono::Utc;
use sqlx::types::Json;
use sqlx::{Executor, QueryBuilder, Sqlite, Transaction};

#[derive(Copy, Clone)]
pub struct WorkoutRepo {}

impl WorkoutRepo {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_workout(
        &self,
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
        .map_err(|e| format!("Failed to create core: {}", e))?;

        let id = result.last_insert_rowid() as u32;
        Ok(id)
    }

    pub async fn update_workout(
        &self,
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
        .map_err(|e| format!("Failed to update core: {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Workout not found".to_string());
        }

        Ok(())
    }

    pub async fn delete_workout(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        id: u32,
    ) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM workouts WHERE id = ?")
            .bind(id)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Failed to delete core (may be in use): {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Workout not found".to_string());
        }

        Ok(())
    }

    pub async fn get_one_workout<'e, E: Executor<'e, Database = Sqlite>>(
        &self,
        executor: E,
        id: u32,
    ) -> Result<WorkoutRes, String> {
        let row: WorkoutEntity = sqlx::query_as("SELECT * FROM workouts WHERE id = ?")
            .bind(id)
            .fetch_optional(executor)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Workout not found".to_string())?;

        Ok(WorkoutRes::from_entity(row))
    }

    pub async fn paginate_workouts<'e, E: Executor<'e, Database = Sqlite>>(
        &self,
        executor: E,
        pagination_filters: Option<WorkoutsFilterReq>,
        pagination_params: PaginationParams,
    ) -> Result<PaginationRes<WorkoutRes>, String> {
        let mut qb = QueryBuilder::new("SELECT * FROM workouts WHERE 1=1");
        self.pagination_filters(pagination_filters, &mut qb);
        keyset_paginate(&pagination_params, None, &mut qb);

        let mut rows: Vec<WorkoutEntity> = qb
            .build_query_as()
            .fetch_all(executor)
            .await
            .map_err(|e| format!("Failed to paginate workouts: {}", e))?;

        let cursors = get_cursors(&pagination_params, &mut rows);

        let items = rows
            .iter()
            .map(|row| Ok(WorkoutRes::from_entity(row.clone())))
            .collect::<Result<Vec<WorkoutRes>, String>>()?;

        Ok(PaginationRes::new(items, cursors))
    }

    fn pagination_filters(
        &self,
        filter_req: Option<WorkoutsFilterReq>,
        qb: &mut QueryBuilder<Sqlite>,
    ) {
        if let Some(req) = filter_req {
            if let Some(name) = req.name {
                qb.push(" AND name LIKE ");
                qb.push_bind(format!("%{}%", name));
            }

            if let Some(description) = req.description {
                qb.push(" AND description LIKE ");
                qb.push_bind(format!("%{}%", description));
            }

            if let Some(active) = req.active {
                qb.push(" AND active = ");
                qb.push_bind(active);
            }
        }
    }

    // --- WORKOUT EXERCISE ---

    pub async fn create_workout_exercise(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        req: WorkoutExerciseReq,
    ) -> Result<u32, String> {
        let created_at = Utc::now();

        let result = sqlx::query(
            r#"
        INSERT INTO workout_exercises (
            created_at, workout_id, name, code, sets_target, reps_or_seconds_target, working_weight,
            rest_period_seconds, tempo, emom, equipments, bands, description
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(created_at)
        .bind(req.workout_id)
        .bind(&req.name)
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
        .map_err(|e| format!("Failed to create core exercise: {}", e))?;

        let id = result.last_insert_rowid() as u32;
        Ok(id)
    }

    pub async fn update_workout_exercise(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        id: u32,
        req: WorkoutExerciseReq,
    ) -> Result<(), String> {
        let result = sqlx::query(
            r#"
        UPDATE workout_exercises
        SET workout_id = ?, code = ?, name = ?,
            sets_target = ?, reps_or_seconds_target = ?, working_weight = ?,
            rest_period_seconds = ?, tempo = ?, emom = ?, equipments = ?,
            bands = ?, description = ?
        WHERE id = ?
        "#,
        )
        .bind(req.workout_id)
        .bind(&req.code)
        .bind(&req.name)
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
        .map_err(|e| format!("Failed to update core exercise: {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Workout exercise not found".to_string());
        }

        Ok(())
    }

    pub async fn delete_workout_exercise(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        id: u32,
    ) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM workout_exercises WHERE id = ?")
            .bind(id)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Failed to delete core exercise (may be in use): {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Workout exercise not found".to_string());
        }

        Ok(())
    }

    pub async fn get_workout_exercises_by_workout_id<'e, E: Executor<'e, Database = Sqlite>>(
        &self,
        executor: E,
        workout_id: u32,
    ) -> Result<Vec<WorkoutExerciseRes>, String> {
        let rows: Vec<WorkoutExerciseEntity> =
            sqlx::query_as("SELECT * FROM workout_exercises WHERE workout_id = ? ORDER BY code")
                .bind(workout_id)
                .fetch_all(executor)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

        let res = rows
            .iter()
            .map(|row| Ok(WorkoutExerciseRes::from_entity(row.clone())))
            .collect::<Result<Vec<WorkoutExerciseRes>, String>>()?;

        Ok(res)
    }

    pub async fn get_one_workout_exercise<'e, E: Executor<'e, Database = Sqlite>>(
        &self,
        executor: E,
        id: u32,
    ) -> Result<WorkoutExerciseRes, String> {
        let entity: WorkoutExerciseEntity =
            sqlx::query_as("SELECT * FROM workout_exercises WHERE id = ?")
                .bind(id)
                .fetch_optional(executor)
                .await
                .map_err(|e| format!("Database error: {}", e))?
                .ok_or_else(|| "Workout exercise not found".to_string())?;

        Ok(WorkoutExerciseRes::from_entity(entity))
    }
}

#[cfg(test)]
mod tests {
    use crate::db::pagination_support::{PaginationDirection, PaginationParams};
    use crate::db::{IN_MEMORY_DB_URL, init_db};
    use crate::enums::{Band, Equipment};
    use crate::workout::workout_dto::{WorkoutExerciseReq, WorkoutReq, WorkoutsFilterReq};
    use crate::workout::workout_repo::WorkoutRepo;
    use chrono::Utc;
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

    fn mock_workout_exercise_req(workout_id: u32, code: &str, name: &str) -> WorkoutExerciseReq {
        WorkoutExerciseReq {
            workout_id,
            code: code.to_string(),
            name: name.to_string(),
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
        let repository = WorkoutRepo::new();

        // Create core
        let workout_id = repository
            .create_workout(&mut tx, mock_workout_req("Upper Body A"))
            .await
            .expect("Failed to create core");

        // Get
        let workout = repository
            .get_one_workout(&mut *tx, workout_id)
            .await
            .expect("Failed to get core");
        assert_eq!(workout.name, "Upper Body A");

        // Update
        let mut updated_req = mock_workout_req("Lower Body A");
        updated_req.description = Some("Leg day!".to_string());
        repository
            .update_workout(&mut tx, workout_id, updated_req)
            .await
            .expect("Failed to update core");

        let updated_workout = repository
            .get_one_workout(&mut *tx, workout_id)
            .await
            .expect("Failed to get updated core");
        assert_eq!(updated_workout.name, "Lower Body A");
        assert_eq!(updated_workout.description, Some("Leg day!".to_string()));

        // Delete
        repository
            .delete_workout(&mut tx, workout_id)
            .await
            .expect("Failed to delete core");

        assert!(
            repository
                .get_one_workout(&mut *tx, workout_id)
                .await
                .is_err()
        );

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_pagination() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutRepo::new();

        // 1. Seed data: Create 5 workouts
        // Names: "Workout A", "Workout B", "Workout C", "Hidden D", "Workout E"
        for i in 0..5 {
            let name = if i == 3 {
                "Hidden D".to_string()
            } else {
                format!("Workout {}", (b'A' + i as u8) as char)
            };

            let mut req = mock_workout_req(&name);
            if i == 3 {
                req.active = false;
            }
            repository.create_workout(&mut tx, req).await.unwrap();
        }

        // 2. Test simple pagination (First 2 items)
        let params = PaginationParams {
            limit: 2,
            cursor: None,
            direction: PaginationDirection::Forward,
        };
        let res = repository
            .paginate_workouts(&mut *tx, None, params)
            .await
            .expect("Pagination failed");

        assert_eq!(res.items.len(), 2);
        assert_eq!(res.items[0].name, "Workout A");
        assert_eq!(res.items[1].name, "Workout B");
        assert!(res.next_cursor.is_some());

        // 3. Test next page (Next 2 items)
        let params_page2 = PaginationParams {
            limit: 2,
            cursor: res.next_cursor,
            direction: PaginationDirection::Forward,
        };
        let res_page2 = repository
            .paginate_workouts(&mut *tx, None, params_page2)
            .await
            .expect("Pagination page 2 failed");

        assert_eq!(res_page2.items.len(), 2);
        assert_eq!(res_page2.items[0].name, "Workout C");
        assert_eq!(res_page2.items[1].name, "Hidden D");

        // 4. Test filtering (Active only)
        let filter = WorkoutsFilterReq {
            name: None,
            description: None,
            active: Some(true),
        };
        let params_all = PaginationParams {
            limit: 10,
            cursor: None,
            direction: PaginationDirection::Forward,
        };
        let res_filtered = repository
            .paginate_workouts(&mut *tx, Some(filter), params_all.clone())
            .await
            .expect("Filtered pagination failed");

        // Should be 4 items (A, B, C, E), excluding Hidden D
        assert_eq!(res_filtered.items.len(), 4);
        assert!(res_filtered.items.iter().all(|w| w.active));

        // 5. Test filtering by name
        let name_filter = WorkoutsFilterReq {
            name: Some("Hidden".to_string()),
            description: None,
            active: None,
        };
        let res_name = repository
            .paginate_workouts(&mut *tx, Some(name_filter), params_all.clone())
            .await
            .expect("Name filter failed");

        assert_eq!(res_name.items.len(), 1);
        assert_eq!(res_name.items[0].name, "Hidden D");

        tx.commit().await.unwrap();
    }

    // --- WORKOUT EXERCISE TESTS ---

    #[tokio::test]
    async fn test_workout_exercise_crud() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutRepo::new();

        // Setup dependencies
        let workout_id = repository
            .create_workout(&mut tx, mock_workout_req("Test Workout"))
            .await
            .unwrap();

        // Create
        let ex_id = repository
            .create_workout_exercise(
                &mut tx,
                mock_workout_exercise_req(workout_id, "A1", "Pushups"),
            )
            .await
            .expect("Failed to create core exercise");

        // Get
        let ex = repository
            .get_one_workout_exercise(&mut *tx, ex_id)
            .await
            .expect("Failed to get core exercise");
        assert_eq!(ex.code, "A1");
        assert_eq!(ex.workout_id, workout_id);
        assert_eq!(ex.equipments, vec![Equipment::Barbell]);
        assert_eq!(ex.bands, vec![Band::Black]);
        assert_eq!(ex.tempo, "2010");

        // Update
        let mut updated_req = mock_workout_exercise_req(workout_id, "A2", "Pullups");
        updated_req.sets_target = 5;
        updated_req.working_weight = 120;
        updated_req.equipments = vec![Equipment::Dumbbells];
        updated_req.bands = vec![];
        updated_req.description = None;

        repository
            .update_workout_exercise(&mut tx, ex_id, updated_req)
            .await
            .expect("Failed to update core exercise");

        let updated_ex = repository
            .get_one_workout_exercise(&mut *tx, ex_id)
            .await
            .expect("Failed to get updated exercise");
        assert_eq!(updated_ex.code, "A2");
        assert_eq!(updated_ex.name, "Pullups");
        assert_eq!(updated_ex.sets_target, 5);
        assert_eq!(updated_ex.working_weight, 120);
        assert_eq!(updated_ex.equipments, vec![Equipment::Dumbbells]);
        assert!(updated_ex.bands.is_empty());
        assert_eq!(updated_ex.description, None);

        repository
            .create_workout_exercise(
                &mut tx,
                mock_workout_exercise_req(workout_id, "B1", "Triceps extension"),
            )
            .await
            .expect("Failed to create core exercise");

        let found = repository
            .get_workout_exercises_by_workout_id(&mut *tx, workout_id)
            .await
            .unwrap();
        assert_eq!(found.len(), 2);

        // Delete
        repository
            .delete_workout_exercise(&mut tx, ex_id)
            .await
            .expect("Failed to delete core exercise");

        assert!(
            repository
                .get_one_workout_exercise(&mut *tx, ex_id)
                .await
                .is_err()
        );

        tx.commit().await.unwrap();
    }

    // --- INTEGRITY TESTS ---

    #[tokio::test]
    async fn test_can_delete_workout_with_exercises() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutRepo::new();

        let workout_id = repository
            .create_workout(&mut tx, mock_workout_req("Protected Workout"))
            .await
            .unwrap();

        // Create a core exercise → now core is referenced
        repository
            .create_workout_exercise(&mut tx, mock_workout_exercise_req(1, "A1", "Pushups"))
            .await
            .unwrap();

        // Attempt to delete core → should fail due to RESTRICT
        let result = repository.delete_workout(&mut tx, workout_id).await;
        assert!(result.is_ok());

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_cannot_delete_workout_exercise_with_logs() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutRepo::new();

        let workout_id = repository
            .create_workout(&mut tx, mock_workout_req("Logged Workout"))
            .await
            .unwrap();

        let ex_id = repository
            .create_workout_exercise(&mut tx, mock_workout_exercise_req(1, "A1", "Pushups"))
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

        // Create a log entry referencing the core exercise
        sqlx::query(
            r#"INSERT INTO workout_logs (
                workout_id, workout_exercise_id, workout_log_group_id,
                exercise_name, set_number, rep_number_or_seconds, weight, description
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(workout_id)
        .bind(ex_id)
        .bind(log_group_id as u32)
        .bind("Pushups")
        .bind(1u8)
        .bind(8u8)
        .bind(100u32)
        .bind("Completed")
        .execute(&mut *tx)
        .await
        .unwrap();

        // Now try to delete the core exercise → should fail
        let result = repository.delete_workout_exercise(&mut tx, ex_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("may be in use"));

        tx.commit().await.unwrap();
    }
}

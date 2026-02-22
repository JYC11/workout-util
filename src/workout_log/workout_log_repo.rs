use crate::db::pagination_support::{
    PaginationParams, PaginationRes, get_cursors, keyset_paginate,
};
use crate::db::{SqliteExecutor, SqliteTx};
use crate::workout_log::workout_log_dto::{
    WorkoutLogDetailRes, WorkoutLogGroupFilterReq, WorkoutLogGroupPageRes, WorkoutLogGroupReq,
    WorkoutLogGroupRes, WorkoutLogReq,
};
use crate::workout_log::workout_log_entity::WorkoutLogGroupEntity;
use chrono::Utc;
use sqlx::{QueryBuilder, Sqlite};

#[derive(Clone, Copy)]
pub struct WorkoutLogRepo {}

impl WorkoutLogRepo {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_log_group(
        &self,
        tx: &mut SqliteTx<'_>,
        req: WorkoutLogGroupReq,
    ) -> Result<u32, String> {
        let created_at = Utc::now();

        let result = sqlx::query(
            r#"INSERT INTO workout_log_groups (created_at, date, notes) VALUES (?, ?, ?)"#,
        )
        .bind(created_at)
        .bind(req.date)
        .bind(req.notes)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Failed to create log group: {}", e))?;

        Ok(result.last_insert_rowid() as u32)
    }

    pub async fn delete_log_group(&self, tx: &mut SqliteTx<'_>, id: u32) -> Result<(), String> {
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

    pub async fn get_one_log_group<'e>(
        &self,
        executor: impl SqliteExecutor<'e>,
        id: u32,
    ) -> Result<WorkoutLogGroupRes, String> {
        let entity: WorkoutLogGroupEntity =
            sqlx::query_as("SELECT * FROM workout_log_groups WHERE id = ?")
                .bind(id)
                .fetch_optional(executor)
                .await
                .map_err(|e| format!("Database error: {}", e))?
                .ok_or_else(|| "Workout log group not found".to_string())?;
        Ok(WorkoutLogGroupRes::from_entity(entity))
    }

    pub async fn create_log(
        &self,
        tx: &mut SqliteTx<'_>,
        req: WorkoutLogReq,
    ) -> Result<u32, String> {
        let result = sqlx::query(
            r#"INSERT INTO workout_logs (
            workout_id, workout_exercise_id, workout_log_group_id,
            exercise_name, set_number, rep_number_or_seconds, weight, description
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(req.workout_id)
        .bind(req.workout_exercise_id)
        .bind(req.workout_log_group_id)
        .bind(req.exercise_name)
        .bind(req.set_number)
        .bind(req.rep_number_or_seconds)
        .bind(req.weight)
        .bind(req.description)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Failed to create core log: {}", e))?;

        Ok(result.last_insert_rowid() as u32)
    }

    pub async fn delete_log(&self, tx: &mut SqliteTx<'_>, id: u32) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM workout_logs WHERE id = ?")
            .bind(id)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Failed to delete core log: {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Workout log not found".to_string());
        }

        Ok(())
    }

    pub async fn get_logs_by_workout_log_group_id<'e>(
        &self,
        executor: impl SqliteExecutor<'e>,
        workout_log_group_id: u32,
    ) -> Result<Vec<WorkoutLogDetailRes>, String> {
        let res: Vec<WorkoutLogDetailRes> = sqlx::query_as(
            r#"
                SELECT wl.id,
                       wlg.id AS workout_log_group_id,
                       wlg.date AS workout_date,
                       wo.id AS workout_id,
                       wo.name AS workout_name,
                       wl.workout_exercise_id,
                       wl.exercise_name AS workout_exercise_name,
                       wl.set_number,
                       wl.rep_number_or_seconds,
                       wl.weight,
                       wl.description
                FROM workout_logs wl
                JOIN workouts wo ON wl.workout_id = wo.id
                JOIN workout_log_groups wlg ON wl.workout_log_group_id = wlg.id
                WHERE wl.workout_log_group_id = ?
                "#,
        )
        .bind(workout_log_group_id)
        .fetch_all(executor)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(res)
    }

    pub async fn paginate_workout_log_groups<'e>(
        &self,
        executor: impl SqliteExecutor<'e>,
        pagination_filters: Option<WorkoutLogGroupFilterReq>,
        pagination_params: PaginationParams,
    ) -> Result<PaginationRes<WorkoutLogGroupPageRes>, String> {
        let mut qb = QueryBuilder::new("SELECT * FROM workout_log_groups WHERE 1=1");
        self.log_group_pagination_filters(pagination_filters, &mut qb);
        keyset_paginate(&pagination_params, None, &mut qb);

        let mut rows: Vec<WorkoutLogGroupPageRes> =
            qb.build_query_as()
                .fetch_all(executor)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

        let cursors = get_cursors(&pagination_params, &mut rows);

        Ok(PaginationRes::new(rows, cursors))
    }

    fn log_group_pagination_filters(
        &self,
        filter_req: Option<WorkoutLogGroupFilterReq>,
        qb: &mut QueryBuilder<Sqlite>,
    ) {
        if let Some(req) = filter_req {
            if let Some(date_gte) = req.workout_date_gte {
                qb.push(" AND date >= ");
                qb.push_bind(date_gte);
            }

            if let Some(date_lte) = req.workout_date_lte {
                qb.push(" AND date <= ");
                qb.push_bind(date_lte);
            }

            if let Some(notes) = req.notes {
                qb.push(" AND notes LIKE ");
                qb.push_bind(format!("%{}%", notes));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::pagination_support::{PaginationDirection, PaginationParams};
    use crate::db::{IN_MEMORY_DB_URL, SqliteTx, init_db};
    use crate::enums::{Band, Equipment};
    use crate::workout_log::workout_log_dto::{
        WorkoutLogGroupFilterReq, WorkoutLogGroupReq, WorkoutLogReq,
    };
    use crate::workout_log::workout_log_repo::WorkoutLogRepo;
    use chrono::{NaiveDate, Utc};
    use sqlx::SqlitePool;

    async fn setup_db() -> SqlitePool {
        init_db(IN_MEMORY_DB_URL).await
    }

    // Helper: Create dummy data
    async fn create_workout_exercise(tx: &mut SqliteTx<'_>) -> (u32, u32) {
        // (plan_id, workout_id, exercise_id)
        let workout_id = sqlx::query(
            "INSERT INTO workouts (created_at, name, description, active) VALUES (?, ?, ?, ?)",
        )
        .bind(Utc::now())
        .bind("Test Workout")
        .bind(Option::<String>::None)
        .bind(true)
        .execute(&mut **tx)
        .await
        .unwrap()
        .last_insert_rowid() as u32;

        sqlx::query(
            r#"INSERT INTO workout_exercises (
                created_at, workout_id, code, name,
                sets_target, reps_or_seconds_target, working_weight,
                rest_period_seconds, tempo, emom, equipments, bands, description
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(Utc::now())
        .bind(workout_id)
        .bind("A1")
        .bind("Dummy Ex")
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

        (workout_id, workout_exercise_id)
    }

    #[tokio::test]
    async fn test_log_group_crud() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutLogRepo::new();

        let today = Utc::now().naive_utc().date();
        let workout_log_group_req = WorkoutLogGroupReq {
            date: today,
            notes: Some("Morning session".to_string()),
        };

        // Create
        let group_id = repository
            .create_log_group(&mut tx, workout_log_group_req)
            .await
            .expect("Failed to create log group");

        // Get
        let group = repository
            .get_one_log_group(&mut *tx, group_id)
            .await
            .expect("Failed to get log group");
        assert_eq!(group.date, today);
        assert_eq!(group.notes, Some("Morning session".to_string()));

        // Delete
        repository
            .delete_log_group(&mut tx, group_id)
            .await
            .expect("Failed to delete log group");

        assert!(
            repository
                .get_one_log_group(&mut *tx, group_id)
                .await
                .is_err()
        );

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_workout_log_crud() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutLogRepo::new();

        let (workout_id, workout_exercise_id) = create_workout_exercise(&mut tx).await;

        let today = Utc::now().naive_utc().date();
        let workout_log_group_req = WorkoutLogGroupReq {
            date: today,
            notes: None,
        };

        let group_id = repository
            .create_log_group(&mut tx, workout_log_group_req)
            .await
            .unwrap();

        // Create log
        let log_req = WorkoutLogReq {
            workout_id,
            workout_exercise_id,
            workout_log_group_id: group_id,
            exercise_name: "Dummy Ex".to_string(),
            set_number: 2,
            rep_number_or_seconds: 8,
            weight: 95,
            description: Some("Felt strong".to_string()),
        };

        let log_id = repository
            .create_log(&mut tx, log_req)
            .await
            .expect("Failed to create logs");

        // Get
        let logs = repository
            .get_logs_by_workout_log_group_id(&mut *tx, group_id)
            .await
            .expect("Failed to get logs");
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.workout_id, workout_id);
        assert_eq!(log.workout_exercise_id, workout_exercise_id);
        assert_eq!(log.workout_log_group_id, group_id);
        assert_eq!(log.set_number, 2);
        assert_eq!(log.weight, 95);
        assert_eq!(log.description, Some("Felt strong".to_string()));

        // Delete
        repository
            .delete_log(&mut tx, log_id)
            .await
            .expect("Failed to delete core log");

        let found_again = repository
            .get_logs_by_workout_log_group_id(&mut *tx, group_id)
            .await
            .expect("Failed to get logs");

        assert_eq!(found_again.len(), 0);

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_paginate_workout_log_groups() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutLogRepo::new();

        // 1. Seed Data
        let date1 = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let req1 = WorkoutLogGroupReq {
            date: date1,
            notes: Some("Note 1".to_string()),
        };
        let _g1_id = repository.create_log_group(&mut tx, req1).await.unwrap();

        let date2 = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();
        let req2 = WorkoutLogGroupReq {
            date: date2,
            notes: Some("Note 2".to_string()),
        };
        let _g2_id = repository.create_log_group(&mut tx, req2).await.unwrap();

        let date3 = NaiveDate::from_ymd_opt(2023, 1, 3).unwrap();
        let req3 = WorkoutLogGroupReq {
            date: date3,
            notes: Some("Note 3".to_string()),
        };
        let _g3_id = repository.create_log_group(&mut tx, req3).await.unwrap();

        // 2. Test Simple Pagination
        let params = PaginationParams {
            limit: 2,
            cursor: None,
            direction: PaginationDirection::Forward,
        };
        let res = repository
            .paginate_workout_log_groups(&mut *tx, None, params)
            .await
            .expect("Pagination failed");

        assert_eq!(res.items.len(), 2);
        assert_eq!(res.items[0].date, date1);
        assert!(res.next_cursor.is_some());

        // 3. Test Next Page
        let params_next = PaginationParams {
            limit: 2,
            cursor: res.next_cursor,
            direction: PaginationDirection::Forward,
        };
        let res_next = repository
            .paginate_workout_log_groups(&mut *tx, None, params_next)
            .await
            .expect("Pagination page 2 failed");

        assert_eq!(res_next.items.len(), 1);
        assert_eq!(res_next.items[0].date, date3);

        // 4. Test Filter by Date (>= 2023-01-02)
        let filter_date = WorkoutLogGroupFilterReq {
            workout_date_gte: Some(date2),
            workout_date_lte: None,
            notes: None,
        };
        let params_all = PaginationParams {
            limit: 10,
            cursor: None,
            direction: PaginationDirection::Forward,
        };
        let res_filtered = repository
            .paginate_workout_log_groups(&mut *tx, Some(filter_date), params_all)
            .await
            .expect("Filtered pagination failed");

        assert_eq!(res_filtered.items.len(), 2);
        for item in res_filtered.items {
            assert!(item.date >= date2);
        }

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_cannot_delete_log_group_with_logs() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = WorkoutLogRepo::new();

        let (workout_id, workout_exercise_id) = create_workout_exercise(&mut tx).await;
        let today = Utc::now().naive_utc().date();
        let workout_log_group_req = WorkoutLogGroupReq {
            date: today,
            notes: None,
        };
        let group_id = repository
            .create_log_group(&mut tx, workout_log_group_req)
            .await
            .unwrap();

        // Create a log referencing the group
        let log_req = WorkoutLogReq {
            workout_id,
            workout_exercise_id,
            workout_log_group_id: group_id,
            exercise_name: "Dummy Ex".to_string(),
            set_number: 1,
            rep_number_or_seconds: 10,
            weight: 80,
            description: None,
        };
        repository.create_log(&mut tx, log_req).await.unwrap();

        // Attempt to delete group → should fail
        let result = repository.delete_log_group(&mut tx, group_id).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("may be in use"));

        tx.commit().await.unwrap();
    }
}

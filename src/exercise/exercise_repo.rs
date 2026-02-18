use crate::db::pagination_support::{
    get_cursors, keyset_paginate, PaginationParams, PaginationRes,
};
use crate::enums::{
    CompoundOrIsolation, Grip, GripWidth, LeverVariation, PushOrPull, SquatOrHinge,
    StraightOrBentArm, UpperOrLower,
};
use crate::exercise::exercise_dto::{
    ExerciseLibraryFilterReq, ExerciseLibraryReq, ExerciseLibraryRes, ValidExercise,
};
use crate::exercise::exercise_entity::ExerciseLibraryEntity;
use sqlx::{Executor, QueryBuilder, Sqlite, Transaction};

#[derive(Clone, Copy)]
pub struct ExerciseRepo {}

impl ExerciseRepo {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_exercise(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        req: ExerciseLibraryReq,
    ) -> Result<u32, String> {
        let entity = ExerciseLibraryEntity::from_req(req)?;

        let result = sqlx::query(
            r#"
        INSERT INTO exercise_library (
            name, push_or_pull, dynamic_or_static, straight_or_bent, squat_or_hinge,
            upper_or_lower, compound_or_isolation, lever_variation, grip, grip_width, description
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        )
            .bind(entity.name)
            .bind(entity.push_or_pull)
            .bind(entity.dynamic_or_static)
            .bind(entity.straight_or_bent)
            .bind(entity.squat_or_hinge)
            .bind(entity.upper_or_lower)
            .bind(entity.compound_or_isolation)
            .bind(entity.lever_variation)
            .bind(entity.grip)
            .bind(entity.grip_width)
            .bind(entity.description)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Failed to create exercise: {}", e))?;

        let id = result.last_insert_rowid() as u32;

        Ok(id)
    }

    pub async fn update_exercise(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        valid_exercise: ValidExercise,
    ) -> Result<(), String> {
        let (
            id,
            name,
            push_or_pull,
            dynamic_or_static,
            straight_or_bent,
            squat_or_hinge,
            upper_or_lower,
            compound_or_isolation,
            lever_variation,
            grip,
            grip_width,
        ) = match valid_exercise {
            ValidExercise::StraightArmCompound(e) => (
                e.id,
                e.name,
                Some(e.push_or_pull),
                e.dynamic_or_static,
                Some(StraightOrBentArm::Straight), // Explicitly set
                None::<SquatOrHinge>,
                UpperOrLower::Upper,
                CompoundOrIsolation::Compound,
                Some(e.lever_variation), // Always Some
                Some(e.grip),
                Some(e.grip_width),
            ),
            ValidExercise::BentArmCompound(e) => (
                e.id,
                e.name,
                Some(e.push_or_pull),
                e.dynamic_or_static,
                Some(StraightOrBentArm::Bent), // Explicitly set
                None::<SquatOrHinge>,
                UpperOrLower::Upper,
                CompoundOrIsolation::Compound,
                e.lever_variation, // Could be None
                Some(e.grip),
                Some(e.grip_width),
            ),
            ValidExercise::UpperBodyIsolation(e) => (
                e.id,
                e.name,
                None::<PushOrPull>,
                e.dynamic_or_static,
                Some(e.straight_or_bent),
                None::<SquatOrHinge>,
                UpperOrLower::Upper,
                CompoundOrIsolation::Isolation,
                None::<LeverVariation>,
                None::<Grip>,
                None::<GripWidth>,
            ),
            ValidExercise::LowerBodyCompound(e) => (
                e.id,
                e.name,
                None::<PushOrPull>,
                e.dynamic_or_static,
                None::<StraightOrBentArm>,
                Some(e.squat_or_hinge),
                UpperOrLower::Lower,
                CompoundOrIsolation::Compound,
                None::<LeverVariation>,
                None::<Grip>,
                None::<GripWidth>,
            ),
            ValidExercise::LowerBodyIsolation(e) => (
                e.id,
                e.name,
                None::<PushOrPull>,
                e.dynamic_or_static,
                None::<StraightOrBentArm>,
                None::<SquatOrHinge>,
                UpperOrLower::Lower,
                CompoundOrIsolation::Isolation,
                None::<LeverVariation>,
                None::<Grip>,
                None::<GripWidth>,
            ),
        };

        let result = sqlx::query(
            r#"
        UPDATE exercise_library SET
            name = ?, push_or_pull = ?, dynamic_or_static = ?, straight_or_bent = ?,
            squat_or_hinge = ?, upper_or_lower = ?, compound_or_isolation = ?,
            lever_variation = ?, grip = ?, grip_width = ?
        WHERE id = ?
        "#,
        )
            .bind(name)
            .bind(push_or_pull)
            .bind(dynamic_or_static)
            .bind(straight_or_bent)
            .bind(squat_or_hinge)
            .bind(upper_or_lower)
            .bind(compound_or_isolation)
            .bind(lever_variation)
            .bind(grip)
            .bind(grip_width)
            .bind(id)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Failed to update exercise: {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Exercise not found".to_string());
        }

        Ok(())
    }

    pub async fn delete_exercise(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        exercise_id: u32,
    ) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM exercise_library WHERE id = ?")
            .bind(exercise_id)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Failed to delete exercise (might be in use): {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Exercise not found".to_string());
        }

        Ok(())
    }

    pub async fn get_one_exercise<'e, E: Executor<'e, Database=Sqlite>>(
        &self,
        executor: E,
        exercise_id: u32,
    ) -> Result<ValidExercise, String> {
        let row: ExerciseLibraryEntity =
            sqlx::query_as("SELECT * FROM exercise_library WHERE id = ?")
                .bind(exercise_id)
                .fetch_optional(executor)
                .await
                .map_err(|e| format!("Database error: {}", e))?
                .ok_or_else(|| "Exercise not found".to_string())?;

        row.to_valid_struct()
    }

    pub async fn paginate_exercises<'e, E: Executor<'e, Database=Sqlite>>(
        &self,
        executor: E,
        filter_req: Option<ExerciseLibraryFilterReq>,
        pagination_params: PaginationParams,
    ) -> Result<PaginationRes<ExerciseLibraryRes>, String> {
        let mut qb = QueryBuilder::new("SELECT * FROM exercise_library WHERE 1=1");
        self.pagination_filters(filter_req, &mut qb);
        keyset_paginate(&pagination_params, &mut qb);

        let mut rows: Vec<ExerciseLibraryRes> = qb
            .build_query_as()
            .fetch_all(executor)
            .await
            .map_err(|e| format!("Failed to paginate exercises: {}", e))?;

        let cursors = get_cursors(&pagination_params, &mut rows);
        Ok(PaginationRes {
            items: rows,
            next_cursor: cursors.next_cursor,
            prev_cursor: cursors.prev_cursor,
        })
    }

    fn pagination_filters(
        &self,
        filter_req: Option<ExerciseLibraryFilterReq>,
        qb: &mut QueryBuilder<Sqlite>,
    ) {
        if let Some(req) = filter_req {
            if let Some(name) = req.name {
                qb.push(" AND name LIKE ");
                qb.push_bind(format!("%{}%", name));
            }

            if let Some(vals) = req.push_or_pull {
                if !vals.is_empty() {
                    qb.push(" AND push_or_pull IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.dynamic_or_static {
                if !vals.is_empty() {
                    qb.push(" AND dynamic_or_static IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.straight_or_bent {
                if !vals.is_empty() {
                    qb.push(" AND straight_or_bent IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.squat_or_hinge {
                if !vals.is_empty() {
                    qb.push(" AND squat_or_hinge IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.upper_or_lower {
                if !vals.is_empty() {
                    qb.push(" AND upper_or_lower IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.compound_or_isolation {
                if !vals.is_empty() {
                    qb.push(" AND compound_or_isolation IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.lever_variation {
                if !vals.is_empty() {
                    qb.push(" AND lever_variation IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.grip {
                if !vals.is_empty() {
                    qb.push(" AND grip IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }

            if let Some(vals) = req.grip_width {
                if !vals.is_empty() {
                    qb.push(" AND grip_width IN (");
                    let mut sep = qb.separated(", ");
                    for v in vals {
                        sep.push_bind(v);
                    }
                    sep.push_unseparated(")");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::pagination_support::{PaginationDirection, PaginationParams};
    use crate::db::{init_db, IN_MEMORY_DB_URL};
    use crate::enums::*;
    use crate::exercise::exercise_dto::{
        ExerciseLibraryFilterReq, ExerciseLibraryReq, ValidExercise,
    };
    use crate::exercise::exercise_repo::ExerciseRepo;
    use sqlx::SqlitePool;

    async fn setup_db() -> SqlitePool {
        init_db(IN_MEMORY_DB_URL).await
    }

    // Helper to create a valid request for an Upper Body Compound exercise
    fn mock_upper_compound_req(name: &str) -> ExerciseLibraryReq {
        ExerciseLibraryReq {
            name: name.to_string(),
            push_or_pull: Some(PushOrPull::Push),
            dynamic_or_static: DynamicOrStatic::Dynamic,
            straight_or_bent: Some(StraightOrBentArm::Bent),
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Upper,
            compound_or_isolation: CompoundOrIsolation::Compound,
            lever_variation: None,
            grip: Some(Grip::Pronated),
            grip_width: Some(GripWidth::Shoulder),
            description: Some("Test description".to_string()),
        }
    }

    #[tokio::test]
    async fn test_create_and_get_exercise() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = ExerciseRepo::new();

        let req = mock_upper_compound_req("Bench Press");

        // 1. Test Create
        repository
            .create_exercise(&mut tx, req)
            .await
            .expect("Failed to create exercise");

        // 2. Test Get One (ID should be 1 for first entry)
        let result = repository
            .get_one_exercise(&mut *tx, 1)
            .await
            .expect("Failed to get exercise");

        match result {
            ValidExercise::BentArmCompound(e) => {
                assert_eq!(e.id, 1);
                assert_eq!(e.name, "Bench Press");
                assert_eq!(e.push_or_pull, PushOrPull::Push);
            }
            _ => panic!("Exercise type mismatch, expected UpperBodyCompound"),
        }

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_paginate_exercises() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = ExerciseRepo::new();

        let req = mock_upper_compound_req("Bench Press");

        // 1. Test Create
        repository
            .create_exercise(&mut tx, req)
            .await
            .expect("Failed to create exercise");

        // 2. Test Paginate (ID should be 1 for first entry)
        let result = repository
            .paginate_exercises(
                &mut *tx,
                None,
                PaginationParams {
                    limit: 1,
                    cursor: None,
                    direction: PaginationDirection::Forward,
                },
            )
            .await
            .expect("Failed to get exercise");

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].id, 1);
        assert_eq!(result.next_cursor, None); // Only 1 item, no next page

        // Create another exercise to test pagination boundaries
        let req2 = mock_upper_compound_req("Overhead Press");
        repository.create_exercise(&mut tx, req2).await.unwrap(); // ID 2

        // Test Paginate Page 1 (Limit 1)
        let result = repository
            .paginate_exercises(
                &mut *tx,
                None,
                PaginationParams {
                    limit: 1,
                    cursor: None,
                    direction: PaginationDirection::Forward,
                },
            )
            .await
            .unwrap();
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].id, 1);
        assert_eq!(result.next_cursor, Some(1));
        assert_eq!(result.prev_cursor, None);

        // Test Paginate Page 2 (Limit 1, Cursor 1)
        let result = repository
            .paginate_exercises(
                &mut *tx,
                None,
                PaginationParams {
                    limit: 1,
                    cursor: Some(1),
                    direction: PaginationDirection::Forward,
                },
            )
            .await
            .unwrap();
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].id, 2);
        assert_eq!(result.next_cursor, None);
        assert_eq!(result.prev_cursor, Some(2)); // Can go back

        // Test Paginate Backward from Page 2 (Cursor 2)
        let result = repository
            .paginate_exercises(
                &mut *tx,
                None,
                PaginationParams {
                    limit: 1,
                    cursor: Some(2),
                    direction: PaginationDirection::Backward,
                },
            )
            .await
            .unwrap();
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].id, 1);
        assert_eq!(result.next_cursor, Some(1)); // Can go forward
        assert_eq!(result.prev_cursor, None); // No more previous

        // 3. Test cursor at 2 should return no results (Forward)
        let result = repository
            .paginate_exercises(
                &mut *tx,
                None,
                PaginationParams {
                    limit: 1,
                    cursor: Some(2),
                    direction: PaginationDirection::Forward,
                },
            )
            .await
            .expect("Failed to get exercise");

        assert_eq!(result.items.len(), 0);

        // 4. Test filter by type Pull should return no results
        let result = repository
            .paginate_exercises(
                &mut *tx,
                Some(ExerciseLibraryFilterReq {
                    name: None,
                    push_or_pull: Some(vec![PushOrPull::Pull]),
                    dynamic_or_static: None,
                    straight_or_bent: None,
                    squat_or_hinge: None,
                    upper_or_lower: None,
                    compound_or_isolation: None,
                    lever_variation: None,
                    grip: None,
                    grip_width: None,
                }),
                PaginationParams {
                    limit: 1,
                    cursor: None,
                    direction: PaginationDirection::Forward,
                },
            )
            .await
            .expect("Failed to get exercise");

        assert_eq!(result.items.len(), 0);

        // 4. Test filter by type Push should return 1 result
        let result = repository
            .paginate_exercises(
                &mut *tx,
                Some(ExerciseLibraryFilterReq {
                    name: None,
                    push_or_pull: Some(vec![PushOrPull::Push]),
                    dynamic_or_static: None,
                    straight_or_bent: None,
                    squat_or_hinge: None,
                    upper_or_lower: None,
                    compound_or_isolation: None,
                    lever_variation: None,
                    grip: None,
                    grip_width: None,
                }),
                PaginationParams {
                    limit: 1,
                    cursor: None,
                    direction: PaginationDirection::Forward,
                },
            )
            .await
            .expect("Failed to get exercise");

        assert_eq!(result.items.len(), 1);

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_update_exercise() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = ExerciseRepo::new();

        // Setup: Create initial exercise
        let req = mock_upper_compound_req("Old Name");
        repository.create_exercise(&mut tx, req).await.unwrap();

        // Get the valid struct
        let mut exercise = repository.get_one_exercise(&mut *tx, 1).await.unwrap();

        // Modify it
        if let ValidExercise::BentArmCompound(ref mut e) = exercise {
            e.name = "New Name".to_string();
            // Change a property to ensure update logic works
            e.grip_width = GripWidth::Wide;
        }

        // 3. Test Update
        repository
            .update_exercise(&mut tx, exercise)
            .await
            .expect("Failed to update");

        // Verify changes
        let updated = repository.get_one_exercise(&mut *tx, 1).await.unwrap();
        if let ValidExercise::BentArmCompound(e) = updated {
            assert_eq!(e.name, "New Name");
            assert_eq!(e.grip_width, GripWidth::Wide);
        } else {
            panic!("Wrong type returned after update");
        }

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_exercise() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();
        let repository = ExerciseRepo::new();

        // Setup: Create
        let req = mock_upper_compound_req("To Delete");
        repository.create_exercise(&mut tx, req).await.unwrap();

        // Ensure it exists
        assert!(repository.get_one_exercise(&mut *tx, 1).await.is_ok());

        // 4. Test Delete
        repository
            .delete_exercise(&mut tx, 1)
            .await
            .expect("Failed to delete");

        // Verify it is gone
        let result = repository.get_one_exercise(&mut *tx, 1).await;
        assert!(result.is_err(), "Exercise should have been deleted");

        tx.commit().await.unwrap();
    }
}

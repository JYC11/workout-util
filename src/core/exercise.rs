use crate::core::enums::{
    CompoundOrIsolation, DynamicOrStatic, Grip, GripWidth, LeverVariation, PushOrPull,
    SquatOrHinge, StraightOrBentArm, UpperOrLower,
};
use sqlx::{FromRow, Sqlite, Transaction};

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct ExerciseLibraryEntity {
    pub id: u32,
    pub name: String,
    pub push_or_pull: Option<PushOrPull>,
    pub dynamic_or_static: DynamicOrStatic,
    pub straight_or_bent: Option<StraightOrBentArm>,
    pub squat_or_hinge: Option<SquatOrHinge>,
    pub upper_or_lower: UpperOrLower,
    pub compound_or_isolation: CompoundOrIsolation,
    pub lever_variation: Option<LeverVariation>,
    pub grip: Option<Grip>,
    pub grip_width: Option<GripWidth>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExerciseLibraryReq {
    pub name: String,
    pub push_or_pull: Option<PushOrPull>,
    pub dynamic_or_static: DynamicOrStatic,
    pub straight_or_bent: Option<StraightOrBentArm>,
    pub squat_or_hinge: Option<SquatOrHinge>,
    pub upper_or_lower: UpperOrLower,
    pub compound_or_isolation: CompoundOrIsolation,
    pub lever_variation: Option<LeverVariation>,
    pub grip: Option<Grip>,
    pub grip_width: Option<GripWidth>,
    pub description: Option<String>,
}

// row is mapped to these valid structs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpperBodyCompoundExercise {
    pub id: u32,
    pub name: String,
    pub push_or_pull: PushOrPull,
    pub dynamic_or_static: DynamicOrStatic,
    pub straight_or_bent: StraightOrBentArm,
    pub lever_variation: Option<LeverVariation>,
    pub grip: Grip,
    pub grip_width: GripWidth,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpperBodyIsolationExercise {
    pub id: u32,
    pub name: String,
    pub dynamic_or_static: DynamicOrStatic,
    pub straight_or_bent: StraightOrBentArm,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LowerBodyCompoundExercise {
    pub id: u32,
    pub name: String,
    pub dynamic_or_static: DynamicOrStatic,
    pub squat_or_hinge: SquatOrHinge,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LowerBodyIsolationExercise {
    pub id: u32,
    pub name: String,
    pub dynamic_or_static: DynamicOrStatic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidExercise {
    UpperBodyCompound(UpperBodyCompoundExercise),
    UpperBodyIsolation(UpperBodyIsolationExercise),
    LowerBodyCompound(LowerBodyCompoundExercise),
    LowerBodyIsolation(LowerBodyIsolationExercise),
}

impl ExerciseLibraryEntity {
    pub fn from_req(req: ExerciseLibraryReq) -> Result<ExerciseLibraryEntity, String> {
        let entity = ExerciseLibraryEntity {
            id: 0, // let db generate id
            name: req.name,
            push_or_pull: req.push_or_pull,
            dynamic_or_static: req.dynamic_or_static,
            straight_or_bent: req.straight_or_bent,
            squat_or_hinge: req.squat_or_hinge,
            upper_or_lower: req.upper_or_lower,
            compound_or_isolation: req.compound_or_isolation,
            lever_variation: req.lever_variation,
            grip: req.grip,
            grip_width: req.grip_width,
            description: req.description,
        };
        entity.to_valid_struct()?; // validate
        Ok(entity)
    }

    pub fn to_valid_struct(&self) -> Result<ValidExercise, String> {
        match self.upper_or_lower {
            UpperOrLower::Upper => match self.compound_or_isolation {
                CompoundOrIsolation::Compound => {
                    let push_or_pull = self.push_or_pull.ok_or(
                        "Upper body compound exercises require a push/pull designation".to_string(),
                    )?;
                    let straight_or_bent = self.straight_or_bent.ok_or(
                        "Upper body compound exercises require a straight/bent arm designation"
                            .to_string(),
                    )?;

                    match straight_or_bent {
                        StraightOrBentArm::Straight => {
                            self.lever_variation.ok_or(
                               "Upper body compound exercises require a lever variation designation"
                           )?;
                        }
                        _ => {}
                    }

                    let grip = self.grip.ok_or(
                        "Upper body compound exercises require a grip designation".to_string(),
                    )?;

                    let grip_width = self.grip_width.ok_or(
                        "Upper body compound exercises require a grip width designation"
                            .to_string(),
                    )?;

                    Ok(ValidExercise::UpperBodyCompound(
                        UpperBodyCompoundExercise {
                            id: self.id,
                            name: self.name.clone(),
                            push_or_pull,
                            dynamic_or_static: self.dynamic_or_static,
                            straight_or_bent,
                            lever_variation: self.lever_variation,
                            grip,
                            grip_width,
                        },
                    ))
                }
                CompoundOrIsolation::Isolation => {
                    let straight_or_bent = self.straight_or_bent.ok_or(
                        "Upper body isolation exercises require a straight/bent arm designation"
                            .to_string(),
                    )?;

                    Ok(ValidExercise::UpperBodyIsolation(
                        UpperBodyIsolationExercise {
                            id: self.id,
                            name: self.name.clone(),
                            dynamic_or_static: self.dynamic_or_static,
                            straight_or_bent,
                        },
                    ))
                }
            },
            UpperOrLower::Lower => match self.compound_or_isolation {
                CompoundOrIsolation::Compound => {
                    let squat_or_hinge = self.squat_or_hinge.ok_or(
                        "Lower body compound exercises require a squat/hinge designation"
                            .to_string(),
                    )?;

                    Ok(ValidExercise::LowerBodyCompound(
                        LowerBodyCompoundExercise {
                            id: self.id,
                            name: self.name.clone(),
                            dynamic_or_static: self.dynamic_or_static,
                            squat_or_hinge,
                        },
                    ))
                }
                CompoundOrIsolation::Isolation => Ok(ValidExercise::LowerBodyIsolation(
                    LowerBodyIsolationExercise {
                        id: self.id,
                        name: self.name.clone(),
                        dynamic_or_static: self.dynamic_or_static,
                    },
                )),
            },
        }
    }
}

pub async fn create_exercise(
    tx: &mut Transaction<'_, Sqlite>,
    req: ExerciseLibraryReq,
) -> Result<(), String> {
    let entity = ExerciseLibraryEntity::from_req(req)?;

    sqlx::query(
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

    Ok(())
}

pub async fn update_exercise(
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
        ValidExercise::UpperBodyCompound(e) => (
            e.id,
            e.name,
            Some(e.push_or_pull),
            e.dynamic_or_static,
            Some(e.straight_or_bent),
            None::<SquatOrHinge>,
            UpperOrLower::Upper,
            CompoundOrIsolation::Compound,
            e.lever_variation,
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

pub async fn get_one_exercise(
    tx: &mut Transaction<'_, Sqlite>,
    exercise_id: u32,
) -> Result<ValidExercise, String> {
    let row: ExerciseLibraryEntity = sqlx::query_as("SELECT * FROM exercise_library WHERE id = ?")
        .bind(exercise_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Exercise not found".to_string())?;

    row.to_valid_struct()
}

pub fn paginate(tx: &mut Transaction<'_, Sqlite>) -> Result<(), String> {
    // TODO add filtering and paging
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::enums::*;
    use crate::db::{IN_MEMORY_DB_URL, init_db};
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

        let req = mock_upper_compound_req("Bench Press");

        // 1. Test Create
        create_exercise(&mut tx, req)
            .await
            .expect("Failed to create exercise");

        // 2. Test Get One (ID should be 1 for first entry)
        let result = get_one_exercise(&mut tx, 1)
            .await
            .expect("Failed to get exercise");

        match result {
            ValidExercise::UpperBodyCompound(e) => {
                assert_eq!(e.id, 1);
                assert_eq!(e.name, "Bench Press");
                assert_eq!(e.push_or_pull, PushOrPull::Push);
            }
            _ => panic!("Exercise type mismatch, expected UpperBodyCompound"),
        }

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_update_exercise() {
        let pool = setup_db().await;
        let mut tx = pool.begin().await.unwrap();

        // Setup: Create initial exercise
        let req = mock_upper_compound_req("Old Name");
        create_exercise(&mut tx, req).await.unwrap();

        // Get the valid struct
        let mut exercise = get_one_exercise(&mut tx, 1).await.unwrap();

        // Modify it
        if let ValidExercise::UpperBodyCompound(ref mut e) = exercise {
            e.name = "New Name".to_string();
            // Change a property to ensure update logic works
            e.grip_width = GripWidth::Wide;
        }

        // 3. Test Update
        update_exercise(&mut tx, exercise)
            .await
            .expect("Failed to update");

        // Verify changes
        let updated = get_one_exercise(&mut tx, 1).await.unwrap();
        if let ValidExercise::UpperBodyCompound(e) = updated {
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

        // Setup: Create
        let req = mock_upper_compound_req("To Delete");
        create_exercise(&mut tx, req).await.unwrap();

        // Ensure it exists
        assert!(get_one_exercise(&mut tx, 1).await.is_ok());

        // 4. Test Delete
        delete_exercise(&mut tx, 1).await.expect("Failed to delete");

        // Verify it is gone
        let result = get_one_exercise(&mut tx, 1).await;
        assert!(result.is_err(), "Exercise should have been deleted");

        tx.commit().await.unwrap();
    }

    #[test]
    fn test_to_valid_struct_upper_body_dynamic_compound() {
        let entity = ExerciseLibraryEntity {
            id: 1,
            name: "Bench Press".to_string(),
            push_or_pull: Some(PushOrPull::Push),
            dynamic_or_static: DynamicOrStatic::Dynamic,
            straight_or_bent: Some(StraightOrBentArm::Bent),
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Upper,
            compound_or_isolation: CompoundOrIsolation::Compound,
            lever_variation: None,
            grip: Some(Grip::Pronated),
            grip_width: Some(GripWidth::Shoulder),
            description: None,
        };

        // 1. Success case
        assert!(entity.to_valid_struct().is_ok());

        // 2. Missing Push/Pull
        let mut invalid = entity.clone();
        invalid.push_or_pull = None;
        assert_eq!(
            invalid.to_valid_struct().err().unwrap(),
            "Upper body compound exercises require a push/pull designation"
        );

        // 3. Missing Straight/Bent
        let mut invalid = entity.clone();
        invalid.straight_or_bent = None;
        assert_eq!(
            invalid.to_valid_struct().err().unwrap(),
            "Upper body compound exercises require a straight/bent arm designation"
        );

        // 4. Missing Grip
        let mut invalid = entity.clone();
        invalid.grip = None;
        assert_eq!(
            invalid.to_valid_struct().err().unwrap(),
            "Upper body compound exercises require a grip designation"
        );

        // 5. Missing Grip Width
        let mut invalid = entity.clone();
        invalid.grip_width = None;
        assert_eq!(
            invalid.to_valid_struct().err().unwrap(),
            "Upper body compound exercises require a grip width designation"
        );
    }

    #[test]
    fn test_to_valid_struct_upper_body_static_compound() {
        let entity = ExerciseLibraryEntity {
            id: 1,
            name: "Planche".to_string(),
            push_or_pull: Some(PushOrPull::Push),
            dynamic_or_static: DynamicOrStatic::Static,
            straight_or_bent: Some(StraightOrBentArm::Straight),
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Upper,
            compound_or_isolation: CompoundOrIsolation::Compound,
            lever_variation: Some(LeverVariation::Full),
            grip: Some(Grip::Pronated),
            grip_width: Some(GripWidth::Shoulder),
            description: None,
        };

        // 1. Success case
        assert!(entity.to_valid_struct().is_ok());

        // 2. Missing Lever Variation
        let mut invalid = entity.clone();
        invalid.lever_variation = None;
        assert_eq!(
            invalid.to_valid_struct().err().unwrap(),
            "Upper body compound exercises require a lever variation designation"
        );
    }

    #[test]
    fn test_to_valid_struct_upper_body_isolation() {
        let entity = ExerciseLibraryEntity {
            id: 2,
            name: "Bicep Curl".to_string(),
            push_or_pull: None,
            dynamic_or_static: DynamicOrStatic::Dynamic,
            straight_or_bent: Some(StraightOrBentArm::Bent),
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Upper,
            compound_or_isolation: CompoundOrIsolation::Isolation,
            lever_variation: None,
            grip: None,
            grip_width: None,
            description: None,
        };

        // 1. Success case
        assert!(entity.to_valid_struct().is_ok());

        // 2. Missing Straight/Bent
        let mut invalid = entity.clone();
        invalid.straight_or_bent = None;
        assert_eq!(
            invalid.to_valid_struct().err().unwrap(),
            "Upper body isolation exercises require a straight/bent arm designation"
        );
    }

    #[test]
    fn test_to_valid_struct_lower_body_compound() {
        let entity = ExerciseLibraryEntity {
            id: 3,
            name: "Squat".to_string(),
            push_or_pull: None,
            dynamic_or_static: DynamicOrStatic::Dynamic,
            straight_or_bent: None,
            squat_or_hinge: Some(SquatOrHinge::Squat),
            upper_or_lower: UpperOrLower::Lower,
            compound_or_isolation: CompoundOrIsolation::Compound,
            lever_variation: None,
            grip: None,
            grip_width: None,
            description: None,
        };

        // 1. Success case
        assert!(entity.to_valid_struct().is_ok());

        // 2. Missing Squat/Hinge
        let mut invalid = entity.clone();
        invalid.squat_or_hinge = None;
        assert_eq!(
            invalid.to_valid_struct().err().unwrap(),
            "Lower body compound exercises require a squat/hinge designation"
        );
    }

    #[test]
    fn test_to_valid_struct_lower_body_isolation() {
        let entity = ExerciseLibraryEntity {
            id: 4,
            name: "Leg Extension".to_string(),
            push_or_pull: None,
            dynamic_or_static: DynamicOrStatic::Dynamic,
            straight_or_bent: None,
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Lower,
            compound_or_isolation: CompoundOrIsolation::Isolation,
            lever_variation: None,
            grip: None,
            grip_width: None,
            description: None,
        };

        // 1. Success case
        assert!(entity.to_valid_struct().is_ok());
    }
}

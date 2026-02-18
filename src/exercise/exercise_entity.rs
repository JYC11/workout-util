use crate::enums::{
    CompoundOrIsolation, DynamicOrStatic, Grip, GripWidth, LeverVariation, PushOrPull,
    SquatOrHinge, StraightOrBentArm, UpperOrLower,
};
use crate::exercise::exercise_dto::{
    BentArmCompoundExercise, ExerciseLibraryReq, LowerBodyCompoundExercise,
    LowerBodyIsolationExercise, StraightArmCompoundExercise, UpperBodyIsolationExercise,
    ValidExercise,
};
use sqlx::{Executor, FromRow};

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

    fn validate_invariants(&self) -> Result<(), String> {
        // Reject contradictory fields
        if self.name.is_empty() {
            return Err("Exercise name cannot be empty".into());
        }
        if self.upper_or_lower == UpperOrLower::Lower && self.push_or_pull.is_some() {
            return Err("Lower body exercises cannot have push/pull designation".into());
        }
        if self.upper_or_lower == UpperOrLower::Upper && self.squat_or_hinge.is_some() {
            return Err("Upper body exercises cannot have squat/hinge designation".into());
        }
        if self.compound_or_isolation == CompoundOrIsolation::Isolation {
            if self.push_or_pull.is_some()
                || self.lever_variation.is_some()
                || self.grip.is_some()
                || self.grip_width.is_some()
            {
                return Err("Isolation exercises cannot have compound-specific attributes".into());
            }
        }
        Ok(())
    }

    pub fn to_valid_struct(&self) -> Result<ValidExercise, String> {
        self.validate_invariants()?;

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
                    let grip = self.grip.ok_or(
                        "Upper body compound exercises require a grip designation".to_string(),
                    )?;
                    let grip_width = self.grip_width.ok_or(
                        "Upper body compound exercises require a grip width designation"
                            .to_string(),
                    )?;

                    match straight_or_bent {
                        StraightOrBentArm::Straight => {
                            let lever_variation = self.lever_variation.ok_or(
                                "Straight-arm compound exercises require a lever variation"
                                    .to_string(),
                            )?;

                            Ok(ValidExercise::StraightArmCompound(
                                StraightArmCompoundExercise {
                                    id: self.id,
                                    name: self.name.clone(),
                                    push_or_pull,
                                    dynamic_or_static: self.dynamic_or_static,
                                    lever_variation,
                                    grip,
                                    grip_width,
                                },
                            ))
                        }
                        StraightOrBentArm::Bent => {
                            Ok(ValidExercise::BentArmCompound(BentArmCompoundExercise {
                                id: self.id,
                                name: self.name.clone(),
                                push_or_pull,
                                dynamic_or_static: self.dynamic_or_static,
                                lever_variation: self.lever_variation, // Optional
                                grip,
                                grip_width,
                            }))
                        }
                    }
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

#[cfg(test)]
mod test {
    use crate::enums::{
        CompoundOrIsolation, DynamicOrStatic, Grip, GripWidth, LeverVariation, PushOrPull,
        SquatOrHinge, StraightOrBentArm, UpperOrLower,
    };
    use crate::exercise::exercise_entity::ExerciseLibraryEntity;

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
            "Straight-arm compound exercises require a lever variation"
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

use crate::context::AppContext;
use crate::core::enums::{
    CompoundOrIsolation, DynamicOrStatic, Grip, GripWidth, LeverVariation, PushOrPull,
    SquatOrHinge, StraightOrBentArm, UpperOrLower,
};
use sqlx::{Sqlite, Transaction};

// mapped to a db row
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExerciseLibraryEntryEntity {
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

pub struct ExerciseLibraryEntryReq {
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

impl ExerciseLibraryEntryEntity {
    pub fn from_req(req: ExerciseLibraryEntryReq) -> Result<ExerciseLibraryEntryEntity, String> {
        let entity = ExerciseLibraryEntryEntity {
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

pub fn create(tx: &Transaction<Sqlite>, req: ExerciseLibraryEntryReq) -> Result<(), String> {
    // TODO insert statement
    Ok(())
}

pub fn update(tx: &Transaction<Sqlite>, valid_exercise: ValidExercise) -> Result<(), String> {
    // TODO update statement
    Ok(())
}

pub fn delete(tx: &Transaction<Sqlite>, exercise_id: u32) -> Result<(), String> {
    // TODO delete statement
    // prevent deleting exercises that are in use by workouts
    Ok(())
}

pub fn get_one(tx: &Transaction<Sqlite>, exercise_id: u32) -> Result<(), String> {
    // TODO select statement and map to ValidExercise
    Ok(())
}

pub fn paginate(tx: &Transaction<Sqlite>) -> Result<(), String> {
    // TODO add filtering and paging
    Ok(())
}

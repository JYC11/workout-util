use crate::db::pagination_support::HasId;
use crate::workout::enums::{
    CompoundOrIsolation, DynamicOrStatic, Grip, GripWidth, LeverVariation, PushOrPull,
    SquatOrHinge, StraightOrBentArm, UpperOrLower,
};
use sqlx::FromRow;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct ExerciseLibraryRes {
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

impl HasId for ExerciseLibraryRes {
    fn id(&self) -> u32 {
        self.id
    }
}

pub struct ExerciseLibraryFilterReq {
    pub name: Option<String>,
    pub push_or_pull: Option<Vec<PushOrPull>>,
    pub dynamic_or_static: Option<Vec<DynamicOrStatic>>,
    pub straight_or_bent: Option<Vec<StraightOrBentArm>>,
    pub squat_or_hinge: Option<Vec<SquatOrHinge>>,
    pub upper_or_lower: Option<Vec<UpperOrLower>>,
    pub compound_or_isolation: Option<Vec<CompoundOrIsolation>>,
    pub lever_variation: Option<Vec<LeverVariation>>,
    pub grip: Option<Vec<Grip>>,
    pub grip_width: Option<Vec<GripWidth>>,
}

impl Default for ExerciseLibraryFilterReq {
    fn default() -> Self {
        Self {
            name: None,
            push_or_pull: None,
            dynamic_or_static: None,
            straight_or_bent: None,
            squat_or_hinge: None,
            upper_or_lower: None,
            compound_or_isolation: None,
            lever_variation: None,
            grip: None,
            grip_width: None,
        }
    }
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
pub struct StraightArmCompoundExercise {
    pub id: u32,
    pub name: String,
    pub push_or_pull: PushOrPull,
    pub dynamic_or_static: DynamicOrStatic,
    pub lever_variation: LeverVariation, // NON-OPTIONAL
    pub grip: Grip,
    pub grip_width: GripWidth,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BentArmCompoundExercise {
    pub id: u32,
    pub name: String,
    pub push_or_pull: PushOrPull,
    pub dynamic_or_static: DynamicOrStatic,
    pub lever_variation: Option<LeverVariation>, // Optional for bent-arm
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
    StraightArmCompound(StraightArmCompoundExercise),
    BentArmCompound(BentArmCompoundExercise),
    UpperBodyIsolation(UpperBodyIsolationExercise),
    LowerBodyCompound(LowerBodyCompoundExercise),
    LowerBodyIsolation(LowerBodyIsolationExercise),
}

pub fn get_exercise_id(v: &ValidExercise) -> u32 {
    match v {
        ValidExercise::StraightArmCompound(x) => x.id,
        ValidExercise::BentArmCompound(x) => x.id,
        ValidExercise::UpperBodyIsolation(x) => x.id,
        ValidExercise::LowerBodyCompound(x) => x.id,
        ValidExercise::LowerBodyIsolation(x) => x.id,
    }
}

pub fn get_exercise_name(v: &ValidExercise) -> &String {
    match v {
        ValidExercise::StraightArmCompound(x) => &x.name,
        ValidExercise::BentArmCompound(x) => &x.name,
        ValidExercise::UpperBodyIsolation(x) => &x.name,
        ValidExercise::LowerBodyCompound(x) => &x.name,
        ValidExercise::LowerBodyIsolation(x) => &x.name,
    }
}

pub fn exercise_to_req(e: &ValidExercise) -> ExerciseLibraryReq {
    match e {
        ValidExercise::StraightArmCompound(x) => ExerciseLibraryReq {
            name: x.name.clone(),
            push_or_pull: Some(x.push_or_pull),
            dynamic_or_static: x.dynamic_or_static,
            straight_or_bent: Some(StraightOrBentArm::Straight),
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Upper,
            compound_or_isolation: CompoundOrIsolation::Compound,
            lever_variation: Some(x.lever_variation),
            grip: Some(x.grip),
            grip_width: Some(x.grip_width),
            description: None,
        },
        ValidExercise::BentArmCompound(x) => ExerciseLibraryReq {
            name: x.name.clone(),
            push_or_pull: Some(x.push_or_pull),
            dynamic_or_static: x.dynamic_or_static,
            straight_or_bent: Some(StraightOrBentArm::Bent),
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Upper,
            compound_or_isolation: CompoundOrIsolation::Compound,
            lever_variation: x.lever_variation,
            grip: Some(x.grip),
            grip_width: Some(x.grip_width),
            description: None,
        },
        ValidExercise::UpperBodyIsolation(x) => ExerciseLibraryReq {
            name: x.name.clone(),
            push_or_pull: None,
            dynamic_or_static: x.dynamic_or_static,
            straight_or_bent: Some(x.straight_or_bent),
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Upper,
            compound_or_isolation: CompoundOrIsolation::Isolation,
            lever_variation: None,
            grip: None,
            grip_width: None,
            description: None,
        },
        ValidExercise::LowerBodyCompound(x) => ExerciseLibraryReq {
            name: x.name.clone(),
            push_or_pull: None,
            dynamic_or_static: x.dynamic_or_static,
            straight_or_bent: None,
            squat_or_hinge: Some(x.squat_or_hinge),
            upper_or_lower: UpperOrLower::Lower,
            compound_or_isolation: CompoundOrIsolation::Compound,
            lever_variation: None,
            grip: None,
            grip_width: None,
            description: None,
        },
        ValidExercise::LowerBodyIsolation(x) => ExerciseLibraryReq {
            name: x.name.clone(),
            push_or_pull: None,
            dynamic_or_static: x.dynamic_or_static,
            straight_or_bent: None,
            squat_or_hinge: None,
            upper_or_lower: UpperOrLower::Lower,
            compound_or_isolation: CompoundOrIsolation::Isolation,
            lever_variation: None,
            grip: None,
            grip_width: None,
            description: None,
        },
    }
}

pub fn exercise_library_default_req() -> ExerciseLibraryReq {
    ExerciseLibraryReq {
        name: "".to_string(),
        push_or_pull: None,
        dynamic_or_static: DynamicOrStatic::Dynamic,
        straight_or_bent: None,
        squat_or_hinge: None,
        upper_or_lower: UpperOrLower::Upper,
        compound_or_isolation: CompoundOrIsolation::Compound,
        lever_variation: None,
        grip: None,
        grip_width: None,
        description: None,
    }
}

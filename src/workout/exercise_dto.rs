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

// data models
pub enum PushOrPull {
    Push,
    Pull,
}

pub enum DynamicOrStatic {
    Dynamic,
    Static,
}

pub enum StraightOrBentArm {
    Straight,
    Bent,
}

pub enum SquatOrHinge {
    Squat,
    Hinge,
}

pub enum UpperOrLower {
    Upper,
    Lowerer,
}

pub enum CompoundOrIsolation {
    Compound,
    Isolation,
}

// mapped to db row
pub struct ExerciseLibraryEntryEntity {
    pub id: u32,
    pub name: String,
    pub push_or_pull: Option<PushOrPull>,
    pub dynamic_or_static: DynamicOrStatic,
    pub straight_or_bent: Option<StraightOrBentArm>,
    pub squat_or_hinge: Option<SquatOrHinge>,
    pub uppers_or_lower: UpperOrLower,
    pub compound_or_isolation: CompoundOrIsolation,
}

// row is mapped to these valid structs
pub struct UpperBodyCompoundExercise {
    pub id: u32,
    pub name: String,
    pub push_or_pull: PushOrPull,
    pub dynamic_or_static: DynamicOrStatic,
    pub straight_or_bent: StraightOrBentArm,
}

pub struct UpperBodyIsolationExercise {
    pub id: u32,
    pub name: String,
    pub dynamic_or_static: DynamicOrStatic,
    pub straight_or_bent: StraightOrBentArm,
}

pub struct LowerBodyCompoundExercise {
    pub id: u32,
    pub name: String,
    pub dynamic_or_static: DynamicOrStatic,
    pub squat_or_hinge: SquatOrHinge,
}

pub struct LowerBodyIsolationExercise {
    pub id: u32,
    pub name: String,
    pub dynamic_or_static: DynamicOrStatic,
}
pub enum LeverVariation {
    Tuck,
    AdvancedTuck,
    Straddle,
    OneLeg,
    HalfLay,
    Full,
}

pub enum Grip {
    Pronated,
    Supinated,
    Neutral,
    GymnasticsRing,
    Floor,
}

pub enum GripWidth {
    Wide,
    Neutral,
    Narrow,
}

pub enum Equipment {
    LowParallettes,
    HighParallettes,
    Bench,
    Dumbbells,
    Barbell,
    SmithMachine,
    GymnasticsRings,
    PullUpBar,
    DipBar,
}

pub enum Band {
    Yellow,
    Red,
    Black,
    Purple,
    Green,
}

pub struct WorkoutExerciseEntity {
    pub id: u32,
    pub created_at: String, // should be DateTime
    pub workout_id: u32,    // fk to Workout
    pub exercise_id: u32,   // fk to ExerciseLibraryEntry
    pub code: String,       // A1, A2, B1, B2 ...
    pub sets: u8,
    pub reps_or_seconds: u8,
    pub weight: u8,
    pub rest_period_seconds: u8,
    pub tempo: String,
    pub lever_variation: Option<LeverVariation>,
    pub grip: Option<Grip>,
    pub grip_width: Option<GripWidth>,
    pub equipments: String, // Vec<Equipment>,
    pub bands: String,      // Vec<Band>,
}

pub struct WorkoutEntity {
    pub id: u32,
}

pub struct WorkoutLogEntity {
    pub id: u32,
    pub created_at: String,
    pub date: String,
    pub workout_id: u32,          // fk to Workout
    pub workout_exercise_id: u32, // fk to WorkoutExercise
    pub set_number: u32,
    pub rep_number_or_seconds: u32,
    pub weight: u32,
}

fn main() {
    println!("Hello, world!");
}

// main app
pub struct WorkoutUtil {}

// timer
pub struct Metronome {
    bpm: u32, // should be fixed to 60
}
pub struct RestTimer {
    seconds: u32,
}
pub struct EMOMTimer {
    seconds: u32,
    rounds: u32,
    rest_period: u32,
}

// data models
pub struct ExerciseLibraryEntry {
    id: u32,
    name: String,
    lever_variation: Option<LeverVariation>,
    grip: Option<Grip>,
    grip_width: Option<GripWidth>,
    equipment: Vec<Equipment>,
    bands: Vec<Band>,
}

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

pub struct WorkoutExercise {
    id: u32,
    created_at: String, // should be DateTime
    workout_id: u32, // fk to Workout
    exercise_id: u32, // fk to ExerciseLibraryEntry
    code: String, // A1, A2, B1, B2 ...
    sets: u32,
    reps_or_seconds: u32,
    weight: u32,
    rest_period: u32,
}

pub struct Workout {
    id: u32,
    exercises: Vec<WorkoutExercise>,
}

pub struct WorkoutLog {
    id: u32,
    created_at: String,
    date: String,
    workout_id: u32, // fk to Workout
    workout_exercise_id: u32, // fk to WorkoutExercise
    set_number: u32,
    rep_number_or_seconds: u32,
    weight: u32,
}


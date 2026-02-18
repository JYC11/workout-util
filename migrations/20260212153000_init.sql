CREATE TABLE IF NOT EXISTS exercise_library (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    push_or_pull TEXT,                -- Enum: Push, Pull (Nullable)
    dynamic_or_static TEXT NOT NULL,  -- Enum: Dynamic, Static
    straight_or_bent TEXT,            -- Enum: Straight, Bent (Nullable)
    squat_or_hinge TEXT,              -- Enum: Squat, Hinge (Nullable)
    upper_or_lower TEXT NOT NULL,     -- Enum: Upper, Lower
    compound_or_isolation TEXT NOT NULL, -- Enum: Compound, Isolation
    lever_variation TEXT,             -- Enum (Nullable)
    grip TEXT,                        -- Enum (Nullable)
    grip_width TEXT,                  -- Enum (Nullable)
    description TEXT
);

CREATE TABLE IF NOT EXISTS workouts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL,         -- DateTime stored as String
    name TEXT NOT NULL,
    description TEXT,
    active BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS workout_exercises (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL,         -- DateTime stored as String
    workout_id INTEGER NOT NULL,
    code TEXT NOT NULL,               -- e.g., "A1", "B2"
    name TEXT NOT NULL,
    sets_target INTEGER NOT NULL,
    reps_or_seconds_target INTEGER NOT NULL,
    working_weight INTEGER NOT NULL,
    rest_period_seconds INTEGER NOT NULL,
    tempo TEXT NOT NULL,
    emom  BOOLEAN NOT NULL,
    equipments TEXT NOT NULL,         -- Serialized List/JSON
    bands TEXT NOT NULL,              -- Serialized List/JSON
    description TEXT,
    FOREIGN KEY (workout_id) REFERENCES workouts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS workout_log_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL,
    date TEXT NOT NULL,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS workout_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workout_id INTEGER NOT NULL,
    workout_exercise_id INTEGER NOT NULL,
    workout_log_group_id INTEGER NOT NULL,
    set_number INTEGER NOT NULL,
    rep_number_or_seconds INTEGER NOT NULL,
    weight INTEGER NOT NULL,
    description TEXT,
    FOREIGN KEY (workout_id) REFERENCES workouts(id) ON DELETE RESTRICT,
    FOREIGN KEY (workout_exercise_id) REFERENCES workout_exercises(id) ON DELETE RESTRICT,
    FOREIGN KEY (workout_log_group_id) REFERENCES workout_log_groups(id) ON DELETE RESTRICT
);
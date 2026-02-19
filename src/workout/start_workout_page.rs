use crate::client::app::PageAction;
use crate::workout::workout_dto::{WorkoutExerciseRes, WorkoutRes};
use crate::workout::workout_service::WorkoutService;
use crate::workout_log::workout_log_service::WorkoutLogService;
use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct StartWorkoutPage {
    current_workout_id: Option<u32>,
    current_workout: Option<WorkoutRes>,
    current_exercises: Vec<WorkoutExerciseRes>,
    workout_service: WorkoutService,
    workout_log_service: WorkoutLogService,
}

impl StartWorkoutPage {}

impl StartWorkoutPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Self {
            current_workout_id: None,
            current_workout: None,
            current_exercises: vec![],
            workout_service: WorkoutService::new(pool.clone()),
            workout_log_service: WorkoutLogService::new(pool.clone()),
        }
    }
}

impl StartWorkoutPage {
    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> PageAction {
        ui.heading("Start Workout");
        if let Some(workout_id) = self.current_workout_id {
            ui.label(format!("Workout ID: {}", workout_id));
        } else {
            ui.label("No workout selected, choose from the Workouts page.");
        }
        PageAction::None
    }

    pub fn load_workout(&mut self, workout_id: u32) {
        self.current_workout_id = Some(workout_id);
    }
}

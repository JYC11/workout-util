use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct StartWorkoutPage {
    pool: Pool<Sqlite>,
    current_workout_id: Option<u32>,
}

impl StartWorkoutPage {}

impl StartWorkoutPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            current_workout_id: None,
        }
    }
}

impl StartWorkoutPage {
    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Start Workout");
        if let Some(workout_id) = self.current_workout_id {
            ui.label(format!("Workout ID: {}", workout_id));
        } else {
            ui.label("No workout selected, choose from the Workouts page.");
        }
    }

    pub fn load_workout(&mut self, workout_id: u32) {
        self.current_workout_id = Some(workout_id);
    }
}

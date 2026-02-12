use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct WorkoutPlansPage {}

impl Default for WorkoutPlansPage {
    fn default() -> Self {
        Self {}
    }
}

impl WorkoutPlansPage {
    pub fn render_page(&mut self, pool: &mut Pool<Sqlite>, ui: &mut egui::Ui) {
        todo!("render workouts page");
    }
}

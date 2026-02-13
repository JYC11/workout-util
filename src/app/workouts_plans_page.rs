use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct WorkoutPlansPage {
    pool: Pool<Sqlite>,
}

impl WorkoutPlansPage {
    pub(crate) fn default(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl WorkoutPlansPage {
    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        todo!("render workouts page");
    }
}

use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct WorkoutLogsPage {
    pool: Pool<Sqlite>,
}

impl WorkoutLogsPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl WorkoutLogsPage {
    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        todo!("render start workout page");
    }
}

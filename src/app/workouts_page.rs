use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct WorkoutsPage {
    pub pool: Pool<Sqlite>,
}

impl WorkoutsPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl WorkoutsPage {
    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        todo!("render workouts page");
    }
}

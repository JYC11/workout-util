use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct StartWorkoutPage {
    pool: Pool<Sqlite>,
}

impl StartWorkoutPage {}

impl StartWorkoutPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl StartWorkoutPage {
    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, workout_id: Option<u32>) {
        todo!("render start workout page");
    }
}

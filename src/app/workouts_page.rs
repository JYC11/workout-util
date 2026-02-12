use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct WorkoutsPage {}

impl Default for WorkoutsPage {
    fn default() -> Self {
        Self {}
    }
}

impl WorkoutsPage {
    pub fn render_page(&mut self, pool: &mut Pool<Sqlite>, ui: &mut egui::Ui) {
        todo!("render workouts page");
    }
}

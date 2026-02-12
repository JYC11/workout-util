use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct StartWorkoutPage {}

impl Default for StartWorkoutPage {
    fn default() -> Self {
        Self {}
    }
}

impl StartWorkoutPage {
    pub fn render_page(&mut self, pool: &mut Pool<Sqlite>, ui: &mut egui::Ui) {
        todo!("render start workout page");
    }
}

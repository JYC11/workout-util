use crate::client::app::WorkoutUtil;
use crate::db::{init_db, DEFAULT_DB_URL};
use eframe::egui;
use std::error::Error;

pub mod app;
pub mod app_utils;

pub async fn run_app() -> Result<(), Box<dyn Error>> {
    let pool = init_db(DEFAULT_DB_URL).await;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Workout Util",
        native_options,
        Box::new(|cc| Ok(Box::new(WorkoutUtil::new(cc, pool)))),
    )?;
    Ok(())
}

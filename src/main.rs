use crate::db::{DEFAULT_DB_URL, init_db};
use app::WorkoutUtil;
use eframe::egui;

mod app;
mod config;
mod db;
mod timer;
mod workout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

use crate::db::{DEFAULT_DB_URL, init_db};
use eframe::egui;
use sqlx::{Pool, Sqlite};

mod config;
mod db;
mod timer;
mod workout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the database asynchronously
    let pool = init_db(DEFAULT_DB_URL).await;

    // 2. Setup eframe options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // 3. Run the app, passing the pool in the creation closure
    eframe::run_native(
        "Workout Util",
        native_options,
        Box::new(|cc| Ok(Box::new(WorkoutUtil::new(cc, pool)))),
    )?;

    Ok(())
}

// main app
pub struct WorkoutUtil {
    pool: Pool<Sqlite>,
}

impl WorkoutUtil {
    // Custom constructor that accepts the pool
    pub fn new(_cc: &eframe::CreationContext<'_>, pool: Pool<Sqlite>) -> Self {
        // You can also customize fonts or styles here using _cc.egui_ctx
        Self { pool }
    }
}

// Implement eframe::App instead of Default
impl eframe::App for WorkoutUtil {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Workout Util");
            ui.label("Database is connected.");
        });
    }
}

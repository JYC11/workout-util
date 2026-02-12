use crate::db::{DEFAULT_DB_URL, init_db};
use eframe::egui;
use sqlx::{Pool, Sqlite};

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

// main app
pub struct WorkoutUtil {
    pool: Pool<Sqlite>,
}

impl WorkoutUtil {
    pub fn new(_cc: &eframe::CreationContext<'_>, pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl eframe::App for WorkoutUtil {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .default_height(60.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Workout Util");
                    ui.separator();
                    if ui.button("Home").clicked() {
                        println!("Clicked home");
                    }
                    if ui.button("Exercises").clicked() {
                        println!("Clicked exercises");
                    }
                    if ui.button("Workouts").clicked() {
                        println!("Clicked workouts");
                    }
                    if ui.button("Start Workout").clicked() {
                        println!("Clicked start workout");
                    }
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .default_height(60.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Workout Timer");
                    ui.separator();
                    ui.label("00:00:00");
                    if ui.button("Start").clicked() {
                        println!("Clicked start");
                    }
                    if ui.button("Stop").clicked() {
                        println!("Clicked stop");
                    }
                    ui.separator();
                    ui.label("Metronome");
                    if ui.button("Start").clicked() {
                        println!("Clicked start");
                    }
                    if ui.button("Stop").clicked() {
                        println!("Clicked stop");
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Workout Util");
            ui.label("Database is connected.");
        });
    }
}

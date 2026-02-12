use eframe::egui;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Home,
    Exercises,
    Workouts,
    StartWorkout,
}

pub struct WorkoutUtil {
    pool: Pool<Sqlite>,
    current_page: Page,
}

impl WorkoutUtil {
    pub fn new(_cc: &eframe::CreationContext<'_>, pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            current_page: Page::Home,
        }
    }

    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Workout Util");
            ui.separator();
            ui.add_space(10.0);

            for (page, label) in [
                (Page::Home, "Home"),
                (Page::Exercises, "Exercises"),
                (Page::Workouts, "Workouts"),
                (Page::StartWorkout, "StartWorkout"),
            ] {
                let is_active = self.current_page == page;

                let button = egui::Button::new(label)
                    .fill(if is_active {
                        ui.visuals().selection.bg_fill
                    } else {
                        egui::Color32::TRANSPARENT
                    })
                    .stroke(if is_active {
                        egui::Stroke::new(2.0, ui.visuals().selection.stroke.color)
                    } else {
                        egui::Stroke::NONE
                    });

                if ui.add(button).clicked() {
                    self.current_page = page;
                    println!("Clicked {}", label);
                }
            }
        });
    }

    fn render_page(&mut self, ui: &mut egui::Ui) {
        match self.current_page {
            Page::Home => self.render_home(ui),
            Page::Exercises => self.render_exercises(ui),
            Page::Workouts => self.render_workouts(ui),
            Page::StartWorkout => self.render_start_workout(ui),
        }
    }

    fn render_home(&mut self, ui: &mut egui::Ui) {
        ui.label("Home Page");
    }

    fn render_exercises(&mut self, ui: &mut egui::Ui) {
        ui.label("Exercises Page");
    }

    fn render_workouts(&mut self, ui: &mut egui::Ui) {
        ui.label("Workouts Page");
    }

    fn render_start_workout(&mut self, ui: &mut egui::Ui) {
        ui.label("Start Workout Page");
    }

    fn footer(&mut self, ui: &mut egui::Ui) {
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
    }
}

impl eframe::App for WorkoutUtil {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .default_height(60.0)
            .show(ctx, |ui| {
                self.header(ui);
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .default_height(60.0)
            .show(ctx, |ui| {
                self.footer(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Workout Util");
            self.render_page(ui);
        });
    }
}

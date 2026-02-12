mod exercises_page;
mod start_workout_page;
mod workouts_page;

use crate::app::exercises_page::ExercisesPage;
use crate::app::start_workout_page::StartWorkoutPage;
use crate::app::workouts_page::WorkoutsPage;
use eframe::egui;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainPageState {
    Home,
    Exercises,
    Workouts,
    StartWorkout,
}

pub struct WorkoutUtil {
    pool: Pool<Sqlite>,
    current_page: MainPageState,
    exercises_page: ExercisesPage,
    workouts_page: WorkoutsPage,
    start_workout_page: StartWorkoutPage,
}

impl WorkoutUtil {
    pub fn new(_cc: &eframe::CreationContext<'_>, pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            current_page: MainPageState::Home,
            exercises_page: ExercisesPage::default(),
            workouts_page: WorkoutsPage::default(),
            start_workout_page: StartWorkoutPage::default(),
        }
    }

    fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Workout Util");
            ui.separator();
            ui.add_space(10.0);

            for (page, label) in [
                (MainPageState::Home, "Home"),
                (MainPageState::Exercises, "Exercises"),
                (MainPageState::Workouts, "Workouts"),
                (MainPageState::StartWorkout, "Start Workout"),
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
                }
            }
        });
    }

    fn render_page(&mut self, ui: &mut egui::Ui) {
        match self.current_page {
            MainPageState::Home => self.render_home(ui),
            MainPageState::Exercises => self.render_exercises(ui),
            MainPageState::Workouts => self.render_workouts(ui),
            MainPageState::StartWorkout => self.render_start_workout(ui),
        }
    }

    fn render_home(&mut self, ui: &mut egui::Ui) {
        ui.heading("Home");
        ui.label("Welcome to Workout Util!");
    }

    fn render_exercises(&mut self, ui: &mut egui::Ui) {
        ui.heading("Exercises");
        self.exercises_page.render_page(&mut self.pool, ui);
    }

    fn render_workouts(&mut self, ui: &mut egui::Ui) {
        ui.heading("Workouts");
        self.workouts_page.render_page(&mut self.pool, ui);
    }

    fn render_start_workout(&mut self, ui: &mut egui::Ui) {
        ui.heading("Start Workout");
        self.start_workout_page.render_page(&mut self.pool, ui);
    }

    fn footer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Rest Timer");
            ui.separator();
            ui.label("00:00:00");
            // TODO add time input and make it count down and emit noise when done
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
            self.render_page(ui);
        });
    }
}

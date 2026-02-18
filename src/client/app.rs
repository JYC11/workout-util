use crate::exercise::exercises_page::ExercisesPage;
use crate::timer::Timer;
use crate::timer::countdown_timer::CountDownTimer;
use crate::timer::emom_timer::EMOMTimer;
use crate::timer::metronome::Metronome;
use crate::workout::start_workout_page::StartWorkoutPage;
use crate::workout_log::workout_logs_page::WorkoutLogsPage;
use crate::workout_log::workouts_page::WorkoutsPage;
use eframe::egui;
use eframe::egui::Ui;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainPageState {
    Home,
    Exercises,
    Workouts,
    StartWorkout,
    WorkoutLogs,
}

pub enum PageAction {
    None,
    GoToStartWorkout(u32),
}

pub struct WorkoutUtil {
    pool: Pool<Sqlite>,
    current_page: MainPageState,
    exercises_page: ExercisesPage,
    workouts_page: WorkoutsPage,
    start_workout_page: StartWorkoutPage,
    workout_logs_page: WorkoutLogsPage,
    metronome: Metronome,
    rest_timer: CountDownTimer,
    emom_timer: EMOMTimer,
}

impl WorkoutUtil {
    pub fn new(_cc: &eframe::CreationContext<'_>, pool: Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
            current_page: MainPageState::Home,
            exercises_page: ExercisesPage::default(pool.clone()),
            workouts_page: WorkoutsPage::default(pool.clone()),
            start_workout_page: StartWorkoutPage::default(pool.clone()),
            workout_logs_page: WorkoutLogsPage::default(pool.clone()),
            metronome: Metronome::new(),
            rest_timer: CountDownTimer::new(),
            emom_timer: EMOMTimer::new(),
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
                (MainPageState::WorkoutLogs, "Workout Logs"),
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

    fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let action = match self.current_page {
            MainPageState::Home => {
                self.render_home(ctx, ui);
                PageAction::None
            }
            MainPageState::Exercises => {
                self.exercises_page.render_page(ctx, ui);
                PageAction::None
            }
            MainPageState::Workouts => self.workouts_page.render_page(ctx, ui),
            MainPageState::StartWorkout => {
                self.start_workout_page.render_page(ctx, ui);
                PageAction::None
            }
            MainPageState::WorkoutLogs => {
                self.workout_logs_page.render_page(ctx, ui);
                PageAction::None
            }
        };

        match action {
            PageAction::GoToStartWorkout(workout_id) => {
                self.start_workout_page.load_workout(workout_id);
                self.current_page = MainPageState::StartWorkout;
            }
            PageAction::None => {}
        }
    }

    fn render_home(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Home");
        ui.label("Welcome to Workout Util!");
    }

    fn footer(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.metronome.tick();
        self.rest_timer.tick();
        self.emom_timer.tick();

        if self.metronome.is_running || self.rest_timer.is_running || self.emom_timer.is_running {
            ctx.request_repaint();
        }

        ui.horizontal(|ui| {
            self.render_volume_slider(ui);
            ui.separator();
            self.render_rest_timer(ui);
            ui.separator();
            self.render_metronome(ui);
            ui.separator();
            self.render_emom_timer(ui);
        });
    }

    fn render_volume_slider(&mut self, ui: &mut Ui) {
        let mut volume: f32 = 10.0;
        ui.add(egui::Slider::new(&mut volume, 1.0..=50.0).text("Volume"));
        self.metronome.volume = volume;
        self.rest_timer.volume = volume;
        self.emom_timer.work_timer.volume = volume;
        self.emom_timer.rest_timer.volume = volume;
    }

    fn render_emom_timer(&mut self, ui: &mut Ui) {
        ui.label("EMOM Timer");
        ui.label("Rounds:");
        ui.add(egui::DragValue::new(&mut self.emom_timer.rounds).range(1..=100));

        ui.label("Work:");
        ui.add(
            egui::DragValue::new(&mut self.emom_timer.work_timer.input_minutes)
                .suffix("m")
                .range(0..=60),
        );
        ui.add(
            egui::DragValue::new(&mut self.emom_timer.work_timer.input_seconds)
                .suffix("s")
                .range(0..=59),
        );

        ui.label("Rest:");
        ui.add(
            egui::DragValue::new(&mut self.emom_timer.rest_timer.input_minutes)
                .suffix("m")
                .range(0..=60),
        );
        ui.add(
            egui::DragValue::new(&mut self.emom_timer.rest_timer.input_seconds)
                .suffix("s")
                .range(0..=59),
        );

        ui.separator();

        let (min, sec) = if self.emom_timer.is_work {
            self.emom_timer.work_timer.minutes_and_seconds()
        } else {
            self.emom_timer.rest_timer.minutes_and_seconds()
        };

        let status = if !self.emom_timer.is_running {
            "Stopped"
        } else if self.emom_timer.is_work {
            "WORK"
        } else {
            "REST"
        };

        ui.label(format!(
            "{} - {:02}:{:02} (Rd {}/{})",
            status, min, sec, self.emom_timer.current_round, self.emom_timer.rounds
        ));

        if ui
            .button(if self.emom_timer.is_running {
                "Stop"
            } else {
                "Start"
            })
            .clicked()
        {
            self.emom_timer.toggle();
        }
    }

    fn render_metronome(&mut self, ui: &mut Ui) {
        ui.label("Metronome");
        if ui
            .button(if self.metronome.is_running {
                "Stop"
            } else {
                "Start"
            })
            .clicked()
        {
            self.metronome.toggle();
        }
    }

    fn render_rest_timer(&mut self, ui: &mut Ui) {
        ui.label("Rest Timer");
        ui.add(
            egui::DragValue::new(&mut self.rest_timer.input_minutes)
                .suffix("m")
                .range(0..=60),
        );
        ui.add(
            egui::DragValue::new(&mut self.rest_timer.input_seconds)
                .suffix("s")
                .range(0..=59),
        );

        let (minutes, seconds) = self.rest_timer.minutes_and_seconds();
        ui.label(format!("{:02}:{:02}", minutes, seconds));

        if ui
            .button(if self.rest_timer.is_running {
                "Stop"
            } else {
                "Start"
            })
            .clicked()
        {
            self.rest_timer.toggle();
        }
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
                self.footer(ctx, ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_page(ctx, ui);
        });
    }
}

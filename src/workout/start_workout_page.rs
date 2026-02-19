use std::sync::mpsc::{channel, Receiver, Sender};
use crate::client::app::PageAction;
use crate::workout::workout_dto::{WorkoutExerciseRes, WorkoutRes};
use crate::workout::workout_service::WorkoutService;
use crate::workout_log::workout_log_service::WorkoutLogService;
use eframe::egui;
use sqlx::{Pool, Sqlite};
use crate::client::app_utils::CommonUiState;
use crate::workout_log::workouts_page::WorkoutsPageMsg;

pub struct StartWorkoutPage {
    current_workout_id: Option<u32>,
    current_workout: Option<WorkoutRes>,
    current_exercises: Vec<WorkoutExerciseRes>,
    workout_service: WorkoutService,
    workout_log_service: WorkoutLogService,
    // Async Communication
    receiver: Receiver<WorkoutsPageMsg>,
    sender: Sender<WorkoutsPageMsg>,
    // UI Status
    common_ui_state: CommonUiState,
}

impl StartWorkoutPage {}

impl StartWorkoutPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        let (sender, receiver) = channel();
        Self {
            current_workout_id: None,
            current_workout: None,
            current_exercises: vec![],
            workout_service: WorkoutService::new(pool.clone()),
            workout_log_service: WorkoutLogService::new(pool.clone()),
            receiver,
            sender,
            common_ui_state: CommonUiState::default(),
        }
    }
}

pub enum StartWorkoutsPageMsg {
    WorkoutLoaded(WorkoutRes, Vec<WorkoutExerciseRes>),
    Saved,
    Deleted,
    Error(String),
}

// top show workout details with comprehensive details of each exercise
// bottom show log form
// add button to save entire log group and log entries all at once
// need a row for each set per exercise which is a log entry
// add checkbox state for each set to indicate whether it was completed or not
// checked sets should be greyed out (but still editable)
// clicking away from current page should show warning about unsaved changes being lost

impl StartWorkoutPage {
    fn handle_async_messages(&mut self) {}

    fn render_workout(&mut self, ui: &mut egui::Ui) {
        assert!(self.current_workout.is_some());
        if let Some(workout) = &self.current_workout {
            todo!("Render workout details here");
        }

        if self.current_exercises.is_empty() {
            ui.label("No exercises found for this workout!");
        }
    }

    fn render_log_form(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        todo!("Render log form here");
    }

    pub fn load_workout(&mut self, workout_id: u32) {
        self.current_workout_id = Some(workout_id);
    }

    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> PageAction {
        ui.heading("Start Workout");
        if let Some(workout_id) = self.current_workout_id {
            ui.label(format!("Workout ID: {}", workout_id));
        } else {
            ui.label("No workout selected, choose from the Workouts page.");
        }
        PageAction::None
    }
}

use crate::app::utils::CommonUiState;
use crate::db::pagination_support::PaginationState;
use crate::workout::workout_dto::WorkoutsFilterReq;
use eframe::egui;
use sqlx::{Pool, Sqlite};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct WorkoutsPage {
    pool: Pool<Sqlite>,
    state: WorkoutsPageState,
    // Workout Search/Filter State
    workout_filters: WorkoutsFilterReq,
    // Workout Pagination State
    workout_pagination_state: PaginationState,
    // Async Communication
    receiver: Receiver<WorkoutsPageMsg>,
    sender: Sender<WorkoutsPageMsg>,
    // UI Status
    common_ui_state: CommonUiState,
}

pub enum WorkoutsPageMsg {
    ListLoaded,
    DetailLoaded,
    Saved,
    Deleted,
    Error(String),
}

impl WorkoutsPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        let (sender, receiver) = channel();
        Self {
            pool,
            state: WorkoutsPageState::DetailsClosed,
            workout_filters: WorkoutsFilterReq::default(),
            workout_pagination_state: PaginationState::default(),
            receiver,
            sender,
            common_ui_state: CommonUiState::default(),
        }
    }
}

pub enum WorkoutsPageState {
    DetailsClosed,
    DetailsOpenView,
    DetailsEditView,
    CreateNew,
}

/* aiming for something like this for the form:
[ Create Workout ]

├───────────────────────────────────────────────┤
│  YOUR WORKOUT PLAN                            │
├───────────────────────────────────────────────┤
│  A1. BENCH PRESS                             │
│  • 4×10 @ 80kg  [✏️]  [×]                     │
│  (Tap pencil to edit sets/reps/weight)         │
├───────────────────────────────────────────────┤
│  A2. PULL-UP                                 │
│  • 3×8 (BW)       [✏️]  [×]                   │
├───────────────────────────────────────────────┤
│  A3. SQUAT                                    │
│  • 5×5 @ 100kg    [✏️]  [×]                   │
├───────────────────────────────────────────────┤
│  [ + ADD EXERCISE ]                           │
├───────────────────────────────────────────────┤
│  [ START THIS WORKOUT ]                       │
└───────────────────────────────────────────────┘
 */

impl WorkoutsPage {
    fn handle_async_messages(&mut self) {}

    fn render_workout_exercise_form(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // for editing the exercise code, sets, reps, weight, etc. of an exercise in a workout
    }

    fn render_workout_builder_form(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // the main form for building a workout, with the exercise list and the exercise forms
        // should be able to add exercises from the list, remove exercises.
    }

    fn render_workout_details_open_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // the details view for a workout, showing the exercises and their details
    }

    fn render_workout_details_edit_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // the details view for a workout, showing the exercises and their details at editable state
    }

    fn render_workouts_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // render the list of workouts, with a button to create a new workout,
        // a button to edit the selected workout and a button to delete the workout
    }

    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        todo!("render workouts page");
    }
}

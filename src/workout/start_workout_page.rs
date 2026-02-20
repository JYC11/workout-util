use crate::client::app::PageAction;
use crate::client::app_utils::CommonUiState;
use crate::workout::workout_dto::{RestMinuteAndSeconds, WorkoutExerciseRes, WorkoutRes};
use crate::workout::workout_service::WorkoutService;
use crate::workout_log::workout_log_dto::{WorkoutLogGroupReq, WorkoutLogReq};
use crate::workout_log::workout_log_service::WorkoutLogService;
use eframe::egui;
use sqlx::{Pool, Sqlite};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct StartWorkoutPage {
    current_workout_id: Option<u32>,
    // Local state for the active session
    active_session: Option<ActiveSession>,
    workout_service: WorkoutService,
    workout_log_service: WorkoutLogService,
    // Async Communication
    receiver: Receiver<StartWorkoutsPageMsg>,
    sender: Sender<StartWorkoutsPageMsg>,
    // UI Status
    common_ui_state: CommonUiState,
}

// Temporary structs to hold form state
#[derive(Debug, Clone)]
struct ActiveSession {
    name: String,
    exercises: Vec<ActiveExercise>,
    description: Option<String>,
}

#[derive(Debug, Clone)]
struct ActiveExercise {
    workout_exercise_id: u32,
    exercise_name: String,
    reps_or_seconds_target: u8,
    working_weight: u16,
    rest_period_seconds: u8,
    tempo: String,
    emom: bool,
    sets: Vec<ActiveSet>,
}

impl ActiveExercise {
    fn new(res: WorkoutExerciseRes, active_sets: Vec<ActiveSet>) -> Self {
        Self {
            workout_exercise_id: res.id,
            exercise_name: res.name,
            reps_or_seconds_target: res.reps_or_seconds_target,
            working_weight: res.working_weight,
            rest_period_seconds: res.rest_period_seconds,
            tempo: res.tempo,
            emom: res.emom,
            sets: active_sets,
        }
    }
}

impl RestMinuteAndSeconds for ActiveExercise {
    fn rest_minutes_and_seconds(&self) -> String {
        format!(
            "{}m {}s rest",
            self.rest_period_seconds / 60,
            self.rest_period_seconds % 60
        )
    }
}

#[derive(Debug, Clone)]
struct ActiveSet {
    set_number: u8,
    weight: u16,
    reps_or_seconds: u8,
    description: String,
    completed: bool,
}

impl ActiveSet {
    fn new(set_number: u8, weight: u16) -> Self {
        Self {
            set_number,
            weight,
            reps_or_seconds: 0,
            description: "".to_string(),
            completed: false,
        }
    }

    fn to_workout_log_req(
        &self,
        workout_id: u32,
        workout_exercise_id: u32,
        exercise_name: String,
    ) -> WorkoutLogReq {
        WorkoutLogReq {
            workout_id,
            workout_exercise_id,
            workout_log_group_id: 0,
            exercise_name,
            set_number: self.set_number,
            weight: self.weight,
            rep_number_or_seconds: self.reps_or_seconds,
            description: if self.description.is_empty() {
                None
            } else {
                Some(self.description.clone())
            },
        }
    }
}

impl StartWorkoutPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        let (sender, receiver) = channel();
        Self {
            current_workout_id: None,
            active_session: None,
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

impl StartWorkoutPage {
    fn handle_async_messages(&mut self) {
        while let Ok(msg) = self.receiver.try_recv() {
            self.common_ui_state.set_as_not_loading();
            match msg {
                StartWorkoutsPageMsg::WorkoutLoaded(workout, exercises) => {
                    // Initialize active session state from the loaded workout
                    let active_exercises = exercises
                        .iter()
                        .map(|e| {
                            let sets = (1..=e.sets_target)
                                .map(|i| ActiveSet::new(i, e.working_weight))
                                .collect();
                            ActiveExercise::new(e.clone(), sets)
                        })
                        .collect();

                    self.active_session = Some(ActiveSession {
                        name: workout.name.clone(),
                        exercises: active_exercises,
                        description: format!("Workout name: {}", workout.name).into(),
                    });
                }
                StartWorkoutsPageMsg::Saved => {
                    self.common_ui_state
                        .show_success("Workout logged successfully!");
                    // Reset state or navigate away?
                    // For now, we keep the ID but maybe clear the form?
                    // self.active_session = None;
                    // self.current_workout_id = None;
                }
                StartWorkoutsPageMsg::Error(e) => {
                    self.common_ui_state.show_error(&e);
                }
                _ => {}
            }
        }
    }

    fn render_workout(&mut self, ui: &mut egui::Ui) -> PageAction {
        let mut page_action = PageAction::None;

        if let Some(session) = &mut self.active_session {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // 1. Render Session Description Edit
                ui.label("Workout Notes:");
                let mut description = session.description.clone().unwrap_or_default();
                if ui
                    .add(
                        egui::TextEdit::multiline(&mut description)
                            .hint_text("How are you feeling today?"),
                    )
                    .changed()
                {
                    session.description = if description.is_empty() {
                        None
                    } else {
                        Some(description)
                    };
                }
                ui.button("Go to workout").clicked().then(|| {
                    page_action = PageAction::GoToWorkoutDetails(self.current_workout_id.unwrap());
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                for (ex_idx, exercise) in session.exercises.iter_mut().enumerate() {
                    let id = ui.make_persistent_id(format!("ex_{}", ex_idx));
                    egui::CollapsingHeader::new(&exercise.exercise_name)
                        .id_salt(id)
                        .default_open(true)
                        .show(ui, |ui| {
                            // 2. Render ActiveExercise Details (Targets)
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Target: {} {}",
                                        exercise.reps_or_seconds_target, "seconds/reps"
                                    ))
                                    .strong(),
                                );
                                ui.label("|");
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Weight: {}kg",
                                        exercise.working_weight
                                    ))
                                    .strong(),
                                );
                                ui.label("|");
                                ui.label(exercise.rest_minutes_and_seconds());

                                if !exercise.tempo.is_empty() {
                                    ui.label("|");
                                    ui.label(format!("Tempo: {}", exercise.tempo));
                                }
                                if exercise.emom {
                                    ui.label("|");
                                    ui.label(
                                        egui::RichText::new("EMOM")
                                            .color(egui::Color32::WHITE)
                                            .background_color(egui::Color32::RED),
                                    );
                                }
                            });
                            ui.separator();

                            egui::Grid::new(format!("grid_{}", ex_idx))
                                .striped(true)
                                .min_col_width(50.0)
                                .spacing([10.0, 4.0])
                                .show(ui, |ui| {
                                    ui.label("Set");
                                    ui.label("Weight");
                                    ui.label("Reps/Seconds");
                                    ui.label("Description");
                                    ui.label("Done");
                                    ui.end_row();

                                    for set in &mut exercise.sets {
                                        ui.label(format!("{}", set.set_number));

                                        let mut w_ui = ui
                                            .add(egui::DragValue::new(&mut set.weight).speed(0.1));
                                        if set.completed {
                                            w_ui = w_ui.on_hover_text("Set completed");
                                        }

                                        let mut r_ui = ui.add(
                                            egui::DragValue::new(&mut set.reps_or_seconds)
                                                .speed(0.1),
                                        );
                                        if set.completed {
                                            r_ui = r_ui.on_hover_text("Set completed");
                                        }

                                        let mut rpe_ui =
                                            ui.text_edit_singleline(&mut set.description);
                                        if set.completed {
                                            rpe_ui = rpe_ui.on_hover_text("Set completed");
                                        }

                                        ui.checkbox(&mut set.completed, "");
                                        ui.end_row();
                                    }
                                });
                        });
                    ui.add_space(10.0);
                }
            });
        } else {
            ui.label("Loading workout...");
        }
        page_action
    }

    fn render_log_form(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // The log form is essentially integrated into the workout view in the "Clipboard" style
        // This function might be redundant if we merge logic, but we'll keep the footer actions here.

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                self.active_session = None;
                self.current_workout_id = None;
            }

            let valid_to_save = self
                .active_session
                .as_ref()
                .map(|s| {
                    s.exercises
                        .iter()
                        .any(|e| e.sets.iter().any(|set| set.completed))
                })
                .unwrap_or(false);

            if ui
                .add_enabled(valid_to_save, egui::Button::new("Finish & Save Log"))
                .clicked()
            {
                self.save_log(ctx);
            }
        });
    }

    fn save_log(&mut self, ctx: &egui::Context) {
        if let Some(session) = &self.active_session {
            self.common_ui_state.set_as_loading();

            let current_workout_id = self
                .current_workout_id
                .expect("Current workout ID should be set");

            // Map ActiveSession to WorkoutLogReq
            let entries: Vec<WorkoutLogReq> = session
                .exercises
                .iter()
                .flat_map(|ex| {
                    ex.sets.iter().filter(|s| s.completed).map(move |s| {
                        s.to_workout_log_req(
                            current_workout_id,
                            ex.workout_exercise_id,
                            ex.exercise_name.clone(),
                        )
                    })
                })
                .collect();

            let log_req = WorkoutLogGroupReq::new(session.description.clone());

            let sender = self.sender.clone();
            let service = self.workout_log_service.clone();
            let ctx = ctx.clone();

            tokio::spawn(async move {
                match service.create_log_group(log_req, entries).await {
                    Ok(_) => {
                        let _ = sender.send(StartWorkoutsPageMsg::Saved);
                    }
                    Err(e) => {
                        let _ = sender.send(StartWorkoutsPageMsg::Error(e));
                    }
                }
                ctx.request_repaint();
            });
        }
    }

    pub fn load_workout(&mut self, ctx: &egui::Context, workout_id: u32) {
        self.current_workout_id = Some(workout_id);
        self.active_session = None;
        self.common_ui_state.set_as_loading();

        // Trigger fetch
        let sender = self.sender.clone();
        let service = self.workout_service.clone();
        let ctx = ctx.clone();

        tokio::spawn(async move {
            match service.get_one(workout_id).await {
                Ok(workout) => match service.get_all_exercises_by_workout_id(workout_id).await {
                    Ok(exercises) => {
                        let _ =
                            sender.send(StartWorkoutsPageMsg::WorkoutLoaded(workout, exercises));
                    }
                    Err(e) => {
                        let _ = sender.send(StartWorkoutsPageMsg::Error(e));
                    }
                },
                Err(e) => {
                    let _ = sender.send(StartWorkoutsPageMsg::Error(e));
                }
            }
            ctx.request_repaint();
        });
    }

    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> PageAction {
        self.handle_async_messages();
        self.common_ui_state.show_toasts(ui);

        ui.heading("Start Workout");

        if self.current_workout_id.is_none() {
            ui.label("No workout selected. Go to Workouts page to start one.");
            return PageAction::None;
        }

        if let Some(session) = &self.active_session {
            ui.label(format!("Workout: {}", session.name));
        }

        // Main Content
        let page_action = self.render_workout(ui);

        // Footer Actions
        self.render_log_form(ctx, ui);

        page_action
    }
}

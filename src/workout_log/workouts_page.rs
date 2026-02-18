use crate::client::app::PageAction;
use crate::client::app_utils::CommonUiState;
use crate::db::pagination_support::{PaginationRes, PaginationState};
use crate::workout::workout_dto::{
    WorkoutExerciseReq, WorkoutExerciseRes, WorkoutReq, WorkoutRes, WorkoutsFilterReq,
    default_exercise_req, default_workout_req, exercise_res_to_req, workout_to_req,
};
use crate::workout::workout_service::WorkoutService;
use eframe::egui;
use sqlx::{Pool, Sqlite};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct WorkoutsPage {
    service: WorkoutService,
    state: WorkoutsPageState,
    // Data
    list_items: Vec<WorkoutRes>,
    current_workout: Option<WorkoutRes>,
    current_exercises: Vec<WorkoutExerciseRes>,
    // Form State (Workout)
    form_workout: WorkoutReq,
    // Form State (Exercise)
    show_exercise_form: bool,
    form_exercise: WorkoutExerciseReq,
    editing_exercise_id: Option<u32>,
    // For Create New Workout mode:
    new_workout_exercises: Vec<WorkoutExerciseReq>,
    // Search/Filter State
    workout_filters: WorkoutsFilterReq,
    // Pagination State
    workout_pagination_state: PaginationState,
    // Async Communication
    receiver: Receiver<WorkoutsPageMsg>,
    sender: Sender<WorkoutsPageMsg>,
    // UI Status
    common_ui_state: CommonUiState,
}

pub enum WorkoutsPageMsg {
    ListLoaded(PaginationRes<WorkoutRes>),
    DetailLoaded(WorkoutRes, Vec<WorkoutExerciseRes>),
    Saved,
    Deleted,
    Error(String),
}

impl WorkoutsPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        let (sender, receiver) = channel();
        Self {
            service: WorkoutService::new(pool),
            state: WorkoutsPageState::DetailsClosed,
            list_items: Vec::new(),
            current_workout: None,
            current_exercises: Vec::new(),
            form_workout: default_workout_req(),
            show_exercise_form: false,
            form_exercise: default_exercise_req(0),
            editing_exercise_id: None,
            new_workout_exercises: Vec::new(),
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

impl WorkoutsPage {
    fn handle_async_messages(&mut self) {
        while let Ok(msg) = self.receiver.try_recv() {
            self.common_ui_state.set_as_not_loading();
            match msg {
                WorkoutsPageMsg::ListLoaded(res) => {
                    self.list_items = res.items;
                    self.workout_pagination_state.next_cursor = res.next_cursor;
                    self.workout_pagination_state.prev_cursor = res.prev_cursor;
                }
                WorkoutsPageMsg::DetailLoaded(workout, exercises) => {
                    self.current_workout = Some(workout.clone());
                    self.current_exercises = exercises;
                    self.form_workout = workout_to_req(&workout);
                    self.new_workout_exercises.clear();
                }
                WorkoutsPageMsg::Saved => {
                    self.common_ui_state.show_success("Saved successfully");
                    if matches!(self.state, WorkoutsPageState::CreateNew) {
                        self.state = WorkoutsPageState::DetailsClosed;
                        self.trigger_list_refresh();
                    } else if matches!(self.state, WorkoutsPageState::DetailsEditView) {
                        // If we were editing, refresh the details
                        if let Some(w) = &self.current_workout {
                            self.fetch_detail(w.id);
                        }
                    }
                }
                WorkoutsPageMsg::Deleted => {
                    self.common_ui_state.show_success("Deleted successfully");
                    self.state = WorkoutsPageState::DetailsClosed;
                    self.trigger_list_refresh();
                }
                WorkoutsPageMsg::Error(e) => {
                    self.common_ui_state.show_error(&e);
                }
            }
        }
    }

    fn trigger_list_refresh(&mut self) {
        self.common_ui_state.not_initialized();
    }

    fn fetch_list(&mut self, ctx: &egui::Context) {
        if self.common_ui_state.loading {
            return;
        }
        self.common_ui_state.set_as_loading();

        let sender = self.sender.clone();
        let filter = self.workout_filters.clone();
        let params = self.workout_pagination_state.to_pagination_params();
        let ctx = ctx.clone();
        let service = self.service.clone();
        tokio::spawn(async move {
            match service.paginate(Some(filter), params).await {
                Ok(res) => {
                    let _ = sender.send(WorkoutsPageMsg::ListLoaded(res));
                }
                Err(e) => {
                    let _ = sender.send(WorkoutsPageMsg::Error(e));
                }
            }
            ctx.request_repaint();
        });
    }

    fn fetch_detail(&mut self, id: u32) {
        self.common_ui_state.set_as_loading();
        let sender = self.sender.clone();
        let service = self.service.clone();
        tokio::spawn(async move {
            match service.get_one(id).await {
                Ok(workout) => match service.get_all_exercises_by_workout_id(id).await {
                    Ok(exercises) => {
                        let _ = sender.send(WorkoutsPageMsg::DetailLoaded(workout, exercises));
                    }
                    Err(e) => {
                        let _ = sender.send(WorkoutsPageMsg::Error(e));
                    }
                },
                Err(e) => {
                    let _ = sender.send(WorkoutsPageMsg::Error(e));
                }
            }
        });
    }

    fn save_workout(&mut self, ctx: &egui::Context) {
        self.common_ui_state.set_as_loading();
        let sender = self.sender.clone();
        let req = self.form_workout.clone();
        let ctx = ctx.clone();
        let service = self.service.clone();

        // Use CreateNew logic vs Edit logic
        if matches!(self.state, WorkoutsPageState::CreateNew) {
            let exercises = self.new_workout_exercises.clone();
            tokio::spawn(async move {
                match service.create(req, exercises).await {
                    Ok(_) => {
                        let _ = sender.send(WorkoutsPageMsg::Saved);
                    }
                    Err(e) => {
                        let _ = sender.send(WorkoutsPageMsg::Error(e));
                    }
                }
                ctx.request_repaint();
            });
        } else if let Some(current) = &self.current_workout {
            let id = current.id;
            // We only update the workout details here.
            // Exercises are updated individually in the edit view.
            tokio::spawn(async move {
                match service.update(id, req, vec![]).await {
                    Ok(_) => {
                        let _ = sender.send(WorkoutsPageMsg::Saved);
                    }
                    Err(e) => {
                        let _ = sender.send(WorkoutsPageMsg::Error(e));
                    }
                }
                ctx.request_repaint();
            });
        }
    }

    fn save_exercise(&mut self, ctx: &egui::Context) {
        // Logic depends on whether we are in CreateNew or EditView
        if matches!(self.state, WorkoutsPageState::CreateNew) {
            // Memory only
            if let Some(idx) = self.editing_exercise_id {
                // Editing an exercise in the 'new_workout_exercises' list (using idx as index)
                if (idx as usize) < self.new_workout_exercises.len() {
                    self.new_workout_exercises[idx as usize] = self.form_exercise.clone();
                }
            } else {
                // Add new
                self.new_workout_exercises.push(self.form_exercise.clone());
            }
            self.show_exercise_form = false;
        } else {
            // DB operation
            self.common_ui_state.set_as_loading();
            let sender = self.sender.clone();
            let mut req = self.form_exercise.clone();
            let ctx = ctx.clone();
            let service = self.service.clone();
            let workout_id = self.current_workout.as_ref().map(|w| w.id).unwrap_or(0);

            // Ensure workout_id is correct
            req.workout_id = workout_id;

            if let Some(ex_id) = self.editing_exercise_id {
                // Update
                tokio::spawn(async move {
                    match service.update_exercise(ex_id, req).await {
                        Ok(_) => {
                            let _ = sender.send(WorkoutsPageMsg::Saved);
                        }
                        Err(e) => {
                            let _ = sender.send(WorkoutsPageMsg::Error(e));
                        }
                    }
                    ctx.request_repaint();
                });
            } else {
                // Create
                tokio::spawn(async move {
                    match service.create_exercise(req).await {
                        Ok(_) => {
                            let _ = sender.send(WorkoutsPageMsg::Saved);
                        }
                        Err(e) => {
                            let _ = sender.send(WorkoutsPageMsg::Error(e));
                        }
                    }
                    ctx.request_repaint();
                });
            }
            self.show_exercise_form = false;
        }
    }

    fn delete_workout(&mut self, ctx: &egui::Context) {
        if let Some(w) = &self.current_workout {
            let id = w.id;
            self.common_ui_state.set_as_loading();
            let sender = self.sender.clone();
            let ctx = ctx.clone();
            let service = self.service.clone();
            tokio::spawn(async move {
                match service.delete(id).await {
                    Ok(_) => {
                        let _ = sender.send(WorkoutsPageMsg::Deleted);
                    }
                    Err(e) => {
                        let _ = sender.send(WorkoutsPageMsg::Error(e));
                    }
                }
                ctx.request_repaint();
            });
        }
    }

    fn delete_workout_by_id(&mut self, ctx: &egui::Context, id: u32) {
        self.common_ui_state.set_as_loading();
        let sender = self.sender.clone();
        let service = self.service.clone();
        let ctx = ctx.clone();
        tokio::spawn(async move {
            match service.delete(id).await {
                Ok(_) => {
                    let _ = sender.send(WorkoutsPageMsg::Deleted);
                }
                Err(e) => {
                    let _ = sender.send(WorkoutsPageMsg::Error(e));
                }
            }
            ctx.request_repaint();
        });
    }

    fn delete_exercise(&mut self, ctx: &egui::Context, id: u32) {
        if matches!(self.state, WorkoutsPageState::CreateNew) {
            // Remove from memory list (id treated as index)
            if (id as usize) < self.new_workout_exercises.len() {
                self.new_workout_exercises.remove(id as usize);
            }
        } else {
            // DB Delete
            self.common_ui_state.set_as_loading();
            let sender = self.sender.clone();
            let ctx = ctx.clone();
            let service = self.service.clone();
            tokio::spawn(async move {
                match service.delete_exercise(id).await {
                    Ok(_) => {
                        let _ = sender.send(WorkoutsPageMsg::Saved);
                    } // Saved triggers reload
                    Err(e) => {
                        let _ = sender.send(WorkoutsPageMsg::Error(e));
                    }
                }
                ctx.request_repaint();
            });
        }
    }

    fn render_exercise_form(&mut self, ui: &mut egui::Ui) {
        let req = &mut self.form_exercise;
        ui.label("Code (e.g. A1)");
        ui.text_edit_singleline(&mut req.code);
        ui.label("Name");
        ui.text_edit_singleline(&mut req.name);
        egui::Grid::new("exercise_form_grid")
            .striped(true)
            .show(ui, |ui| {
                ui.label("Sets");
                ui.add(egui::DragValue::new(&mut req.sets_target));
                ui.end_row();

                ui.label("Reps/Secs");
                ui.add(egui::DragValue::new(&mut req.reps_or_seconds_target));
                ui.end_row();

                ui.label("Weight");
                ui.add(egui::DragValue::new(&mut req.working_weight));
                ui.end_row();

                ui.label("Rest (s)");
                ui.add(egui::DragValue::new(&mut req.rest_period_seconds));
                ui.end_row();
            });

        ui.label("Tempo");
        ui.text_edit_singleline(&mut req.tempo);
        ui.checkbox(&mut req.emom, "EMOM");
        ui.label("Description");
        let mut desc = req.description.clone().unwrap_or_default();
        ui.text_edit_multiline(&mut desc);
        req.description = if desc.is_empty() { None } else { Some(desc) };
    }

    fn render_workout_form(&mut self, ui: &mut egui::Ui) {
        ui.label("Name");
        ui.text_edit_singleline(&mut self.form_workout.name);

        ui.label("Description");
        let mut desc = self.form_workout.description.clone().unwrap_or_default();
        ui.text_edit_multiline(&mut desc);
        self.form_workout.description = if desc.is_empty() { None } else { Some(desc) };

        ui.checkbox(&mut self.form_workout.active, "Active");
    }

    fn render_details_open_view(&mut self, ui: &mut egui::Ui) -> PageAction {
        ui.heading("Workout Details");
        ui.separator();
        let mut page_action = PageAction::None;

        if let Some(w) = &self.current_workout {
            ui.label(egui::RichText::new(&w.name).strong().size(18.0));
            if let Some(d) = &w.description {
                ui.label(d);
            }
            ui.label(if w.active { "Active" } else { "Inactive" });
        }

        ui.separator();
        ui.heading("Exercises");

        egui::Grid::new("view_exercises_grid")
            .striped(true)
            .show(ui, |ui| {
                for ex in &self.current_exercises {
                    ui.label(egui::RichText::new(&ex.code).strong());
                    ui.label(&ex.name);
                    ui.label(format!(
                        "{}x{} @ {}kg",
                        ex.sets_target, ex.reps_or_seconds_target, ex.working_weight
                    ));
                    ui.end_row();
                }
            });

        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.button("Close").clicked() {
                self.state = WorkoutsPageState::DetailsClosed;
                page_action = PageAction::None
            }
            if ui.button("Edit").clicked() {
                if let Some(w) = &self.current_workout {
                    self.form_workout = workout_to_req(w);
                    self.state = WorkoutsPageState::DetailsEditView;
                    self.show_exercise_form = false;
                }
                page_action = PageAction::None;
            }
            if ui.button("Start Workout").clicked() {
                page_action =
                    PageAction::GoToStartWorkout(self.current_workout.as_ref().unwrap().id);
            }
        });

        page_action
    }

    fn render_details_edit_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> PageAction {
        if let Some(value) = self.show_exercise_sub_form(ctx, ui) {
            return value;
        }

        ui.heading("Edit Workout");
        ui.separator();

        self.render_workout_form(ui);

        ui.separator();
        ui.horizontal(|ui| {
            ui.heading("Exercises");
            if ui.button("+ Add Exercise").clicked() {
                self.form_exercise =
                    default_exercise_req(self.current_workout.as_ref().map(|w| w.id).unwrap_or(0));
                self.editing_exercise_id = None;
                self.show_exercise_form = true;
            }
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("edit_exercises_grid")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Code");
                    ui.label("Name");
                    ui.label("Target");
                    ui.label("Actions");
                    ui.end_row();

                    let mut edit_id = None;
                    let mut delete_id = None;

                    for ex in &self.current_exercises {
                        ui.label(&ex.code);
                        ui.label(&ex.name);
                        ui.label(format!(
                            "{}x{} @ {}",
                            ex.sets_target, ex.reps_or_seconds_target, ex.working_weight
                        ));

                        ui.horizontal(|ui| {
                            if ui.button("Edit").clicked() {
                                edit_id = Some(ex.id);
                            }
                            if ui.button("Delete").clicked() {
                                delete_id = Some(ex.id);
                            }
                        });
                        ui.end_row();
                    }

                    if let Some(id) = edit_id {
                        // Find the exercise and load form
                        if let Some(ex) = self.current_exercises.iter().find(|e| e.id == id) {
                            self.form_exercise = exercise_res_to_req(ex);
                            self.editing_exercise_id = Some(id);
                            self.show_exercise_form = true;
                        }
                    }
                    if let Some(id) = delete_id {
                        self.delete_exercise(ctx, id);
                    }
                });
        });

        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.button("Close").clicked() {
                self.state = WorkoutsPageState::DetailsClosed;
            }
            if ui.button("Save Workout Details").clicked() {
                self.save_workout(ctx);
            }
            if ui.button("Delete Workout").clicked() {
                self.delete_workout(ctx);
            }
        });

        PageAction::None
    }

    fn render_create(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> PageAction {
        if let Some(value) = self.show_exercise_sub_form(ctx, ui) {
            return value;
        }

        ui.heading("Create New Workout");
        ui.separator();

        self.render_workout_form(ui);

        ui.separator();
        ui.horizontal(|ui| {
            ui.heading("Exercises");
            if ui.button("+ Add Exercise").clicked() {
                self.form_exercise = default_exercise_req(0);
                self.editing_exercise_id = None;
                self.show_exercise_form = true;
            }
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("create_exercises_grid")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Code");
                    ui.label("Name");
                    ui.label("Target");
                    ui.label("Actions");
                    ui.end_row();

                    let mut edit_idx = None;
                    let mut delete_idx = None;

                    for (idx, ex) in self.new_workout_exercises.iter().enumerate() {
                        ui.label(&ex.code);
                        ui.label(&ex.name);
                        ui.label(format!(
                            "{}x{} @ {}",
                            ex.sets_target, ex.reps_or_seconds_target, ex.working_weight
                        ));

                        ui.horizontal(|ui| {
                            if ui.button("Edit").clicked() {
                                edit_idx = Some(idx);
                            }
                            if ui.button("Delete").clicked() {
                                delete_idx = Some(idx);
                            }
                        });
                        ui.end_row();
                    }

                    if let Some(idx) = edit_idx {
                        self.form_exercise = self.new_workout_exercises[idx].clone();
                        self.editing_exercise_id = Some(idx as u32);
                        self.show_exercise_form = true;
                    }
                    if let Some(idx) = delete_idx {
                        self.delete_exercise(ctx, idx as u32);
                    }
                });
        });

        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                self.state = WorkoutsPageState::DetailsClosed;
            }
            if ui.button("Save Workout").clicked() {
                self.save_workout(ctx);
            }
        });

        PageAction::None
    }

    fn show_exercise_sub_form(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
    ) -> Option<PageAction> {
        if self.show_exercise_form {
            ui.heading(if self.editing_exercise_id.is_some() {
                "Edit Exercise (New)"
            } else {
                "Add Exercise (New)"
            });
            self.render_exercise_form(ui);
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    self.show_exercise_form = false;
                }
                if ui.button("Save Exercise").clicked() {
                    self.save_exercise(ctx);
                }
            });
            ui.separator();
            return Some(PageAction::None);
        }
        None
    }

    fn render_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> PageAction {
        let mut page_action = PageAction::None;

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            // Pagination controls
            ui.horizontal(|ui| {
                ui.label("Limit:");
                let mut limit = self.workout_pagination_state.limit;
                if ui
                    .add(egui::DragValue::new(&mut limit).speed(1.0).range(1..=100))
                    .changed()
                {
                    self.workout_pagination_state.limit = limit;
                    self.trigger_list_refresh();
                }

                if self.workout_pagination_state.has_previous() {
                    if ui.button("Previous").clicked() {
                        self.workout_pagination_state.go_backwards();
                        self.trigger_list_refresh();
                    }
                }
                if self.workout_pagination_state.has_next() {
                    if ui.button("Next").clicked() {
                        self.workout_pagination_state.go_forwards();
                        self.trigger_list_refresh();
                    }
                }
            });
            ui.separator();

            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Workouts");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Add New").clicked() {
                            self.state = WorkoutsPageState::CreateNew;
                            self.form_workout = default_workout_req();
                            self.new_workout_exercises.clear();
                            self.show_exercise_form = false;
                        }
                    });
                });
                ui.separator();

                // Filters
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    let mut name = self.workout_filters.name.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut name).changed() {
                        self.workout_filters.name = if name.is_empty() { None } else { Some(name) };
                        self.workout_pagination_state.reset_pagination();
                        self.trigger_list_refresh();
                    }

                    ui.label("Description:");
                    let mut desc = self.workout_filters.description.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut desc).changed() {
                        self.workout_filters.description =
                            if desc.is_empty() { None } else { Some(desc) };
                        self.workout_pagination_state.reset_pagination();
                        self.trigger_list_refresh();
                    }

                    ui.label("Active:");
                    let mut active = self.workout_filters.active;
                    egui::ComboBox::from_id_salt("workout_active_filter")
                        .selected_text(match active {
                            Some(true) => "Active",
                            Some(false) => "Inactive",
                            None => "All",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut active, None, "All");
                            ui.selectable_value(&mut active, Some(true), "Active");
                            ui.selectable_value(&mut active, Some(false), "Inactive");
                        });

                    if active != self.workout_filters.active {
                        self.workout_filters.active = active;
                        self.workout_pagination_state.reset_pagination();
                        self.trigger_list_refresh();
                    }
                });

                ui.separator();

                enum ListAction {
                    Details(u32),
                    Edit(u32),
                    Delete(u32),
                    StartWorkout(u32),
                }
                let mut action = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("workout_list_grid")
                        .striped(true)
                        .min_col_width(100.0)
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.label("Active");
                            ui.label("Actions");
                            ui.end_row();

                            for item in &self.list_items {
                                ui.label(&item.name);
                                ui.label(if item.active { "Yes" } else { "No" });
                                ui.horizontal(|ui| {
                                    if ui.button("Details").clicked() {
                                        action = Some(ListAction::Details(item.id));
                                    }
                                    if ui.button("Edit").clicked() {
                                        action = Some(ListAction::Edit(item.id));
                                    }
                                    if ui.button("Delete").clicked() {
                                        action = Some(ListAction::Delete(item.id));
                                    }
                                    if ui.button("Start Workout").clicked() {
                                        action = Some(ListAction::StartWorkout(item.id));
                                    }
                                });
                                ui.end_row();
                            }
                        });
                });

                page_action = if let Some(act) = action {
                    match act {
                        ListAction::Details(id) => {
                            self.fetch_detail(id);
                            self.state = WorkoutsPageState::DetailsOpenView;
                            PageAction::None
                        }
                        ListAction::Edit(id) => {
                            self.fetch_detail(id);
                            self.state = WorkoutsPageState::DetailsEditView;
                            PageAction::None
                        }
                        ListAction::Delete(id) => {
                            self.delete_workout_by_id(ctx, id);
                            PageAction::None
                        }
                        ListAction::StartWorkout(id) => PageAction::GoToStartWorkout(id),
                    }
                } else {
                    PageAction::None
                };
            });
        });

        page_action
    }

    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> PageAction {
        self.handle_async_messages();

        self.common_ui_state.show_toasts(ui);

        if !self.common_ui_state.initialized
            && matches!(self.state, WorkoutsPageState::DetailsClosed)
        {
            self.fetch_list(ctx);
            self.common_ui_state.initialize();
        }

        match self.state {
            WorkoutsPageState::DetailsClosed => self.render_list(ctx, ui),
            WorkoutsPageState::DetailsOpenView => self.render_details_open_view(ui),
            WorkoutsPageState::DetailsEditView => self.render_details_edit_view(ctx, ui),
            WorkoutsPageState::CreateNew => self.render_create(ctx, ui),
        }
    }
}

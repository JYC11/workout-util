use crate::app::utils;
use crate::db::pagination_support::{
    PaginationDirection, PaginationParams, PaginationRes, PaginationState,
};
use crate::workout::enums::{
    CompoundOrIsolation, DynamicOrStatic, Grip, GripWidth, LeverVariation, PushOrPull,
    SquatOrHinge, StraightOrBentArm, UpperOrLower,
};
use crate::workout::exercise::{
    ExerciseLibraryEntity, create_exercise, delete_exercise, get_one_exercise, paginate_exercises,
    update_exercise,
};
use crate::workout::exercise_dto::{
    ExerciseLibraryFilterReq, ExerciseLibraryReq, ExerciseLibraryRes, ValidExercise,
    exercise_library_default_req, exercise_to_req, get_exercise_id, get_exercise_name,
};
use eframe::egui;
use sqlx::{Pool, Sqlite};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

pub struct ExercisesPage {
    state: ExercisesPageState,

    // Data
    list_items: Vec<ExerciseLibraryRes>,
    current_detail: Option<ValidExercise>,

    // Form State
    form_data: ExerciseLibraryReq,

    // Search/Filter State
    filter_name: String,
    pagination_filters: ExerciseLibraryFilterReq,

    // Pagination State
    pagination_state: PaginationState,

    // Async Communication
    receiver: Receiver<ExercisesPageMsg>,
    sender: Sender<ExercisesPageMsg>,

    // UI Status
    error_message: Option<(String, Instant)>,
    success_message: Option<(String, Instant)>,
    loading: bool,
    initialized: bool,
}

pub enum ExercisesPageMsg {
    ListLoaded(PaginationRes<ExerciseLibraryRes>),
    DetailLoaded(ValidExercise),
    Saved,
    Deleted,
    Error(String),
}

impl Default for ExercisesPage {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            state: ExercisesPageState::DetailsClosed,
            list_items: Vec::new(),
            current_detail: None,
            form_data: exercise_library_default_req(),
            filter_name: String::new(),
            pagination_filters: ExerciseLibraryFilterReq::default(),
            pagination_state: PaginationState::default(),
            receiver,
            sender,
            error_message: None,
            success_message: None,
            loading: false,
            initialized: false,
        }
    }
}

pub enum ExercisesPageState {
    DetailsClosed,
    DetailsOpenView,
    DetailsEditView,
    CreateNew,
}

impl ExercisesPage {
    fn handle_async_messages(&mut self) {
        while let Ok(msg) = self.receiver.try_recv() {
            self.loading = false;
            match msg {
                ExercisesPageMsg::ListLoaded(res) => {
                    self.list_items = res.items;
                    self.pagination_state.next_cursor = res.next_cursor;
                    self.pagination_state.prev_cursor = res.prev_cursor;
                }
                ExercisesPageMsg::DetailLoaded(valid_exercise) => {
                    self.form_data = exercise_to_req(&valid_exercise);
                    self.current_detail = Some(valid_exercise);
                }
                ExercisesPageMsg::Saved => {
                    self.show_success("Exercise saved successfully");
                    if matches!(self.state, ExercisesPageState::CreateNew) {
                        self.state = ExercisesPageState::DetailsClosed;
                        self.trigger_list_refresh();
                    } else {
                        self.state = ExercisesPageState::DetailsClosed;
                        self.trigger_list_refresh();
                    }
                }
                ExercisesPageMsg::Deleted => {
                    self.show_success("Exercise deleted");
                    self.state = ExercisesPageState::DetailsClosed;
                    self.trigger_list_refresh();
                }
                ExercisesPageMsg::Error(e) => {
                    self.show_error(&e);
                }
            }
        }
    }

    fn show_error(&mut self, msg: &str) {
        self.error_message = Some((msg.to_string(), Instant::now()));
    }

    fn show_success(&mut self, msg: &str) {
        self.success_message = Some((msg.to_string(), Instant::now()));
    }

    fn trigger_list_refresh(&mut self) {
        self.initialized = false;
    }

    fn fetch_list(&mut self, pool: &Pool<Sqlite>) {
        if self.loading {
            return;
        }
        self.loading = true;

        let sender = self.sender.clone();
        let pool = pool.clone();
        let name_filter = if self.filter_name.is_empty() {
            None
        } else {
            Some(self.filter_name.clone())
        };
        let params = self.pagination_state.to_pagination_params();

        tokio::spawn(async move {
            let filter = ExerciseLibraryFilterReq {
                name: name_filter,
                push_or_pull: None,
                dynamic_or_static: None,
                straight_or_bent: None,
                squat_or_hinge: None,
                upper_or_lower: None,
                compound_or_isolation: None,
                lever_variation: None,
                grip: None,
                grip_width: None,
            };

            match paginate_exercises(&pool, Some(filter), params).await {
                Ok(res) => {
                    let _ = sender.send(ExercisesPageMsg::ListLoaded(res));
                }
                Err(e) => {
                    let _ = sender.send(ExercisesPageMsg::Error(e));
                }
            }
        });
    }

    fn fetch_detail(&mut self, pool: &Pool<Sqlite>, id: u32) {
        self.loading = true;
        let tx = self.sender.clone();
        let pool = pool.clone();

        tokio::spawn(async move {
            match get_one_exercise(&pool, id).await {
                Ok(e) => {
                    let _ = tx.send(ExercisesPageMsg::DetailLoaded(e));
                }
                Err(e) => {
                    let _ = tx.send(ExercisesPageMsg::Error(e));
                }
            }
        });
    }

    fn save_exercise(&mut self, pool: &Pool<Sqlite>) {
        self.loading = true;
        let sender = self.sender.clone();
        let pool = pool.clone();
        let req = self.form_data.clone();
        let is_edit = matches!(self.state, ExercisesPageState::DetailsEditView);

        let id_opt = self.current_detail.as_ref().map(|d| get_exercise_id(d));

        tokio::spawn(async move {
            let mut conn = match pool.begin().await {
                Ok(c) => c,
                Err(e) => {
                    let _ = sender.send(ExercisesPageMsg::Error(e.to_string()));
                    return;
                }
            };

            let res = if is_edit {
                if let Some(id) = id_opt {
                    match ExerciseLibraryEntity::from_req(req) {
                        Ok(mut entity) => {
                            entity.id = id; // Set the ID
                            match entity.to_valid_struct() {
                                Ok(valid) => match update_exercise(&mut conn, valid).await {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e),
                                },
                                Err(e) => Err(e),
                            }
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Err("No ID found for edit".to_string())
                }
            } else {
                create_exercise(&mut conn, req).await.map(|_| ())
            };

            match res {
                Ok(_) => {
                    if let Err(e) = conn.commit().await {
                        let _ =
                            sender.send(ExercisesPageMsg::Error(format!("Commit failed: {}", e)));
                    } else {
                        let _ = sender.send(ExercisesPageMsg::Saved);
                    }
                }
                Err(e) => {
                    let _ = sender.send(ExercisesPageMsg::Error(e));
                }
            }
        });
    }

    fn delete_current(&mut self, pool: &Pool<Sqlite>) {
        let id = if let Some(d) = &self.current_detail {
            get_exercise_id(d)
        } else {
            return;
        };
        self.loading = true;
        let sender = self.sender.clone();
        let pool = pool.clone();

        tokio::spawn(async move {
            let mut conn = match pool.begin().await {
                Ok(c) => c,
                Err(e) => {
                    let _ = sender.send(ExercisesPageMsg::Error(e.to_string()));
                    return;
                }
            };

            match delete_exercise(&mut conn, id).await {
                Ok(_) => {
                    if let Err(e) = conn.commit().await {
                        let _ =
                            sender.send(ExercisesPageMsg::Error(format!("Commit failed: {}", e)));
                    } else {
                        let _ = sender.send(ExercisesPageMsg::Deleted);
                    }
                }
                Err(e) => {
                    let _ = sender.send(ExercisesPageMsg::Error(e));
                }
            }
        });
    }

    fn render_details_open_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("Exercise Details");
        ui.separator();

        if let Some(detail) = &self.current_detail {
            egui::Grid::new("details_grid")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Name:");
                    ui.label(get_exercise_name(detail));
                    ui.end_row();

                    // Render other fields based on enum variants
                    // Since ValidExercise varies, we inspect the enum
                    match detail {
                        ValidExercise::StraightArmCompound(e) => {
                            ui.label("Type:");
                            ui.label("Straight Arm Compound");
                            ui.end_row();
                            ui.label("Push/Pull:");
                            ui.label(format!("{:?}", e.push_or_pull));
                            ui.end_row();
                            ui.label("Static/Dynamic:");
                            ui.label(format!("{:?}", e.dynamic_or_static));
                            ui.end_row();
                            ui.label("Lever:");
                            ui.label(format!("{:?}", e.lever_variation));
                            ui.end_row();
                            ui.label("Grip:");
                            ui.label(format!("{:?}", e.grip));
                            ui.end_row();
                            ui.label("Width:");
                            ui.label(format!("{:?}", e.grip_width));
                            ui.end_row();
                        }
                        ValidExercise::BentArmCompound(e) => {
                            ui.label("Type:");
                            ui.label("Bent Arm Compound");
                            ui.end_row();
                            ui.label("Push/Pull:");
                            ui.label(format!("{:?}", e.push_or_pull));
                            ui.end_row();
                            ui.label("Static/Dynamic:");
                            ui.label(format!("{:?}", e.dynamic_or_static));
                            ui.end_row();
                            if let Some(l) = &e.lever_variation {
                                ui.label("Lever:");
                                ui.label(format!("{:?}", l));
                                ui.end_row();
                            }
                            ui.label("Grip:");
                            ui.label(format!("{:?}", e.grip));
                            ui.end_row();
                            ui.label("Width:");
                            ui.label(format!("{:?}", e.grip_width));
                            ui.end_row();
                        }
                        ValidExercise::UpperBodyIsolation(e) => {
                            ui.label("Type:");
                            ui.label("Upper Body Isolation");
                            ui.end_row();
                            ui.label("Static/Dynamic:");
                            ui.label(format!("{:?}", e.dynamic_or_static));
                            ui.end_row();
                            ui.label("Arm:");
                            ui.label(format!("{:?}", e.straight_or_bent));
                            ui.end_row();
                        }
                        ValidExercise::LowerBodyCompound(e) => {
                            ui.label("Type:");
                            ui.label("Lower Body Compound");
                            ui.end_row();
                            ui.label("Static/Dynamic:");
                            ui.label(format!("{:?}", e.dynamic_or_static));
                            ui.end_row();
                            ui.label("Squat/Hinge:");
                            ui.label(format!("{:?}", e.squat_or_hinge));
                            ui.end_row();
                        }
                        ValidExercise::LowerBodyIsolation(e) => {
                            ui.label("Type:");
                            ui.label("Lower Body Isolation");
                            ui.end_row();
                            ui.label("Static/Dynamic:");
                            ui.label(format!("{:?}", e.dynamic_or_static));
                            ui.end_row();
                        }
                    }
                });
        } else {
            ui.label("No exercise selected.");
        }

        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.button("Close").clicked() {
                self.state = ExercisesPageState::DetailsClosed;
            }
            if ui.button("Edit").clicked() {
                if let Some(d) = &self.current_detail {
                    self.form_data = exercise_to_req(d);
                    self.state = ExercisesPageState::DetailsEditView;
                }
            }
        });
    }

    fn render_form(&mut self, ui: &mut egui::Ui) {
        let req = &mut self.form_data;

        ui.label("Name");
        ui.text_edit_singleline(&mut req.name);

        ui.label("Upper/Lower Body");
        egui::ComboBox::from_id_salt("upper_lower")
            .selected_text(format!("{:?}", req.upper_or_lower))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut req.upper_or_lower, UpperOrLower::Upper, "Upper");
                ui.selectable_value(&mut req.upper_or_lower, UpperOrLower::Lower, "Lower");
            });

        ui.label("Compound/Isolation");
        egui::ComboBox::from_id_salt("comp_iso")
            .selected_text(format!("{:?}", req.compound_or_isolation))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut req.compound_or_isolation,
                    CompoundOrIsolation::Compound,
                    "Compound",
                );
                ui.selectable_value(
                    &mut req.compound_or_isolation,
                    CompoundOrIsolation::Isolation,
                    "Isolation",
                );
            });

        ui.label("Dynamic/Static");
        egui::ComboBox::from_id_salt("dyn_stat")
            .selected_text(format!("{:?}", req.dynamic_or_static))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut req.dynamic_or_static,
                    DynamicOrStatic::Dynamic,
                    "Dynamic",
                );
                ui.selectable_value(
                    &mut req.dynamic_or_static,
                    DynamicOrStatic::Static,
                    "Static",
                );
            });

        // Contextual Fields
        let is_upper = req.upper_or_lower == UpperOrLower::Upper;
        let is_compound = req.compound_or_isolation == CompoundOrIsolation::Compound;

        if is_upper {
            if is_compound {
                ui.label("Push/Pull");
                utils::combo_opt(
                    ui,
                    "push_pull",
                    &mut req.push_or_pull,
                    vec![PushOrPull::Push, PushOrPull::Pull],
                );

                ui.label("Straight/Bent Arm");
                utils::combo_opt(
                    ui,
                    "str_bent",
                    &mut req.straight_or_bent,
                    vec![StraightOrBentArm::Straight, StraightOrBentArm::Bent],
                );

                ui.label("Grip");
                utils::combo_opt(
                    ui,
                    "grip",
                    &mut req.grip,
                    vec![
                        Grip::Pronated,
                        Grip::Supinated,
                        Grip::Neutral,
                        Grip::GymnasticsRing,
                        Grip::Floor,
                        Grip::Mixed,
                    ],
                );

                ui.label("Grip Width");
                utils::combo_opt(
                    ui,
                    "width",
                    &mut req.grip_width,
                    vec![GripWidth::Narrow, GripWidth::Shoulder, GripWidth::Wide],
                );

                ui.label("Lever Variation");
                utils::combo_opt(
                    ui,
                    "lever",
                    &mut req.lever_variation,
                    vec![
                        LeverVariation::Tuck,
                        LeverVariation::AdvancedTuck,
                        LeverVariation::Straddle,
                        LeverVariation::HalfLay,
                        LeverVariation::OneLeg,
                        LeverVariation::Full,
                    ],
                );
            } else {
                // Upper Isolation
                ui.label("Straight/Bent Arm");
                utils::combo_opt(
                    ui,
                    "str_bent",
                    &mut req.straight_or_bent,
                    vec![StraightOrBentArm::Straight, StraightOrBentArm::Bent],
                );
            }
        } else {
            // Lower
            if is_compound {
                ui.label("Squat/Hinge");
                utils::combo_opt(
                    ui,
                    "sq_hinge",
                    &mut req.squat_or_hinge,
                    vec![SquatOrHinge::Squat, SquatOrHinge::Hinge],
                );
            }
        }

        ui.label("Description");
        let mut desc = req.description.clone().unwrap_or_default();
        ui.text_edit_multiline(&mut desc);
        req.description = if desc.is_empty() { None } else { Some(desc) };
    }

    fn render_details_edit_view(&mut self, ui: &mut egui::Ui, pool: &mut Pool<Sqlite>) {
        ui.heading("Edit Exercise");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            self.render_form(ui);
        });

        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.button("Close").clicked() {
                self.state = ExercisesPageState::DetailsClosed;
            }
            if ui.button("Save").clicked() {
                self.save_exercise(pool);
            }
            if ui.button("Delete").clicked() {
                self.delete_current(pool);
            }
        });
    }

    fn render_create(&mut self, ui: &mut egui::Ui, pool: &mut Pool<Sqlite>) {
        ui.heading("Create New Exercise");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            self.render_form(ui);
        });

        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.button("Close").clicked() {
                self.state = ExercisesPageState::DetailsClosed;
            }
            if ui.button("Save").clicked() {
                self.save_exercise(pool);
            }
        });
    }

    fn render_list(&mut self, ui: &mut egui::Ui, pool: &mut Pool<Sqlite>) {
        ui.horizontal(|ui| {
            ui.heading("Exercises");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Add New").clicked() {
                    self.state = ExercisesPageState::CreateNew;
                    self.form_data = exercise_library_default_req();
                }
            });
        });
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Search:");
            if ui.text_edit_singleline(&mut self.filter_name).changed() {
                self.pagination_state.reset_pagination();
                self.trigger_list_refresh();
            }
        });

        ui.separator();

        enum ListAction {
            Details(u32),
            Edit(u32),
            Delete(u32),
        }

        let mut action = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("list_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Group");
                    ui.label("Type");
                    ui.label("Actions");
                    ui.end_row();

                    for item in &self.list_items {
                        ui.label(&item.name);
                        ui.label(format!("{:?}", item.upper_or_lower));
                        ui.label(format!("{:?}", item.compound_or_isolation));

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
                        });
                        ui.end_row();
                    }
                });
        });

        if let Some(act) = action {
            match act {
                ListAction::Details(id) => {
                    self.fetch_detail(pool, id);
                    self.state = ExercisesPageState::DetailsOpenView;
                }
                ListAction::Edit(id) => {
                    self.fetch_detail(pool, id);
                    self.state = ExercisesPageState::DetailsEditView;
                }
                ListAction::Delete(id) => {
                    self.loading = true;
                    let tx = self.sender.clone();
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        let mut conn = match pool.begin().await {
                            Ok(c) => c,
                            Err(e) => {
                                let _ = tx.send(ExercisesPageMsg::Error(e.to_string()));
                                return;
                            }
                        };
                        if let Err(e) = delete_exercise(&mut conn, id).await {
                            let _ = tx.send(ExercisesPageMsg::Error(e));
                        } else {
                            let _ = conn.commit().await;
                            let _ = tx.send(ExercisesPageMsg::Deleted);
                        }
                    });
                }
            }
        }

        ui.separator();

        ui.horizontal(|ui| {
            if self.pagination_state.has_previous() {
                if ui.button("Previous").clicked() {
                    self.pagination_state.go_backwards();
                    self.trigger_list_refresh();
                }
            }

            if self.pagination_state.has_next() {
                if ui.button("Next").clicked() {
                    self.pagination_state.go_forwards();
                    self.trigger_list_refresh();
                }
            }
        });
    }

    pub fn render_page(&mut self, pool: &mut Pool<Sqlite>, ui: &mut egui::Ui) {
        self.handle_async_messages();

        // Show Toasts
        if let Some((msg, time)) = &self.error_message {
            if time.elapsed() > Duration::from_secs(5) {
                self.error_message = None;
            } else {
                ui.colored_label(egui::Color32::RED, msg);
            }
        }
        if let Some((msg, time)) = &self.success_message {
            if time.elapsed() > Duration::from_secs(3) {
                self.success_message = None;
            } else {
                ui.colored_label(egui::Color32::GREEN, msg);
            }
        }

        if !self.initialized && matches!(self.state, ExercisesPageState::DetailsClosed) {
            self.fetch_list(pool);
            self.initialized = true;
        }

        match self.state {
            ExercisesPageState::DetailsClosed => self.render_list(ui, pool),
            ExercisesPageState::DetailsOpenView => self.render_details_open_view(ui),
            ExercisesPageState::DetailsEditView => self.render_details_edit_view(ui, pool),
            ExercisesPageState::CreateNew => self.render_create(ui, pool),
        }
    }
}

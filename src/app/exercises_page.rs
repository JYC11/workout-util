use crate::app::utils;
use crate::app::utils::{CommonUiState, filter_combo};
use crate::db::pagination_support::{PaginationRes, PaginationState};
use crate::workout::enums::{
    CompoundOrIsolation, DynamicOrStatic, Grip, GripWidth, LeverVariation, PushOrPull,
    SquatOrHinge, StraightOrBentArm, UpperOrLower,
};
use crate::workout::exercise::{
    ExerciseLibraryEntity, create_exercise, delete_exercise, get_one_exercise, paginate_exercises,
    update_exercise,
};
use crate::workout::exercise_dto::{
    ExerciseLibraryFilterReq, ExerciseLibraryReq, ExerciseLibraryRes, ExerciseName, ValidExercise,
    exercise_library_default_req, exercise_to_req, get_exercise_id, get_exercise_name,
};
use eframe::egui;
use sqlx::{Pool, Sqlite};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

pub struct ExercisesPage {
    pool: Pool<Sqlite>,
    state: ExercisesPageState,
    // Data
    list_items: Vec<ExerciseLibraryRes>,
    current_detail: Option<ValidExercise>,
    // Form State
    form_data: ExerciseLibraryReq,
    // Search/Filter State
    pagination_filters: ExerciseLibraryFilterReq,
    // Pagination State
    pagination_state: PaginationState,
    // Async Communication
    receiver: Receiver<ExercisesPageMsg>,
    sender: Sender<ExercisesPageMsg>,
    // UI Status
    common_ui_state: CommonUiState,
}

pub enum ExercisesPageMsg {
    ListLoaded(PaginationRes<ExerciseLibraryRes>),
    DetailLoaded(ValidExercise),
    Saved,
    Deleted,
    Error(String),
}

impl ExercisesPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        let (sender, receiver) = channel();
        Self {
            pool,
            state: ExercisesPageState::DetailsClosed,
            list_items: Vec::new(),
            current_detail: None,
            form_data: exercise_library_default_req(),
            pagination_filters: ExerciseLibraryFilterReq::default(),
            pagination_state: PaginationState::default(),
            receiver,
            sender,
            common_ui_state: CommonUiState::default(),
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
            self.common_ui_state.set_as_not_loading();
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
                    self.common_ui_state
                        .show_success("Exercise saved successfully");
                    if matches!(self.state, ExercisesPageState::CreateNew) {
                        self.state = ExercisesPageState::DetailsClosed;
                        self.trigger_list_refresh();
                    } else {
                        self.state = ExercisesPageState::DetailsClosed;
                        self.trigger_list_refresh();
                    }
                }
                ExercisesPageMsg::Deleted => {
                    self.common_ui_state.show_success("Exercise deleted");
                    self.state = ExercisesPageState::DetailsClosed;
                    self.trigger_list_refresh();
                }
                ExercisesPageMsg::Error(e) => {
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
        let pool = self.pool.clone();
        let filter = self.pagination_filters.clone();
        let params = self.pagination_state.to_pagination_params();

        let ctx = ctx.clone();

        tokio::spawn(async move {
            match paginate_exercises(&pool, Some(filter), params).await {
                Ok(res) => {
                    let _ = sender.send(ExercisesPageMsg::ListLoaded(res));
                }
                Err(e) => {
                    let _ = sender.send(ExercisesPageMsg::Error(e));
                }
            }
            ctx.request_repaint();
        });
    }

    fn fetch_detail(&mut self, ctx: &egui::Context, id: u32) {
        self.common_ui_state.set_as_loading();
        let sender = self.sender.clone();
        let pool = self.pool.clone();
        let ctx = ctx.clone();

        tokio::spawn(async move {
            match get_one_exercise(&pool, id).await {
                Ok(e) => {
                    let _ = sender.send(ExercisesPageMsg::DetailLoaded(e));
                }
                Err(e) => {
                    let _ = sender.send(ExercisesPageMsg::Error(e));
                }
            }
            ctx.request_repaint();
        });
    }

    fn save_exercise(&mut self, ctx: &egui::Context) {
        self.common_ui_state.set_as_loading();
        let sender = self.sender.clone();
        let pool = self.pool.clone();
        let req = self.form_data.clone();
        let is_edit = matches!(self.state, ExercisesPageState::DetailsEditView);
        let ctx = ctx.clone();

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
            ctx.request_repaint();
        });
    }

    fn delete_current(&mut self, ctx: &egui::Context) {
        let id = if let Some(d) = &self.current_detail {
            get_exercise_id(d)
        } else {
            return;
        };
        self.common_ui_state.set_as_loading();
        let sender = self.sender.clone();
        let pool = self.pool.clone();
        let ctx = ctx.clone();

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
            ctx.request_repaint();
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

    fn render_details_edit_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
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
                self.save_exercise(ctx);
            }
            if ui.button("Delete").clicked() {
                self.delete_current(ctx);
            }
        });
    }

    fn render_create(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
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
                self.save_exercise(ctx);
            }
        });
    }

    fn render_filters(&mut self, ui: &mut egui::Ui) {
        let mut changed = false;

        ui.collapsing("Filters", |ui| {
            egui::Grid::new("filters_grid")
                .num_columns(4)
                .striped(true)
                .show(ui, |ui| {
                    changed |= filter_combo(
                        ui,
                        "Upper/Lower",
                        &mut self.pagination_filters.upper_or_lower,
                        &[UpperOrLower::Upper, UpperOrLower::Lower],
                    );
                    changed |= filter_combo(
                        ui,
                        "Compound/Isolation",
                        &mut self.pagination_filters.compound_or_isolation,
                        &[
                            CompoundOrIsolation::Compound,
                            CompoundOrIsolation::Isolation,
                        ],
                    );
                    ui.end_row();

                    changed |= filter_combo(
                        ui,
                        "Push/Pull",
                        &mut self.pagination_filters.push_or_pull,
                        &[PushOrPull::Push, PushOrPull::Pull],
                    );
                    changed |= filter_combo(
                        ui,
                        "Dynamic/Static",
                        &mut self.pagination_filters.dynamic_or_static,
                        &[DynamicOrStatic::Dynamic, DynamicOrStatic::Static],
                    );
                    ui.end_row();

                    changed |= filter_combo(
                        ui,
                        "Straight/Bent",
                        &mut self.pagination_filters.straight_or_bent,
                        &[StraightOrBentArm::Straight, StraightOrBentArm::Bent],
                    );
                    changed |= filter_combo(
                        ui,
                        "Squat/Hinge",
                        &mut self.pagination_filters.squat_or_hinge,
                        &[SquatOrHinge::Squat, SquatOrHinge::Hinge],
                    );
                    ui.end_row();

                    changed |= filter_combo(
                        ui,
                        "Grip",
                        &mut self.pagination_filters.grip,
                        &[
                            Grip::Pronated,
                            Grip::Supinated,
                            Grip::Neutral,
                            Grip::GymnasticsRing,
                            Grip::Floor,
                            Grip::Mixed,
                        ],
                    );
                    changed |= filter_combo(
                        ui,
                        "Grip Width",
                        &mut self.pagination_filters.grip_width,
                        &[GripWidth::Narrow, GripWidth::Shoulder, GripWidth::Wide],
                    );
                    ui.end_row();

                    changed |= filter_combo(
                        ui,
                        "Lever",
                        &mut self.pagination_filters.lever_variation,
                        &[
                            LeverVariation::Tuck,
                            LeverVariation::AdvancedTuck,
                            LeverVariation::Straddle,
                            LeverVariation::HalfLay,
                            LeverVariation::OneLeg,
                            LeverVariation::Full,
                        ],
                    );
                    ui.end_row();
                });
        });

        if changed {
            self.pagination_state.reset_pagination();
            self.trigger_list_refresh();
        }
    }

    fn render_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.horizontal(|ui| {
                ui.label("Limit:");
                let mut limit = self.pagination_state.limit;
                if ui
                    .add(egui::DragValue::new(&mut limit).speed(1.0).range(1..=100))
                    .changed()
                {
                    self.pagination_state.limit = limit;
                    self.trigger_list_refresh();
                }

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
            ui.separator();

            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
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

                let mut name = if let Some(name) = &self.pagination_filters.name {
                    name.clone()
                } else {
                    "".to_string()
                };

                ui.horizontal(|ui| {
                    ui.label("Search:");
                    if ui.text_edit_singleline(&mut name).changed() {
                        self.pagination_filters.name = Some(name.clone());
                        self.pagination_state.reset_pagination();
                        self.trigger_list_refresh();
                    }
                });

                self.render_filters(ui);

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
                                ui.label(&item.full_name());
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
                            self.fetch_detail(ctx, id);
                            self.state = ExercisesPageState::DetailsOpenView;
                        }
                        ListAction::Edit(id) => {
                            self.fetch_detail(ctx, id);
                            self.state = ExercisesPageState::DetailsEditView;
                        }
                        ListAction::Delete(id) => {
                            self.common_ui_state.set_as_loading();
                            let sender = self.sender.clone();
                            let pool = self.pool.clone();
                            tokio::spawn(async move {
                                let mut conn = match pool.begin().await {
                                    Ok(c) => c,
                                    Err(e) => {
                                        let _ = sender.send(ExercisesPageMsg::Error(e.to_string()));
                                        return;
                                    }
                                };
                                if let Err(e) = delete_exercise(&mut conn, id).await {
                                    let _ = sender.send(ExercisesPageMsg::Error(e));
                                } else {
                                    let _ = conn.commit().await;
                                    let _ = sender.send(ExercisesPageMsg::Deleted);
                                }
                            });
                        }
                    }
                }
            });
        });
    }

    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.handle_async_messages();

        // Show Toasts
        if let Some((msg, time)) = &self.common_ui_state.error_message {
            if time.elapsed() > Duration::from_secs(5) {
                self.common_ui_state.clear_error();
            } else {
                ui.colored_label(egui::Color32::RED, msg);
            }
        }
        if let Some((msg, time)) = &self.common_ui_state.success_message {
            if time.elapsed() > Duration::from_secs(3) {
                self.common_ui_state.clear_success();
            } else {
                ui.colored_label(egui::Color32::GREEN, msg);
            }
        }

        if !self.common_ui_state.initialized
            && matches!(self.state, ExercisesPageState::DetailsClosed)
        {
            self.fetch_list(ctx);
            self.common_ui_state.initialize();
        }

        match self.state {
            ExercisesPageState::DetailsClosed => self.render_list(ctx, ui),
            ExercisesPageState::DetailsOpenView => self.render_details_open_view(ui),
            ExercisesPageState::DetailsEditView => self.render_details_edit_view(ctx, ui),
            ExercisesPageState::CreateNew => self.render_create(ctx, ui),
        }
    }
}

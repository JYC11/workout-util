use crate::client::app_utils::CommonUiState;
use crate::db::pagination_support::{PaginationRes, PaginationState};
use crate::workout_log::workout_log_dto::{
    WorkoutLogDetailRes, WorkoutLogGroupFilterReq, WorkoutLogGroupPageRes, WorkoutLogGroupRes,
};
use crate::workout_log::workout_log_service::WorkoutLogService;
use eframe::egui;
use sqlx::{Pool, Sqlite};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct WorkoutLogsPage {
    service: WorkoutLogService,
    state: WorkoutLogsPageState,
    // Data
    list_items: Vec<WorkoutLogGroupPageRes>,
    current_log_group: Option<WorkoutLogGroupRes>,
    current_logs: Vec<WorkoutLogDetailRes>,
    // Async Communication
    receiver: Receiver<WorkoutLogsPageMsg>,
    sender: Sender<WorkoutLogsPageMsg>,
    // Search/Filter State
    pagination_filters: WorkoutLogGroupFilterReq,
    // Filter UI state (strings for text input)
    filter_date_gte_str: String,
    filter_date_lte_str: String,
    filter_notes_str: String,
    // Pagination State
    pagination_state: PaginationState,
    // UI Status
    common_ui_state: CommonUiState,
}

pub enum WorkoutLogsPageMsg {
    ListLoaded(PaginationRes<WorkoutLogGroupPageRes>),
    DetailLoaded(WorkoutLogGroupRes, Vec<WorkoutLogDetailRes>),
    Error(String),
}

impl WorkoutLogsPage {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        let (sender, receiver) = channel();
        Self {
            service: WorkoutLogService::new(pool.clone()),
            state: WorkoutLogsPageState::DetailsClosed,
            list_items: Vec::new(),
            current_log_group: None,
            current_logs: Vec::new(),
            pagination_filters: WorkoutLogGroupFilterReq::default(),
            filter_date_gte_str: String::new(),
            filter_date_lte_str: String::new(),
            filter_notes_str: String::new(),
            pagination_state: PaginationState::default(),
            receiver,
            sender,
            common_ui_state: CommonUiState::default(),
        }
    }
}

pub enum WorkoutLogsPageState {
    DetailsClosed,
    DetailsOpenView,
}

impl WorkoutLogsPage {
    fn handle_async_messages(&mut self) {
        while let Ok(msg) = self.receiver.try_recv() {
            match msg {
                WorkoutLogsPageMsg::ListLoaded(res) => {
                    self.list_items = res.items;
                    self.pagination_state.next_cursor = res.next_cursor;
                    self.pagination_state.prev_cursor = res.prev_cursor;
                    self.common_ui_state.set_as_not_loading();
                }
                WorkoutLogsPageMsg::DetailLoaded(log_group, logs) => {
                    self.current_log_group = Some(log_group);
                    self.current_logs = logs;
                    self.state = WorkoutLogsPageState::DetailsOpenView;
                    self.common_ui_state.set_as_not_loading();
                }
                WorkoutLogsPageMsg::Error(err) => {
                    self.common_ui_state.show_error(&err);
                    self.common_ui_state.set_as_not_loading();
                }
            }
        }
    }

    fn trigger_list_refresh(&mut self) {
        self.common_ui_state.not_initialized();
    }

    fn fetch_list(&mut self, ctx: &egui::Context) {
        if self.common_ui_state.initialized || self.common_ui_state.loading {
            return;
        }
        self.common_ui_state.set_as_loading();
        self.common_ui_state.initialize();

        let service = self.service.clone();
        let sender = self.sender.clone();
        let filters = Some(self.pagination_filters.clone());
        let params = self.pagination_state.to_pagination_params();
        let ctx = ctx.clone();

        tokio::spawn(async move {
            let result = service.paginate_log_groups(filters, params).await;
            match result {
                Ok(res) => {
                    let _ = sender.send(WorkoutLogsPageMsg::ListLoaded(res));
                }
                Err(e) => {
                    let _ = sender.send(WorkoutLogsPageMsg::Error(e));
                }
            }
            ctx.request_repaint();
        });
    }

    fn fetch_detail(&mut self, ctx: &egui::Context, id: u32) {
        self.common_ui_state.set_as_loading();

        let service = self.service.clone();
        let sender = self.sender.clone();
        let ctx = ctx.clone();

        tokio::spawn(async move {
            let log_group_result = service.get_log_group(id).await;
            let logs_result = service.get_logs_by_workout_log_group_id(id).await;

            match (log_group_result, logs_result) {
                (Ok(log_group), Ok(logs)) => {
                    let _ = sender.send(WorkoutLogsPageMsg::DetailLoaded(log_group, logs));
                }
                (Err(e), _) | (_, Err(e)) => {
                    let _ = sender.send(WorkoutLogsPageMsg::Error(e));
                }
            }
            ctx.request_repaint();
        });
    }

    fn render_filters(&mut self, ui: &mut egui::Ui) {
        let mut filters_changed = false;

        ui.horizontal(|ui| {
            ui.label("Date From:");
            if ui
                .text_edit_singleline(&mut self.filter_date_gte_str)
                .changed()
            {
                self.pagination_filters.workout_date_gte =
                    chrono::NaiveDate::parse_from_str(&self.filter_date_gte_str, "%Y-%m-%d").ok();
                filters_changed = true;
            }

            ui.label("Date To:");
            if ui
                .text_edit_singleline(&mut self.filter_date_lte_str)
                .changed()
            {
                self.pagination_filters.workout_date_lte =
                    chrono::NaiveDate::parse_from_str(&self.filter_date_lte_str, "%Y-%m-%d").ok();
                filters_changed = true;
            }

            ui.label("Notes:");
            if ui
                .text_edit_singleline(&mut self.filter_notes_str)
                .changed()
            {
                self.pagination_filters.notes = if self.filter_notes_str.is_empty() {
                    None
                } else {
                    Some(self.filter_notes_str.clone())
                };
                filters_changed = true;
            }

            if ui.button("Clear Filters").clicked() {
                self.filter_date_gte_str.clear();
                self.filter_date_lte_str.clear();
                self.filter_notes_str.clear();
                self.pagination_filters = WorkoutLogGroupFilterReq::default();
                filters_changed = true;
            }
        });

        if filters_changed {
            self.pagination_state.current_cursor = None;
            self.trigger_list_refresh();
        }
    }

    fn render_pagination(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut limit = self.pagination_state.limit;
            if ui
                .add(egui::DragValue::new(&mut limit).speed(1.0).range(1..=100))
                .changed()
            {
                self.pagination_state.limit = limit;
                self.trigger_list_refresh();
            }

            if self.pagination_state.has_previous() {
                if ui.button("← Previous").clicked() {
                    self.pagination_state.go_backwards();
                    self.trigger_list_refresh();
                }
            }

            if self.pagination_state.has_next() {
                if ui.button("Next →").clicked() {
                    self.pagination_state.go_forwards();
                    self.trigger_list_refresh();
                }
            }
        });
    }

    fn render_details_open_view(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.state = WorkoutLogsPageState::DetailsClosed;
            self.current_log_group = None;
            self.current_logs.clear();
            return;
        }

        ui.separator();

        if let Some(log_group) = &self.current_log_group {
            // Header info
            ui.heading(format!("Workout Log - {}", log_group.date));
            if let Some(notes) = &log_group.notes {
                ui.label(format!("Notes: {}", notes));
            }
            ui.separator();

            // Group logs by exercise name
            let mut grouped_logs: std::collections::BTreeMap<String, Vec<&WorkoutLogDetailRes>> =
                std::collections::BTreeMap::new();
            for log in &self.current_logs {
                grouped_logs
                    .entry(log.workout_exercise_name.clone())
                    .or_default()
                    .push(log);
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (exercise_name, logs) in grouped_logs {
                    ui.collapsing(&exercise_name, |ui| {
                        egui::Grid::new(format!("log_grid_{}", exercise_name))
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Set");
                                ui.label("Reps/Secs");
                                ui.label("Weight");
                                ui.label("Description");
                                ui.end_row();

                                for log in logs {
                                    ui.label(format!("{}", log.set_number));
                                    ui.label(format!("{}", log.rep_number_or_seconds));
                                    ui.label(format!("{} lbs", log.weight));
                                    ui.label(log.description.as_deref().unwrap_or("-"));
                                    ui.end_row();
                                }
                            });
                    });
                }
            });
        }
    }

    fn render_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            self.render_pagination(ui);
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Workout Logs");
                });
                self.render_filters(ui);
                ui.separator();

                if self.common_ui_state.loading {
                    ui.spinner();
                    return;
                }

                self.common_ui_state.show_toasts(ui);

                if self.list_items.is_empty() {
                    ui.label("No workout logs found.");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("workout_logs_grid")
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Date");
                                ui.label("Notes");
                                ui.label("Actions");
                                ui.end_row();

                                let items_clone: Vec<_> = self
                                    .list_items
                                    .iter()
                                    .map(|i| (i.id, i.date, i.notes.clone()))
                                    .collect();
                                for (id, date, notes) in items_clone {
                                    ui.label(format!("{}", date));
                                    ui.label(notes.as_deref().unwrap_or("-"));
                                    if ui.button("View").clicked() {
                                        self.fetch_detail(ctx, id);
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                }
                ui.separator();
            });
        });
    }
    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.handle_async_messages();

        match self.state {
            WorkoutLogsPageState::DetailsClosed => {
                self.fetch_list(ctx);
                self.render_list(ctx, ui);
            }
            WorkoutLogsPageState::DetailsOpenView => {
                self.render_details_open_view(ui);
            }
        }
    }
}

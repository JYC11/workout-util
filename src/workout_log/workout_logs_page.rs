use crate::client::app_utils::CommonUiState;
use crate::db::pagination_support::PaginationState;
use crate::workout_log::workout_log_dto::{
    WorkoutLogDetailRes, WorkoutLogGroupFilterReq, WorkoutLogGroupPageRes, WorkoutLogGroupRes,
    WorkoutLogRes,
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
    // Pagination State
    pagination_state: PaginationState,
    // UI Status
    common_ui_state: CommonUiState,
}

pub enum WorkoutLogsPageMsg {
    ListLoaded(Vec<WorkoutLogGroupPageRes>),
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
        todo!("handle async messages");
    }

    fn trigger_list_refresh(&mut self) {
        self.common_ui_state.not_initialized();
    }

    fn fetch_list(&mut self, ctx: &egui::Context) {
        todo!("fetch list");
    }

    fn fetch_detail(&mut self, id: u32) {
        todo!("fetch detail");
    }

    fn render_details_open_view(&mut self, ui: &mut egui::Ui) {
        todo!("render details open view");
    }

    fn render_list(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        todo!("render list");
    }

    pub fn render_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        todo!("render start workout page");
    }
}

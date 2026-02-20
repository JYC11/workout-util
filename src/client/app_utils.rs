use crate::db::pagination_support::PaginationState;
use eframe::egui;
use std::time::{Duration, Instant};

pub struct CommonUiState {
    pub error_message: Option<(String, Instant)>,
    pub success_message: Option<(String, Instant)>,
    pub loading: bool,
    pub initialized: bool,
}

impl Default for CommonUiState {
    fn default() -> Self {
        Self {
            error_message: None,
            success_message: None,
            loading: false,
            initialized: false,
        }
    }
}

impl CommonUiState {
    pub fn set_as_loading(&mut self) {
        self.loading = true;
    }

    pub fn set_as_not_loading(&mut self) {
        self.loading = false;
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn clear_success(&mut self) {
        self.success_message = None;
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn not_initialized(&mut self) {
        self.initialized = false
    }

    pub fn show_error(&mut self, msg: &str) {
        self.error_message = Some((msg.to_string(), Instant::now()));
    }

    pub fn show_success(&mut self, msg: &str) {
        self.success_message = Some((msg.to_string(), Instant::now()));
    }

    pub fn show_toasts(&mut self, ui: &mut egui::Ui) {
        if let Some((msg, time)) = &self.error_message {
            if time.elapsed() > Duration::from_secs(5) {
                self.clear_error();
            } else {
                ui.colored_label(egui::Color32::RED, msg);
            }
        }
        if let Some((msg, time)) = &self.success_message {
            if time.elapsed() > Duration::from_secs(3) {
                self.clear_success();
            } else {
                ui.colored_label(egui::Color32::GREEN, msg);
            }
        }
    }
}

pub fn combo_opt<T: std::fmt::Debug + PartialEq + Copy>(
    ui: &mut egui::Ui,
    salt: &str,
    current: &mut Option<T>,
    options: Vec<T>,
) {
    let text = if let Some(val) = current {
        format!("{:?}", val)
    } else {
        "Select...".to_string()
    };
    egui::ComboBox::from_id_salt(salt)
        .selected_text(text)
        .show_ui(ui, |ui| {
            for opt in options {
                ui.selectable_value(current, Some(opt), format!("{:?}", opt));
            }
        });
}

pub fn filter_combo<T: Copy + PartialEq + std::fmt::Debug + 'static>(
    ui: &mut egui::Ui,
    label: &str,
    filter: &mut Option<Vec<T>>,
    options: &[T],
) -> bool {
    let mut current = filter.as_ref().and_then(|v| v.first().cloned());
    let original = current;

    ui.label(label);
    egui::ComboBox::from_id_salt(label)
        .selected_text(if let Some(v) = current {
            format!("{:?}", v)
        } else {
            "Any".to_string()
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut current, None, "Any");
            for &opt in options {
                ui.selectable_value(&mut current, Some(opt), format!("{:?}", opt));
            }
        });

    if current != original {
        *filter = current.map(|v| vec![v]);
        true
    } else {
        false
    }
}

pub fn render_pagination(ui: &mut egui::Ui, pagination_state: &mut PaginationState) -> bool {
    let mut refresh_needed = false;

    ui.horizontal(|ui| {
        ui.label("Limit:");
        let mut limit = pagination_state.limit;
        if ui
            .add(egui::DragValue::new(&mut limit).speed(1.0).range(1..=100))
            .changed()
        {
            pagination_state.limit = limit;
            pagination_state.reset_pagination();
            refresh_needed = true;
        }

        if pagination_state.has_previous() {
            if ui.button("Previous").clicked() {
                pagination_state.go_backwards();
                refresh_needed = true;
            }
        }

        if pagination_state.has_next() {
            if ui.button("Next").clicked() {
                pagination_state.go_forwards();
                refresh_needed = true;
            }
        }
    });

    refresh_needed
}

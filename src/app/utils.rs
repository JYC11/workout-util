use eframe::egui;
use std::time::Instant;

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

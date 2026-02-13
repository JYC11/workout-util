use eframe::egui;

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

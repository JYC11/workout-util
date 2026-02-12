use eframe::egui;
use sqlx::{Pool, Sqlite};

pub struct ExercisesPage {
    state: ExercisesPageState,
}

impl Default for ExercisesPage {
    fn default() -> Self {
        Self {
            state: ExercisesPageState::DetailsClosed,
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
    fn render_details_open_view(&mut self, ui: &mut egui::Ui) {
        // renders the exercise details here from db prepopulated with the selected exercise
        // the "close" button returns to the list view
        // the "edit" button converts to the edit details view
        todo!("render exercise details");
    }
    fn render_details_edit_view(&mut self, ui: &mut egui::Ui) {
        // renders the exercise details here from db prepopulated with the selected exercise
        // the "close" button returns to the list view
        // the "save" button converts to the edit details view, displays "saved" toast
        // the "delete" button deletes the exercise and returns to the list view
        // the options for enum selects should be drop downs with the enum values as labels
        todo!("render exercise details");
    }

    fn render_create(&mut self, ui: &mut egui::Ui) {
        // renders the create exercise form here without prepopulated values
        // the "close" button returns to the list view
        // the "save" button converts to the view details view, displays "saved" toast
        // the options for enum selects should be drop downs with the enum values as labels
        todo!("render create exercise");
    }

    fn render_list(&mut self, ui: &mut egui::Ui) {
        // the "add" button opens the details create view
        // the "edit" button opens the details edit view
        // the "delete" button deletes the exercise
        // the "details" button opens the details open view
        // contains a table with exercise name, description, and edit/delete buttons for each exercise
        // the table should be paginated and searchable by all fields with enums as drop downs
        // pagination should be keyset based, with the next/previous buttons at the bottom of the table
        todo!("render exercises list");
    }

    pub fn render_page(&mut self, pool: &mut Pool<Sqlite>, ui: &mut egui::Ui) {
        match self.state {
            ExercisesPageState::DetailsClosed => self.render_list(ui),
            ExercisesPageState::DetailsOpenView => self.render_details_open_view(ui),
            ExercisesPageState::DetailsEditView => self.render_details_edit_view(ui),
            ExercisesPageState::CreateNew => self.render_create(ui),
        }
    }
}

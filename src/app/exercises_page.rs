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
    DetailsOpen,
    CreateNew,
}

impl ExercisesPage {
    fn render_details_view(&mut self, ui: &mut egui::Ui) {
        // renders the exercise details here from db prepopulated with the selected exercise
        // the "close" button returns to the list view
        // the "save" button converts to the edit details view, displays "saved" toast
        todo!("render exercise details");
    }

    fn render_create(&mut self, ui: &mut egui::Ui) {
        // renders the create exercise form here without prepopulated values
        // the "close" button returns to the list view
        // the "save" button converts to the view details view, displays "saved" toast
        todo!("render create exercise");
    }

    fn render_list(&mut self, ui: &mut egui::Ui) {
        // the "add" button opens the details create view
        // the "edit" button opens the details edit view
        todo!("render exercises list");
    }

    pub fn render_page(&mut self, pool: &mut Pool<Sqlite>, ui: &mut egui::Ui) {
        match self.state {
            ExercisesPageState::DetailsClosed => self.render_list(ui),
            ExercisesPageState::DetailsOpen => self.render_details_view(ui),
            ExercisesPageState::CreateNew => self.render_create(ui),
        }
    }
}

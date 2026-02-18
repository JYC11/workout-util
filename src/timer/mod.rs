mod audio_engine;
pub mod countdown_timer;
pub mod emom_timer;
pub mod metronome;

pub trait Timer {
    fn toggle(&mut self);
    fn tick(&mut self);
}

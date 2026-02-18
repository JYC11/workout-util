use std::time::{Duration, Instant};
use crate::timer::audio_engine::AudioEngine;

// timer
pub struct Metronome {
    pub bpm: f64, // should be fixed to 60
    pub volume: f32,
    pub is_running: bool,
    pub last_tick: Option<Instant>,
    pub audio_engine: AudioEngine,
}

impl Metronome {
    pub fn new() -> Self {
        Self {
            bpm: 60.0,
            volume: 10.0,
            is_running: false,
            last_tick: None,
            audio_engine: AudioEngine::new(),
        }
    }

    pub fn toggle(&mut self) {
        self.is_running = !self.is_running;
        if self.is_running {
            self.last_tick = Some(Instant::now());
            self.audio_engine.play_sound(self.volume / 100.0);
        }
    }

    pub fn tick(&mut self) {
        if self.is_running {
            let interval = Duration::from_secs_f64(60.0 / self.bpm);
            if let Some(last) = self.last_tick {
                if last.elapsed() >= interval {
                    self.audio_engine.play_sound(self.volume);
                    self.last_tick = Some(Instant::now());
                }
            }
        }
    }
}

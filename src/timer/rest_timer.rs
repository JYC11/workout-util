use crate::timer::audio_engine::AudioEngine;
use std::time::{Duration, Instant};

pub struct RestTimer {
    pub input_minutes: u32,
    pub input_seconds: u32,
    pub current_seconds: u32,
    pub volume: f32,
    pub is_running: bool,
    pub last_tick: Option<Instant>,
    pub audio_engine: AudioEngine,
}

impl RestTimer {
    pub fn new() -> Self {
        Self {
            input_minutes: 0,
            input_seconds: 0,
            current_seconds: 0,
            volume: 10.0,
            is_running: false,
            last_tick: None,
            audio_engine: AudioEngine::new(),
        }
    }

    pub fn toggle(&mut self) {
        if self.is_running {
            self.is_running = false;
            self.last_tick = None;
        } else {
            if self.current_seconds == 0 {
                self.current_seconds = self.input_minutes * 60 + self.input_seconds;
            }

            if self.current_seconds > 0 {
                self.is_running = true;
                self.last_tick = Some(Instant::now());
            }
        }
    }

    pub fn tick(&mut self) {
        if self.is_running {
            if let Some(last) = self.last_tick {
                if last.elapsed() >= Duration::from_secs(1) {
                    if self.current_seconds > 0 {
                        self.current_seconds -= 1;
                        self.last_tick = Some(Instant::now());
                    }

                    if self.current_seconds == 0 {
                        self.is_running = false;
                        self.last_tick = None;
                        self.audio_engine.play_sound(self.volume);
                    }
                }
            }
        }
    }
}

use crate::timer::audio_engine::{AudioBackend, AudioEngine};
use std::time::{Duration, Instant};

// timer
pub struct Metronome {
    pub bpm: f64, // should be fixed to 60
    pub volume: f32,
    pub is_running: bool,
    pub last_tick: Option<Instant>,
    pub audio_engine: Box<dyn AudioBackend>,
}

impl Metronome {
    pub fn new() -> Self {
        Self {
            bpm: 60.0,
            volume: 10.0,
            is_running: false,
            last_tick: None,
            audio_engine: Box::new(AudioEngine::new()),
        }
    }

    #[cfg(test)]
    pub fn with_engine(audio_engine: Box<dyn AudioBackend>) -> Self {
        Self {
            bpm: 60.0,
            volume: 10.0,
            is_running: false,
            last_tick: None,
            audio_engine,
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

// src/timer/metronome.rs (test module)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer::audio_engine::{AudioCallRecorder, FakeAudioEngine};

    #[test]
    fn test_toggle_plays_sound() {
        let recorder = AudioCallRecorder::default();
        let fake_engine = Box::new(FakeAudioEngine::new(recorder.clone()));
        let mut metronome = Metronome::with_engine(fake_engine);

        assert_eq!(*recorder.play_count.lock().unwrap(), 0);

        metronome.toggle();

        assert_eq!(*recorder.play_count.lock().unwrap(), 1);
        assert_eq!(*recorder.last_volume.lock().unwrap(), Some(0.1)); // 10.0 / 100.0
    }

    #[test]
    fn test_tick_plays_sound_on_interval() {
        let recorder = AudioCallRecorder::default();
        let fake_engine = Box::new(FakeAudioEngine::new(recorder.clone()));
        let mut metronome = Metronome::with_engine(fake_engine);

        metronome.is_running = true;
        metronome.last_tick = Some(Instant::now());

        // Tick too soon
        metronome.tick();
        assert_eq!(*recorder.play_count.lock().unwrap(), 0);

        // Simulate time passed
        metronome.last_tick = Some(Instant::now() - Duration::from_secs(2));
        metronome.tick();

        assert_eq!(*recorder.play_count.lock().unwrap(), 1);
        assert_eq!(*recorder.last_volume.lock().unwrap(), Some(10.0));
    }
}
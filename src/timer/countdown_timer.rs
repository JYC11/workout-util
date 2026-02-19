use crate::timer::audio_engine::{AudioBackend, AudioEngine};
use crate::timer::Timer;
use std::time::{Duration, Instant};

pub struct CountDownTimer {
    pub input_minutes: u32,
    pub input_seconds: u32,
    pub current_seconds: u32,
    pub volume: f32,
    pub is_running: bool,
    pub last_tick: Option<Instant>,
    pub audio_engine: Box<dyn AudioBackend>,
}

impl CountDownTimer {
    pub fn new() -> Self {
        Self {
            input_minutes: 0,
            input_seconds: 0,
            current_seconds: 0,
            volume: 10.0,
            is_running: false,
            last_tick: None,
            audio_engine: Box::new(AudioEngine::new()),
        }
    }

    #[cfg(test)]
    pub fn with_engine(audio_engine: Box<dyn AudioBackend>) -> Self {
        Self {
            input_minutes: 0,
            input_seconds: 0,
            current_seconds: 0,
            volume: 10.0,
            is_running: false,
            last_tick: None,
            audio_engine,
        }
    }

    #[cfg(test)]
    pub fn advance_seconds(&mut self, seconds: u32) {
        if self.is_running {
            if self.current_seconds > seconds {
                self.current_seconds -= seconds;
                // prevent immediate double-tick if real tick is called
                self.last_tick = Some(Instant::now());
            } else {
                self.current_seconds = 0;
                self.is_running = false;
                self.last_tick = None;
                self.audio_engine.play_sound(self.volume);
            }
        }
    }

    #[cfg(test)]
    pub fn set_duration(&mut self, minutes: u32, seconds: u32) {
        self.input_minutes = minutes;
        self.input_seconds = seconds;
        self.current_seconds = 0;
        self.is_running = false;
        self.last_tick = None;
    }

    pub fn minutes_and_seconds(&self) -> (u32, u32) {
        (self.current_seconds / 60, self.current_seconds % 60)
    }
}

impl Timer for CountDownTimer {
    fn toggle(&mut self) {
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

    fn tick(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer::audio_engine::{AudioCallRecorder, FakeAudioEngine};

    #[test]
    fn test_new_timer_not_running() {
        let recorder = AudioCallRecorder::default();
        let timer = CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder)));

        assert!(!timer.is_running);
        assert_eq!(timer.current_seconds, 0);
    }

    #[test]
    fn test_toggle_starts_timer_with_duration() {
        let recorder = AudioCallRecorder::default();
        let mut timer = CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder)));
        timer.set_duration(0, 5);

        timer.toggle();

        assert!(timer.is_running);
        assert_eq!(timer.current_seconds, 5);
    }

    #[test]
    fn test_toggle_stops_timer() {
        let recorder = AudioCallRecorder::default();
        let mut timer = CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder)));
        timer.set_duration(0, 5);
        timer.toggle(); // Start
        timer.toggle(); // Stop

        assert!(!timer.is_running);
        assert!(timer.last_tick.is_none());
    }

    #[test]
    fn test_toggle_with_zero_duration_does_not_start() {
        let recorder = AudioCallRecorder::default();
        let mut timer = CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder)));
        timer.set_duration(0, 0);

        timer.toggle();

        assert!(!timer.is_running);
        assert_eq!(timer.current_seconds, 0);
    }

    #[test]
    fn test_tick_decrements_seconds() {
        let recorder = AudioCallRecorder::default();
        let mut timer = CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder)));
        timer.set_duration(0, 3);
        timer.toggle();
        timer.last_tick = Some(Instant::now() - Duration::from_secs(1));

        timer.tick();

        assert_eq!(timer.current_seconds, 2);
        assert!(timer.is_running);
    }

    #[test]
    fn test_tick_plays_sound_when_timer_completes() {
        let recorder = AudioCallRecorder::default();
        let mut timer =
            CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder.clone())));
        timer.set_duration(0, 1);
        timer.toggle();
        timer.last_tick = Some(Instant::now() - Duration::from_secs(2));

        timer.tick();

        assert_eq!(timer.current_seconds, 0);
        assert!(!timer.is_running);
        assert_eq!(*recorder.play_count.lock().unwrap(), 1);
        assert_eq!(*recorder.last_volume.lock().unwrap(), Some(10.0));
    }

    #[test]
    fn test_tick_does_not_play_sound_before_completion() {
        let recorder = AudioCallRecorder::default();
        let mut timer =
            CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder.clone())));
        timer.set_duration(0, 5);
        timer.toggle();
        timer.last_tick = Some(Instant::now() - Duration::from_secs(1));

        timer.tick();

        assert_eq!(timer.current_seconds, 4);
        assert!(timer.is_running);
        assert_eq!(*recorder.play_count.lock().unwrap(), 0);
    }

    #[test]
    fn test_tick_respects_one_second_interval() {
        let recorder = AudioCallRecorder::default();
        let mut timer =
            CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder.clone())));
        timer.set_duration(0, 5);
        timer.toggle();
        timer.last_tick = Some(Instant::now()); // Just now, not enough time elapsed

        timer.tick();

        assert_eq!(timer.current_seconds, 5); // Should not decrement yet
        assert_eq!(*recorder.play_count.lock().unwrap(), 0);
    }

    #[test]
    fn test_set_duration_resets_timer() {
        let recorder = AudioCallRecorder::default();
        let mut timer = CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder)));
        timer.set_duration(0, 5);
        timer.toggle();
        timer.set_duration(1, 30);

        assert_eq!(timer.input_minutes, 1);
        assert_eq!(timer.input_seconds, 30);
        assert_eq!(timer.current_seconds, 0);
        assert!(!timer.is_running);
    }

    #[test]
    fn test_multiple_ticks_until_completion() {
        let recorder = AudioCallRecorder::default();
        let mut timer =
            CountDownTimer::with_engine(Box::new(FakeAudioEngine::new(recorder.clone())));
        timer.set_duration(0, 3);
        timer.toggle();

        // Tick 3 times to complete
        for _ in 0..3 {
            timer.last_tick = Some(Instant::now() - Duration::from_secs(2));
            timer.tick();
        }

        assert_eq!(timer.current_seconds, 0);
        assert!(!timer.is_running);
        assert_eq!(*recorder.play_count.lock().unwrap(), 1);
    }
}

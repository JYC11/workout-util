#[cfg(test)]
use crate::timer::audio_engine::AudioBackend;
use crate::timer::countdown_timer::CountDownTimer;
use crate::timer::Timer;

pub struct EMOMTimer {
    pub current_round: u32,
    pub rounds: u32,
    pub is_running: bool,
    pub work_timer: CountDownTimer,
    pub rest_timer: CountDownTimer,
    pub is_work: bool,
}

impl EMOMTimer {
    pub fn new() -> Self {
        Self {
            current_round: 0,
            rounds: 0,
            is_running: false,
            work_timer: CountDownTimer::new(),
            rest_timer: CountDownTimer::new(),
            is_work: true,
        }
    }

    #[cfg(test)]
    pub fn with_engines(
        work_engine: Box<dyn AudioBackend>,
        rest_engine: Box<dyn AudioBackend>,
    ) -> Self {
        Self {
            current_round: 0,
            rounds: 0,
            is_running: false,
            work_timer: CountDownTimer::with_engine(work_engine),
            rest_timer: CountDownTimer::with_engine(rest_engine),
            is_work: true,
        }
    }

    fn handle_rest_finished(&mut self) {
        if self.current_round < self.rounds {
            self.current_round += 1;
            self.is_work = true;
            self.work_timer.toggle();
        } else {
            self.is_running = false;
            self.current_round = 0;
            self.is_work = true;
        }
    }
}

impl Timer for EMOMTimer {
    fn toggle(&mut self) {
        if self.is_running {
            self.is_running = false;
            if self.is_work {
                self.work_timer.toggle();
            } else {
                self.rest_timer.toggle();
            }
        } else {
            if self.rounds > 0 {
                self.is_running = true;
                if self.current_round == 0 {
                    self.current_round = 1;
                    self.is_work = true;
                    self.work_timer.toggle();
                } else {
                    if self.is_work {
                        self.work_timer.toggle();
                    } else {
                        self.rest_timer.toggle();
                    }
                }
            }
        }
    }

    fn tick(&mut self) {
        if !self.is_running {
            return;
        }

        if self.is_work {
            self.work_timer.tick();
            if !self.work_timer.is_running && self.work_timer.current_seconds == 0 {
                self.is_work = false;
                self.rest_timer.toggle();

                if !self.rest_timer.is_running {
                    self.handle_rest_finished();
                }
            }
        } else {
            self.rest_timer.tick();
            if !self.rest_timer.is_running && self.rest_timer.current_seconds == 0 {
                self.handle_rest_finished();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer::audio_engine::{AudioCallRecorder, FakeAudioEngine};

    #[test]
    fn test_emom_transitions() {
        let work_recorder = AudioCallRecorder::default();
        let rest_recorder = AudioCallRecorder::default();
        let mut timer = EMOMTimer::with_engines(
            Box::new(FakeAudioEngine::new(work_recorder.clone())),
            Box::new(FakeAudioEngine::new(rest_recorder.clone())),
        );

        timer.rounds = 2;
        timer.work_timer.set_duration(0, 5);
        timer.rest_timer.set_duration(0, 3);

        // Start Round 1 Work
        timer.toggle();
        assert!(timer.is_running);
        assert_eq!(timer.current_round, 1);
        assert!(timer.is_work);
        assert!(timer.work_timer.is_running);

        // Finish Work
        timer.work_timer.advance_seconds(5);
        timer.tick(); // Trigger transition

        // Check Transition to Rest
        assert!(timer.is_running);
        assert!(!timer.is_work);
        assert!(timer.rest_timer.is_running);
        assert_eq!(timer.rest_timer.current_seconds, 3);
        assert_eq!(*work_recorder.play_count.lock().unwrap(), 1);

        // Finish Rest
        timer.rest_timer.advance_seconds(3);
        timer.tick(); // Trigger transition

        // Check Transition to Round 2 Work
        assert!(timer.is_running);
        assert_eq!(timer.current_round, 2);
        assert!(timer.is_work);
        assert!(timer.work_timer.is_running);
        assert_eq!(*rest_recorder.play_count.lock().unwrap(), 1);

        // Finish Round 2 Work
        timer.work_timer.advance_seconds(5);
        timer.tick();

        // Round 2 Rest
        assert!(!timer.is_work);
        assert!(timer.rest_timer.is_running);
        assert_eq!(*work_recorder.play_count.lock().unwrap(), 2);

        // Finish Round 2 Rest (End of EMOM)
        timer.rest_timer.advance_seconds(3);
        timer.tick();

        assert!(!timer.is_running);
        assert_eq!(*rest_recorder.play_count.lock().unwrap(), 2);
        assert_eq!(timer.current_round, 0);
    }

    #[test]
    fn test_zero_rest_duration() {
        let work_recorder = AudioCallRecorder::default();
        let rest_recorder = AudioCallRecorder::default();
        let mut timer = EMOMTimer::with_engines(
            Box::new(FakeAudioEngine::new(work_recorder.clone())),
            Box::new(FakeAudioEngine::new(rest_recorder.clone())),
        );

        timer.rounds = 2;
        timer.work_timer.set_duration(0, 5);
        timer.rest_timer.set_duration(0, 0);

        timer.toggle();

        // Finish Work
        timer.work_timer.advance_seconds(5);
        timer.tick();

        // Should have skipped rest and gone to round 2 work immediately
        assert!(timer.is_running);
        assert_eq!(timer.current_round, 2);
        assert!(timer.is_work);
        assert!(timer.work_timer.is_running);
    }
}

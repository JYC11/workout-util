use rodio::source::SineWave;
use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub trait AudioBackend {
    fn play_sound(&mut self, volume: f32);
}

pub struct AudioEngine {
    _stream_handler: OutputStream,
    sink: Sink,
}

impl AudioEngine {
    pub fn new() -> Self {
        let stream_handler =
            OutputStreamBuilder::open_default_stream().expect("Failed to open stream");
        let sink = Sink::connect_new(&stream_handler.mixer());
        Self {
            _stream_handler: stream_handler,
            sink,
        }
    }
}

impl AudioBackend for AudioEngine {
    fn play_sound(&mut self, volume: f32) {
        self.sink.append(
            SineWave::new(880.0)
                .take_duration(Duration::from_millis(100))
                .amplify(volume),
        );
    }
}

#[cfg(test)]
#[derive(Clone, Default)]
pub struct AudioCallRecorder {
    pub play_count: Arc<Mutex<u32>>,
    pub last_volume: Arc<Mutex<Option<f32>>>,
}

#[cfg(test)]
pub struct FakeAudioEngine {
    recorder: AudioCallRecorder,
}

#[cfg(test)]
impl FakeAudioEngine {
    pub fn new(recorder: AudioCallRecorder) -> Self {
        Self { recorder }
    }
}

#[cfg(test)]
impl AudioBackend for FakeAudioEngine {
    fn play_sound(&mut self, volume: f32) {
        *self.recorder.play_count.lock().unwrap() += 1;
        *self.recorder.last_volume.lock().unwrap() = Some(volume);
    }
}

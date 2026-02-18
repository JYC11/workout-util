use rodio::source::{SineWave, TakeDuration};
use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};
use std::time::Duration;

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

    pub fn play_sound(&mut self, volume: f32) {
        self.sink.append(
            SineWave::new(880.0)
                .take_duration(Duration::from_millis(100))
                .amplify(volume)
        );
    }
}

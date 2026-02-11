// timer
pub struct Metronome {
    pub bpm: u32, // should be fixed to 60
}

pub struct RestTimer {
    pub seconds: u32,
}

pub struct EMOMTimer {
    seconds: u32,
    rounds: u32,
    pub rest_period: u32,
}

use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::Sdl;

use super::commons::CanTick;

/// Audio manager
pub struct Audio {
    time: u8,
    ticks: u32,
    freq: u32,
    device: AudioQueue<i16>,
}

impl Audio {
    /// Create a new audio managaer from an SDL context.
    pub fn new(sdl: &Sdl, freq: u32) -> Result<Audio, String> {
        let subsystem = sdl.audio()?;
        let spec = AudioSpecDesired {
            freq: Some(48_000),
            channels: Some(2),
            samples: None, // default sample size
        };
        let device = subsystem.open_queue::<i16, _>(None, &spec)?;
        Ok(Audio {
            time: 0,
            ticks: 0,
            freq,
            device,
        })
    }

    /// Play a beeping sound for a given duration (at 60Hz).
    pub fn play_sound(&mut self, duration: u8) {
        // create square wave
        let tone_volume = 1_000i16;
        let period: usize = (self.device.spec().freq / 256).try_into().unwrap();
        // create double the amount of samples to account for delays
        let sample_count: usize =
            (2 * self.device.spec().freq * self.device.spec().channels as i32 * duration as i32
                / 60)
                .try_into()
                .unwrap();
        let mut sound_to_play = vec![0; sample_count];
        for x in 0..sample_count {
            sound_to_play[x] = if (x / period) % 2 == 0 {
                tone_volume
            } else {
                -tone_volume
            };
        }

        // play
        self.device.pause();
        self.device.clear();
        self.device
            .queue_audio(&sound_to_play)
            .unwrap_or_else(|err| {
                eprintln!("Cannot play audio: {}", err);
            });
        self.device.resume();
        self.time = duration;
        self.ticks = duration as u32 * self.freq / 60;
    }
}

/// Each tick, the audio manager check if the audio timer is expired and possibly stops the playback.
impl CanTick for Audio {
    fn tick(&mut self) {
        if self.time > 0 {
            self.ticks -= 1;
            self.time = (self.ticks * 60 / self.freq) as u8;
        }
        if self.time == 0 {
            self.device.pause();
        }
    }
}

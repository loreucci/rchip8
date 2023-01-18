use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::Sdl;

use super::commons::CanTick;

const AUDIO_FREQUENCY: i32 = 48_000;

pub struct Audio {
    timer: u8,
    device: AudioQueue<i16>,
}

impl Audio {
    pub fn new(sdl: &Sdl) -> Result<Audio, String> {
        let subsystem = sdl.audio()?;
        let spec = AudioSpecDesired {
            freq: Some(AUDIO_FREQUENCY),
            channels: Some(2),
            samples: None, // default sample size
        };
        let device = subsystem.open_queue::<i16, _>(None, &spec)?;
        Ok(Audio { timer: 0, device })
    }

    pub fn play_sound(&mut self, duration: u8) {
        // create square wave
        let tone_volume = 1_000i16;
        let period: usize = (AUDIO_FREQUENCY / 256).try_into().unwrap();
        let sample_count: usize = (AUDIO_FREQUENCY * duration as i32 / 60).try_into().unwrap();
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
        self.timer = duration;
    }
}

impl CanTick for Audio {
    fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
            if self.timer == 0 {
                self.device.pause();
            }
        }
    }
}

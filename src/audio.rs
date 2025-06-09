use std::collections::HashMap;

#[cfg(all(feature = "audio", not(test)))]
use rodio::{OutputStream, OutputStreamHandle, Sink, Decoder, source::Source};
use std::io::Cursor;
use std::time::Duration;
use std::thread;

#[derive(Debug, Clone)]
pub struct AudioSettings {
    pub master: f32,
    pub sfx: f32,
    pub music: f32,
    pub voice: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { master: 1.0, sfx: 1.0, music: 1.0, voice: 1.0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AudioChannel {
    Master,
    Sfx,
    Music,
    Voice,
}

/// Very small audio manager used for tests and demos.
/// In production this would stream audio via `rodio`.
pub struct AudioSystem {
    #[cfg(all(feature = "audio", not(test)))]
    stream: OutputStream,
    #[cfg(all(feature = "audio", not(test)))]
    handle: OutputStreamHandle,
    sounds: HashMap<String, Vec<u8>>, // key -> raw audio bytes
    #[cfg(all(feature = "audio", not(test)))]
    music_sink: Option<Sink>,
    #[cfg(any(test, not(feature = "audio")))]
    pub current_music: Option<String>,
    pub settings: AudioSettings,
    /// Records which sound keys were played. Useful in tests.
    pub played_log: Vec<String>,
}

impl AudioSystem {
    /// Create a new audio system.
    #[cfg(all(feature = "audio", not(test)))]
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("audio output");
        Self {
            stream,
            handle,
            sounds: HashMap::new(),
            music_sink: None,
            settings: AudioSettings::default(),
            played_log: Vec::new(),
        }
    }

    /// Headless constructor used without the `audio` feature or in tests.
    #[cfg(any(test, not(feature = "audio")))]
    pub fn new() -> Self {
        Self { sounds: HashMap::new(), current_music: None, settings: AudioSettings::default(), played_log: Vec::new() }
    }

    /// Load a sound from raw bytes.
    pub fn load_sound_from_bytes(&mut self, key: &str, data: Vec<u8>) {
        self.sounds.insert(key.to_string(), data);
    }

    /// Play a sound effect previously loaded.
    pub fn play(&mut self, key: &str) {
        if let Some(bytes) = self.sounds.get(key) {
            #[cfg(all(feature = "audio", not(test)))]
            if let Ok(decoder) = Decoder::new(Cursor::new(bytes.clone())) {
                let sink = Sink::try_new(&self.handle).expect("sink");
                sink.set_volume(self.settings.master * self.settings.sfx);
                sink.append(decoder.convert_samples());
                sink.detach();
            }
        }
        self.played_log.push(key.to_string());
    }

    /// Play a background music track, crossfading if one is already playing.
    pub fn play_background_music(&mut self, key: &str) {
        #[cfg(all(feature = "audio", not(test)))]
        {
            if let Some(bytes) = self.sounds.get(key) {
                if let Ok(decoder) = Decoder::new(Cursor::new(bytes.clone())) {
                    let new_sink = Sink::try_new(&self.handle).expect("sink");
                    new_sink.set_volume(0.0);
                    new_sink.append(decoder.convert_samples());
                    new_sink.play();
                    let target_volume = self.settings.master * self.settings.music;
                    if let Some(old) = self.music_sink.replace(new_sink.clone()) {
                        let new_clone = new_sink.clone();
                        thread::spawn(move || {
                            for step in 0..10 {
                                let v = step as f32 / 10.0;
                                old.set_volume(target_volume * (1.0 - v));
                                new_clone.set_volume(target_volume * v);
                                thread::sleep(Duration::from_millis(100));
                            }
                            old.stop();
                            new_clone.set_volume(target_volume);
                        });
                    } else {
                        new_sink.set_volume(target_volume);
                    }
                    self.music_sink = Some(new_sink);
                }
            }
        }
        #[cfg(any(test, not(feature = "audio")))]
        {
            self.current_music = Some(key.to_string());
        }
        self.played_log.push(format!("music:{}", key));
    }

    /// Change the volume for a specific audio channel.
    pub fn set_volume(&mut self, channel: AudioChannel, value: f32) {
        match channel {
            AudioChannel::Master => self.settings.master = value,
            AudioChannel::Sfx => self.settings.sfx = value,
            AudioChannel::Music => self.settings.music = value,
            AudioChannel::Voice => self.settings.voice = value,
        }
        #[cfg(all(feature = "audio", not(test)))]
        {
            if let Some(sink) = &self.music_sink {
                sink.set_volume(self.settings.master * self.settings.music);
            }
        }
    }
}

use std::collections::HashMap;

#[cfg(all(feature = "audio", not(test)))]
use rodio::{OutputStream, OutputStreamHandle, Sink, Decoder, source::Source};
use std::io::Cursor;

/// Very small audio manager used for tests and demos.
/// In production this would stream audio via `rodio`.
pub struct AudioSystem {
    #[cfg(all(feature = "audio", not(test)))]
    stream: OutputStream,
    #[cfg(all(feature = "audio", not(test)))]
    handle: OutputStreamHandle,
    sounds: HashMap<String, Vec<u8>>, // key -> raw audio bytes
    /// Records which sound keys were played. Useful in tests.
    pub played_log: Vec<String>,
}

impl AudioSystem {
    /// Create a new audio system.
    #[cfg(all(feature = "audio", not(test)))]
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("audio output");
        Self { stream, handle, sounds: HashMap::new(), played_log: Vec::new() }
    }

    /// Headless constructor used without the `audio` feature or in tests.
    #[cfg(any(test, not(feature = "audio")))]
    pub fn new() -> Self {
        Self { sounds: HashMap::new(), played_log: Vec::new() }
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
                sink.append(decoder.convert_samples());
                sink.detach();
            }
        }
        self.played_log.push(key.to_string());
    }
}

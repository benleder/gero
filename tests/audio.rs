use gero::audio::AudioSystem;
use gero::audio::{AudioChannel};

#[test]
fn load_and_play_records_sound() {
    let mut audio = AudioSystem::new();
    audio.load_sound_from_bytes("beep", vec![1, 2, 3]);
    audio.play("beep");
    assert_eq!(audio.played_log, vec!["beep"]);
}

#[test]
fn playing_unloaded_sound_is_logged() {
    let mut audio = AudioSystem::new();
    audio.play("missing");
    assert_eq!(audio.played_log, vec!["missing"]);
}

#[test]
fn volume_adjustments_update_settings() {
    let mut audio = AudioSystem::new();
    audio.set_volume(AudioChannel::Music, 0.5);
    assert_eq!(audio.settings.music, 0.5);
    audio.set_volume(AudioChannel::Master, 0.8);
    assert_eq!(audio.settings.master, 0.8);
}

#[test]
fn background_music_changes_track() {
    let mut audio = AudioSystem::new();
    audio.load_sound_from_bytes("track1", vec![1]);
    audio.load_sound_from_bytes("track2", vec![2]);
    audio.play_background_music("track1");
    assert_eq!(audio.current_music.as_deref(), Some("track1"));
    audio.play_background_music("track2");
    assert_eq!(audio.current_music.as_deref(), Some("track2"));
    assert_eq!(audio.played_log, vec!["music:track1", "music:track2"]);
}

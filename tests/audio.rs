use gero::audio::AudioSystem;

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

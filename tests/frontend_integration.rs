use gero::frontend::Renderer;
use gero::models::{Unit, UnitType, Faction, Position};
use gero::state::GameState;

#[test]
fn renderer_can_render_state() {
    let mut unit = Unit::new("u1", "Test", UnitType::Guardsman, Faction::Imperial);
    unit.grid_position = Position { x: 1, y: 1 };
    let state = GameState::new(vec![unit]);
    let mut renderer = Renderer::new_headless(800, 600);
    renderer.render_state(&state);
    assert!(renderer.sprites.contains_key("u1"));
}

#[test]
fn renderer_issues_draw_calls() {
    let mut renderer = Renderer::new_headless(64, 64);
    renderer.load_sprite_from_bytes("guard", vec![vec![0, 1], vec![2, 3]]);
    let mut unit = Unit::new("u", "U", UnitType::Guardsman, Faction::Imperial);
    unit.sprite_id = "guard".into();
    unit.animation_state.frame_index = 1;
    unit.grid_position = Position { x: 3, y: 4 };
    let state = GameState::new(vec![unit]);
    renderer.render_state(&state);
    assert_eq!(renderer.draw_log.len(), 1);
    let call = &renderer.draw_log[0];
    assert_eq!(call.sprite_id, "guard");
    assert_eq!(call.position, (3, 4));
    assert_eq!(call.frame_index, 1);
}

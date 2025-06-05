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

use gero::models::{Unit, UnitType, Faction, StatusEffect, EffectType};
use gero::state::GameState;

#[test]
fn status_effects_persist_through_save() {
    let mut unit = Unit::new("u", "Unit", UnitType::Guardsman, Faction::Imperial);
    unit.status_effects.push(StatusEffect { effect_type: EffectType::Stun, remaining_turns: 2, magnitude: 0 });
    let state = GameState::new(vec![unit.clone()]);
    let data = state.save_to_string();
    let loaded = GameState::load_from_str(&data);
    assert_eq!(loaded.units[0].status_effects.len(), 1);
    assert!(matches!(loaded.units[0].status_effects[0].effect_type, EffectType::Stun));
    assert_eq!(loaded.units[0].status_effects[0].remaining_turns, 2);
}

use gero::combat::{CombatEncounter, EnvironmentalEffect};
use gero::models::{Unit, UnitType, Faction, Position};
use gero::grid::GridMap;

#[test]
fn smoke_cloud_expires() {
    let unit = Unit::new("u1", "Unit", UnitType::Guardsman, Faction::Imperial);
    let mut encounter = CombatEncounter::new(vec![unit], vec![], GridMap::new(3, 3), None);
    encounter.environmental_effects.push(EnvironmentalEffect::SmokeCloud {
        center: Position { x: 1, y: 1 },
        radius: 1,
        turns_remaining: 2,
    });

    encounter.start_turn();
    encounter.end_turn();
    assert_eq!(encounter.environmental_effects.len(), 1);
    if let EnvironmentalEffect::SmokeCloud { turns_remaining, .. } = &encounter.environmental_effects[0] {
        assert_eq!(*turns_remaining, 1);
    } else {
        panic!("wrong effect");
    }

    encounter.start_turn();
    encounter.end_turn();
    assert!(encounter.environmental_effects.is_empty());
}

#[test]
fn fire_patch_deals_damage() {
    let mut unit = Unit::new("u", "U", UnitType::Guardsman, Faction::Imperial);
    unit.grid_position = Position { x: 0, y: 0 };
    let starting_hp = unit.health_points;
    let mut encounter = CombatEncounter::new(vec![unit], vec![], GridMap::new(2, 2), None);
    encounter.environmental_effects.push(EnvironmentalEffect::FirePatch {
        grid_cells: vec![Position { x: 0, y: 0 }],
        damage_per_turn: 2,
    });

    encounter.start_turn();
    let hp_after = encounter.player_units[0].health_points;
    assert_eq!(hp_after, starting_hp - 2);
}

#[test]
fn acid_pool_reduces_agility_temporarily() {
    let mut unit = Unit::new("u", "U", UnitType::Guardsman, Faction::Imperial);
    unit.base_stats.agility = 4;
    unit.apply_equipment();
    unit.grid_position = Position { x: 0, y: 0 };
    let mut encounter = CombatEncounter::new(vec![unit], vec![], GridMap::new(2, 2), None);
    encounter.environmental_effects.push(EnvironmentalEffect::AcidPool {
        grid_cells: vec![Position { x: 0, y: 0 }],
        movement_penalty: 0.5,
    });

    encounter.start_turn();
    assert_eq!(encounter.player_units[0].current_stats.agility, 2);
    encounter.end_turn();
    assert_eq!(encounter.player_units[0].current_stats.agility, 4);
}

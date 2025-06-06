use gero::combat::{CombatEncounter};
use gero::models::{Unit, UnitType, Faction, Weapon, WeaponTier, Ability, AbilityType, AbilityEffect, AnimationType, Position};
use gero::grid::GridMap;

fn basic_weapon(range: u32) -> Weapon {
    Weapon {
        id: "w".into(),
        name: "Blade".into(),
        tier: WeaponTier::Basic,
        damage: 2,
        accuracy: 1.0,
        range,
        armor_piercing: None,
        action_point_cost: 1,
        critical_chance: 0.0,
        abilities_granted: Vec::new(),
    }
}

#[test]
fn ai_moves_toward_target_when_out_of_range() {
    let mut enemy = Unit::new("e", "E", UnitType::OrkBoy, Faction::Ork);
    enemy.base_stats.agility = 4;
    enemy.apply_equipment(); // update current_stats
    enemy.equipment.weapon = Some(basic_weapon(1));
    let mut player = Unit::new("p", "P", UnitType::Guardsman, Faction::Imperial);
    player.grid_position = Position { x: 3, y: 0 };

    let mut encounter = CombatEncounter::new(vec![player], vec![enemy], GridMap::new(5,5));
    encounter.turn_order.initiative.clear();
    encounter.turn_order.add_unit("e".into());
    encounter.turn_order.add_unit("p".into());

    encounter.run_enemy_turn(50);

    assert_eq!(encounter.enemy_units[0].grid_position, Position { x: 2, y: 0 });
    assert_eq!(encounter.player_units[0].health_points, encounter.player_units[0].current_stats.max_health);
}

#[test]
fn ai_uses_best_available_ability() {
    let mut enemy = Unit::new("e", "E", UnitType::OrkBoy, Faction::Ork);
    enemy.action_points = 2;
    enemy.equipment.weapon = Some(basic_weapon(3));
    enemy.abilities.push(Ability {
        id: "a".into(),
        name: "Zap".into(),
        ability_type: AbilityType::RangedAttack,
        description: String::new(),
        action_point_cost: 1,
        cooldown: 1,
        current_cooldown: 0,
        range: 3,
        area_of_effect: None,
        effect: AbilityEffect { damage: Some(5), healing: None, buff: None, debuff: None, status_applied: None, duration: None },
        animation: AnimationType::AbilityCast,
        sound_effect_key: String::new(),
    });

    let mut player = Unit::new("p", "P", UnitType::Guardsman, Faction::Imperial);
    player.grid_position = Position { x: 0, y: 2 };

    let mut encounter = CombatEncounter::new(vec![player], vec![enemy], GridMap::new(5,5));
    encounter.turn_order.initiative.clear();
    encounter.turn_order.add_unit("e".into());
    encounter.turn_order.add_unit("p".into());

    let starting_hp = encounter.player_units[0].health_points;
    encounter.run_enemy_turn(50);

    assert_eq!(encounter.player_units[0].health_points, starting_hp - 5);
    assert_eq!(encounter.enemy_units[0].abilities[0].current_cooldown, 1);
    assert_eq!(encounter.enemy_units[0].action_points, 1);
}

#[test]
fn ai_falls_back_to_weapon_when_ability_unavailable() {
    let mut enemy = Unit::new("e", "E", UnitType::OrkBoy, Faction::Ork);
    enemy.action_points = 2;
    enemy.equipment.weapon = Some(basic_weapon(1));
    enemy.abilities.push(Ability {
        id: "cool".into(),
        name: "Cool Blast".into(),
        ability_type: AbilityType::RangedAttack,
        description: String::new(),
        action_point_cost: 1,
        cooldown: 2,
        current_cooldown: 1, // not ready
        range: 3,
        area_of_effect: None,
        effect: AbilityEffect { damage: Some(5), healing: None, buff: None, debuff: None, status_applied: None, duration: None },
        animation: AnimationType::AbilityCast,
        sound_effect_key: String::new(),
    });

    let mut player = Unit::new("p", "P", UnitType::Guardsman, Faction::Imperial);
    player.grid_position = Position { x: 1, y: 0 };

    let mut encounter = CombatEncounter::new(vec![player], vec![enemy], GridMap::new(3,3));
    encounter.turn_order.initiative.clear();
    encounter.turn_order.add_unit("e".into());
    encounter.turn_order.add_unit("p".into());

    let starting_hp = encounter.player_units[0].health_points;
    encounter.run_enemy_turn(50);

    assert_eq!(encounter.player_units[0].health_points, starting_hp - 2); // weapon damage
    assert_eq!(encounter.enemy_units[0].grid_position, Position { x: 0, y: 0 });
    assert_eq!(encounter.enemy_units[0].abilities[0].current_cooldown, 1);
    assert_eq!(encounter.enemy_units[0].action_points, 1);
}

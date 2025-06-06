use gero::models::{Unit, UnitType, Faction, Ability, AbilityType, AbilityEffect, AnimationType, StatsModifier, EffectType};
use gero::combat::use_ability;

fn make_heal_buff_ability() -> Ability {
    Ability {
        id: "heal".into(),
        name: "Heal".into(),
        ability_type: AbilityType::Healing,
        description: String::new(),
        action_point_cost: 1,
        cooldown: 0,
        current_cooldown: 0,
        range: 5,
        area_of_effect: None,
        effect: AbilityEffect {
            damage: None,
            healing: Some(3),
            buff: Some(StatsModifier { strength_mod: 1, toughness_mod: 0, agility_mod: 0, intellect_mod: 0, willpower_mod: 0, fellowship_mod: 0 }),
            debuff: None,
            status_applied: None,
            duration: None,
        },
        animation: AnimationType::AbilityCast,
        sound_effect_key: String::new(),
    }
}

fn make_status_ability() -> Ability {
    Ability {
        id: "poison".into(),
        name: "Poison".into(),
        ability_type: AbilityType::RangedAttack,
        description: String::new(),
        action_point_cost: 1,
        cooldown: 0,
        current_cooldown: 0,
        range: 5,
        area_of_effect: None,
        effect: AbilityEffect {
            damage: None,
            healing: None,
            buff: None,
            debuff: None,
            status_applied: Some(EffectType::Poison),
            duration: Some(2),
        },
        animation: AnimationType::AbilityCast,
        sound_effect_key: String::new(),
    }
}

#[test]
fn heal_and_buff_increases_stats() {
    let mut user = Unit::new("u", "User", UnitType::Guardsman, Faction::Imperial);
    let mut target = Unit::new("t", "Target", UnitType::Guardsman, Faction::Imperial);
    target.health_points = 5;
    user.action_points = 2;
    user.abilities.push(make_heal_buff_ability());
    let _ = use_ability(&mut user, 0, &mut [&mut target], None).unwrap();
    assert_eq!(target.health_points, 8);
    assert_eq!(target.current_stats.strength, target.base_stats.strength + 1);
}

#[test]
fn applying_status_effect_adds_to_unit() {
    let mut user = Unit::new("u", "User", UnitType::Guardsman, Faction::Imperial);
    let mut target = Unit::new("t", "Target", UnitType::Guardsman, Faction::Imperial);
    user.action_points = 2;
    user.abilities.push(make_status_ability());
    assert!(target.status_effects.is_empty());
    let _ = use_ability(&mut user, 0, &mut [&mut target], None).unwrap();
    assert_eq!(target.status_effects.len(), 1);
    let se = &target.status_effects[0];
    assert!(matches!(se.effect_type, EffectType::Poison));
    assert_eq!(se.remaining_turns, 2);
}

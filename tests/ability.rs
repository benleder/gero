use gero::models::{Unit, UnitType, Faction, Ability, AbilityType, AbilityEffect, AreaOfEffect, AnimationType};
use gero::combat::{use_ability, tick_cooldowns};

#[test]
fn single_target_ability() {
    let mut user = Unit::new("u", "User", UnitType::Guardsman, Faction::Imperial);
    let mut target = Unit::new("t", "Target", UnitType::OrkBoy, Faction::Ork);

    user.action_points = 2;
    user.abilities.push(Ability {
        id: "a".into(),
        name: "Bolt".into(),
        ability_type: AbilityType::RangedAttack,
        description: String::new(),
        action_point_cost: 1,
        cooldown: 2,
        current_cooldown: 0,
        range: 5,
        area_of_effect: None,
        effect: AbilityEffect {
            damage: Some(3),
            healing: None,
            buff: None,
            debuff: None,
            status_applied: None,
            duration: None,
        },
        animation: AnimationType::AbilityCast,
        sound_effect_key: String::new(),
    });

    let res = use_ability(&mut user, 0, &mut [&mut target]);
    assert!(res.is_ok());
    assert_eq!(user.action_points, 1);
    assert_eq!(user.abilities[0].current_cooldown, 2);
    assert_eq!(target.health_points, target.current_stats.max_health - 3);

    tick_cooldowns(&mut user);
    assert_eq!(user.abilities[0].current_cooldown, 1);
}

#[test]
fn aoe_hits_multiple_targets() {
    let mut user = Unit::new("u", "User", UnitType::Guardsman, Faction::Imperial);
    let mut t1 = Unit::new("t1", "T1", UnitType::OrkBoy, Faction::Ork);
    let mut t2 = Unit::new("t2", "T2", UnitType::OrkBoy, Faction::Ork);

    user.action_points = 2;
    user.abilities.push(Ability {
        id: "blast".into(),
        name: "Blast".into(),
        ability_type: AbilityType::RangedAttack,
        description: String::new(),
        action_point_cost: 1,
        cooldown: 1,
        current_cooldown: 0,
        range: 5,
        area_of_effect: Some(AreaOfEffect::Circle { radius: 1 }),
        effect: AbilityEffect {
            damage: Some(2),
            healing: None,
            buff: None,
            debuff: None,
            status_applied: None,
            duration: None,
        },
        animation: AnimationType::AbilityCast,
        sound_effect_key: String::new(),
    });

    let res = use_ability(&mut user, 0, &mut [&mut t1, &mut t2]);
    assert!(res.is_ok());
    assert_eq!(t1.health_points, t1.current_stats.max_health - 2);
    assert_eq!(t2.health_points, t2.current_stats.max_health - 2);
}

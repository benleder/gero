use gero::models::{Unit, UnitType, Faction, Armor, ArmorTier, Weapon, WeaponTier};

#[test]
fn armor_modifiers_change_stats() {
    let mut unit = Unit::new("u", "Unit", UnitType::Guardsman, Faction::Imperial);
    unit.base_stats.toughness = 3;
    unit.base_stats.agility = 4;
    unit.apply_equipment();

    let armor = Armor {
        id: "a1".into(),
        name: "Flak".into(),
        tier: ArmorTier::Flak,
        toughness_bonus: 2,
        agility_penalty: -1,
        special_properties: Vec::new(),
    };

    unit.equip_armor(armor.clone());
    assert_eq!(unit.current_stats.toughness, 5);
    assert_eq!(unit.current_stats.agility, 3);

    unit.unequip_armor();
    assert_eq!(unit.current_stats.toughness, unit.base_stats.toughness);
    assert_eq!(unit.current_stats.agility, unit.base_stats.agility);
}

#[test]
fn weapon_equipment_pipeline_keeps_stats_unchanged() {
    let mut unit = Unit::new("u", "Unit", UnitType::Guardsman, Faction::Imperial);
    unit.base_stats.strength = 2;
    unit.base_stats.agility = 3;
    unit.apply_equipment();

    let weapon = Weapon {
        id: "w1".into(),
        name: "Lasgun".into(),
        tier: WeaponTier::Basic,
        damage: 2,
        accuracy: 1.0,
        range: 5,
        armor_piercing: None,
        action_point_cost: 1,
        critical_chance: 0.0,
        abilities_granted: Vec::new(),
    };

    let base = unit.base_stats.clone();
    unit.equip_weapon(weapon.clone());
    assert_eq!(unit.current_stats.strength, base.strength);
    assert_eq!(unit.current_stats.toughness, base.toughness);
    assert_eq!(unit.current_stats.agility, base.agility);
    assert_eq!(unit.current_stats.intellect, base.intellect);
    assert_eq!(unit.current_stats.willpower, base.willpower);
    assert_eq!(unit.current_stats.fellowship, base.fellowship);
    assert_eq!(unit.current_stats.max_health, base.max_health);
    assert_eq!(unit.current_stats.max_action, base.max_action);

    let returned = unit.unequip_weapon().expect("weapon returned");
    assert_eq!(returned.id, weapon.id);
    assert_eq!(returned.name, weapon.name);
    assert!(matches!(returned.tier, WeaponTier::Basic));
    assert_eq!(returned.damage, weapon.damage);
    assert_eq!(returned.accuracy, weapon.accuracy);
    assert_eq!(returned.range, weapon.range);
    assert_eq!(returned.armor_piercing, weapon.armor_piercing);
    assert_eq!(returned.action_point_cost, weapon.action_point_cost);
    assert_eq!(returned.critical_chance, weapon.critical_chance);
    assert_eq!(returned.abilities_granted.len(), weapon.abilities_granted.len());

    assert_eq!(unit.current_stats.strength, base.strength);
    assert_eq!(unit.current_stats.toughness, base.toughness);
    assert_eq!(unit.current_stats.agility, base.agility);
    assert_eq!(unit.current_stats.intellect, base.intellect);
    assert_eq!(unit.current_stats.willpower, base.willpower);
    assert_eq!(unit.current_stats.fellowship, base.fellowship);
    assert_eq!(unit.current_stats.max_health, base.max_health);
    assert_eq!(unit.current_stats.max_action, base.max_action);
}

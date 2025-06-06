use gero::models::{Unit, UnitType, Faction, Armor, ArmorTier};

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

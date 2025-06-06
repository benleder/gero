use gero::models::{Unit, UnitType, Faction, Weapon, WeaponTier};
use gero::combat::resolve_attack;

fn setup_units() -> (Unit, Unit, Weapon) {
    let mut attacker = Unit::new("a", "Attacker", UnitType::Guardsman, Faction::Imperial);
    attacker.current_stats.agility = 3;
    attacker.current_stats.strength = 2;
    let mut defender = Unit::new("d", "Defender", UnitType::OrkBoy, Faction::Ork);
    defender.current_stats.toughness = 2;
    let weapon = Weapon {
        id: "w".into(),
        name: "Rifle".into(),
        tier: WeaponTier::Basic,
        damage: 3,
        accuracy: 0.5,
        range: 5,
        armor_piercing: None,
        action_point_cost: 1,
        critical_chance: 0.0,
        abilities_granted: Vec::new(),
    };
    (attacker, defender, weapon)
}

#[test]
fn attack_misses_with_low_hit_chance() {
    let (mut a, mut d, w) = setup_units();
    // High roll so it should miss
    let res = resolve_attack(&mut a, &w, &mut d, 99, 0);
    assert!(!res.hit);
    assert_eq!(res.damage, 0);
    // action points spent even on miss
    assert_eq!(a.action_points, a.current_stats.max_action - w.action_point_cost);
}

#[test]
fn critical_hit_doubles_damage() {
    let (mut a, mut d, mut w) = setup_units();
    w.damage = 2;
    let starting_hp = d.health_points;
    // roll <=10 triggers critical
    let res = resolve_attack(&mut a, &w, &mut d, 5, 0);
    assert!(res.hit);
    assert_eq!(d.health_points, starting_hp - res.damage);
    assert!(res.damage > w.damage); // should be doubled
}

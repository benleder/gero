use gero::models::{Unit, UnitType, Faction, Position};
use gero::grid::{GridMap, TerrainType, try_move};

#[test]
fn hazardous_tile_applies_damage() {
    let mut unit = Unit::new("u", "U", UnitType::Guardsman, Faction::Imperial);
    unit.current_stats.agility = 10; // 5 MP
    let mut map = GridMap::new(3, 1);
    map.set_terrain(&Position { x: 2, y: 0 }, TerrainType::Hazardous);
    let start_hp = unit.health_points;
    assert!(try_move(&mut unit, Position { x: 2, y: 0 }, &map));
    assert_eq!(unit.grid_position, Position { x: 2, y: 0 });
    assert_eq!(unit.health_points, start_hp - 1);
}

#[test]
fn move_out_of_bounds_fails() {
    let mut unit = Unit::new("u", "U", UnitType::Guardsman, Faction::Imperial);
    unit.current_stats.agility = 4;
    let map = GridMap::new(2, 2);
    assert!(!try_move(&mut unit, Position { x: 2, y: 2 }, &map));
    assert_eq!(unit.grid_position, Position { x: 0, y: 0 });
}

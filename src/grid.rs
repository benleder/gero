use crate::models::{Position, Unit};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Normal,
    Difficult,
    Hazardous,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TerrainType>,
}

impl GridMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, tiles: vec![TerrainType::Normal; width * height] }
    }

    fn index(&self, pos: &Position) -> usize {
        pos.y * self.width + pos.x
    }

    pub fn set_terrain(&mut self, pos: &Position, terrain: TerrainType) {
        let idx = self.index(pos);
        self.tiles[idx] = terrain;
    }

    pub fn terrain_at(&self, pos: &Position) -> &TerrainType {
        &self.tiles[self.index(pos)]
    }

    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}

/// Calculate movement cost between two adjacent tiles
fn tile_cost(terrain: &TerrainType, diagonal: bool) -> u32 {
    let mut cost = if diagonal { 2 } else { 1 };
    match terrain {
        TerrainType::Difficult => cost += 1,
        TerrainType::Hazardous => cost += 2,
        TerrainType::Blocked => cost = u32::MAX,
        TerrainType::Normal => {}
    }
    cost
}

/// Attempt to move a unit to dest if within movement points
pub fn try_move(unit: &mut Unit, dest: Position, map: &GridMap) -> bool {
    if !map.in_bounds(&dest) {
        return false;
    }

    // Simple cost: manhattan with diagonal cost, ignoring pathfinding
    let dx = (dest.x as isize - unit.grid_position.x as isize).abs() as u32;
    let dy = (dest.y as isize - unit.grid_position.y as isize).abs() as u32;
    let diag = dx.min(dy);
    let straight = dx.max(dy) - diag;
    let mut cost = diag * 2 + straight;

    // Terrain cost of destination
    cost += match map.terrain_at(&dest) {
        TerrainType::Difficult => 1,
        TerrainType::Hazardous => 2,
        TerrainType::Blocked => return false,
        TerrainType::Normal => 0,
    };

    let max_mp = unit.current_stats.agility as u32 / 2;
    if cost <= max_mp {
        unit.grid_position = dest;
        if let TerrainType::Hazardous = map.terrain_at(&unit.grid_position) {
            unit.health_points -= 1;
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Faction, UnitType};

    fn basic_unit() -> Unit {
        Unit::new("u1", "test", UnitType::Guardsman, Faction::Imperial)
    }

    #[test]
    fn move_within_range() {
        let mut unit = basic_unit();
        unit.current_stats.agility = 4;
        let map = GridMap::new(10, 10);
        assert!(try_move(&mut unit, Position { x: 2, y: 0 }, &map));
    }

    #[test]
    fn move_too_far() {
        let mut unit = basic_unit();
        unit.current_stats.agility = 2;
        let map = GridMap::new(10, 10);
        assert!(!try_move(&mut unit, Position { x: 3, y: 0 }, &map));
    }
}


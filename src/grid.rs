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

/// Attempt to move a unit to `dest` using A* pathfinding. The unit will move if
/// the cheapest path costs no more movement points than allowed by its agility.
pub fn try_move(unit: &mut Unit, dest: Position, map: &GridMap) -> bool {
    use std::collections::{BinaryHeap, HashMap};

    if !map.in_bounds(&dest) {
        return false;
    }

    if let TerrainType::Blocked = map.terrain_at(&dest) {
        return false;
    }

    let max_mp = unit.current_stats.agility as u32 / 2;

    // Heuristic using octile distance (diagonal cost = 2, straight = 1)
    let heuristic = |a: &Position, b: &Position| -> u32 {
        let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
        let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
        let diag = dx.min(dy);
        let straight = dx.max(dy) - diag;
        diag as u32 * 2 + straight as u32
    };

    #[derive(Eq, PartialEq)]
    struct Node {
        score: u32,
        cost: u32,
        pos: Position,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other
                .score
                .cmp(&self.score)
                .then_with(|| other.cost.cmp(&self.cost))
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let start = unit.grid_position.clone();
    open.push(Node { score: heuristic(&start, &dest), cost: 0, pos: start.clone() });

    let mut best: HashMap<Position, u32> = HashMap::new();
    best.insert(start.clone(), 0);

    let dirs: &[(isize, isize)] = &[
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut final_cost = None;
    while let Some(Node { score: _, cost, pos }) = open.pop() {
        if let Some(best) = final_cost {
            if cost > best {
                continue;
            }
        }
        if pos == dest {
            final_cost = Some(cost);
            break;
        }

        for (dx, dy) in dirs {
            let nx = pos.x as isize + dx;
            let ny = pos.y as isize + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let npos = Position {
                x: nx as usize,
                y: ny as usize,
            };
            if !map.in_bounds(&npos) {
                continue;
            }
            let diagonal = *dx != 0 && *dy != 0;
            if diagonal {
                let adj1 = Position { x: pos.x, y: ny as usize };
                let adj2 = Position { x: nx as usize, y: pos.y };
                if matches!(map.terrain_at(&adj1), TerrainType::Blocked)
                    || matches!(map.terrain_at(&adj2), TerrainType::Blocked)
                {
                    continue;
                }
            }
            let step = tile_cost(map.terrain_at(&npos), diagonal);
            if step == u32::MAX {
                continue;
            }
            let next_cost = cost + step;
            if next_cost > max_mp {
                continue;
            }
            let entry = best.entry(npos.clone()).or_insert(u32::MAX);
            if next_cost < *entry {
                *entry = next_cost;
                open.push(Node {
                    score: next_cost + heuristic(&npos, &dest),
                    cost: next_cost,
                    pos: npos,
                });
            }
        }
    }

    if let Some(cost) = final_cost {
        if cost <= max_mp {
            unit.grid_position = dest;
            if let TerrainType::Hazardous = map.terrain_at(&unit.grid_position) {
                unit.health_points -= 1;
            }
            return true;
        }
    }

    false
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

    #[test]
    fn path_around_block() {
        let mut unit = basic_unit();
        unit.current_stats.agility = 8; // 4 MP
        let mut map = GridMap::new(5, 5);
        map.set_terrain(&Position { x: 1, y: 0 }, TerrainType::Blocked);
        assert!(try_move(&mut unit, Position { x: 2, y: 0 }, &map));
        assert_eq!(unit.grid_position, Position { x: 2, y: 0 });
    }

    #[test]
    fn no_path_blocked() {
        let mut unit = basic_unit();
        unit.current_stats.agility = 8;
        let mut map = GridMap::new(3, 3);
        map.set_terrain(&Position { x: 1, y: 0 }, TerrainType::Blocked);
        map.set_terrain(&Position { x: 0, y: 1 }, TerrainType::Blocked);
        assert!(!try_move(&mut unit, Position { x: 1, y: 1 }, &map));
        assert_eq!(unit.grid_position, Position { x: 0, y: 0 });
    }
}


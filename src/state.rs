use serde::{Serialize, Deserialize};
use crate::models::Unit;
use crate::grid::GridMap;
use crate::combat::{TurnQueue, EnvironmentalEffect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub units: Vec<Unit>,
    pub map: GridMap,
    pub turn_queue: TurnQueue,
    pub environmental_effects: Vec<EnvironmentalEffect>,
}

impl GameState {
    pub fn new(units: Vec<Unit>) -> Self {
        let mut turn_queue = TurnQueue::new();
        for u in &units {
            turn_queue.add_unit(u.id.clone());
        }
        Self {
            units,
            map: GridMap::new(10, 10),
            turn_queue,
            environmental_effects: Vec::new(),
        }
    }

    pub fn save_to_string(&self) -> String {
        serde_json::to_string(self).expect("serialize game state")
    }

    pub fn load_from_str(data: &str) -> Self {
        serde_json::from_str(data).expect("deserialize game state")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{UnitType, Faction, Position};
    use crate::grid::{TerrainType};

    #[test]
    fn save_load_roundtrip() {
        let unit = Unit::new("u", "Unit", UnitType::Guardsman, Faction::Imperial);
        let state = GameState::new(vec![unit.clone()]);
        let data = state.save_to_string();
        let loaded = GameState::load_from_str(&data);
        assert_eq!(loaded.units[0].id, unit.id);
        assert_eq!(loaded.map.width, 10);
        assert_eq!(loaded.turn_queue.initiative.len(), 1);
        assert!(loaded.environmental_effects.is_empty());
    }

    #[test]
    fn roundtrip_nontrivial_encounter() {
        let unit1 = Unit::new("u1", "Unit1", UnitType::Guardsman, Faction::Imperial);
        let unit2 = Unit::new("u2", "Unit2", UnitType::OrkBoy, Faction::Ork);
        let mut state = GameState::new(vec![unit1.clone(), unit2.clone()]);
        state.map = GridMap::new(5, 5);
        state.map.set_terrain(&Position { x: 1, y: 1 }, TerrainType::Difficult);
        state.environmental_effects.push(EnvironmentalEffect::SmokeCloud {
            center: Position { x: 2, y: 2 },
            radius: 1,
            turns_remaining: 3,
        });
        state.turn_queue.next_turn();

        let data = state.save_to_string();
        let loaded = GameState::load_from_str(&data);

        assert_eq!(loaded.map.width, 5);
        assert_eq!(loaded.map.height, 5);
        assert!(matches!(loaded.map.terrain_at(&Position { x: 1, y: 1 }), TerrainType::Difficult));
        assert_eq!(loaded.environmental_effects.len(), 1);
        match &loaded.environmental_effects[0] {
            EnvironmentalEffect::SmokeCloud { center, radius, turns_remaining } => {
                assert_eq!(*center, Position { x: 2, y: 2 });
                assert_eq!(*radius, 1);
                assert_eq!(*turns_remaining, 3);
            }
            _ => panic!("wrong effect"),
        }
        assert_eq!(loaded.turn_queue.current_unit_id, state.turn_queue.current_unit_id);
    }
}


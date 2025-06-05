use serde::{Serialize, Deserialize};
use crate::models::Unit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub units: Vec<Unit>,
}

impl GameState {
    pub fn new(units: Vec<Unit>) -> Self {
        Self { units }
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
    use crate::models::{UnitType, Faction};

    #[test]
    fn save_load_roundtrip() {
        let unit = Unit::new("u", "Unit", UnitType::Guardsman, Faction::Imperial);
        let state = GameState::new(vec![unit.clone()]);
        let data = state.save_to_string();
        let loaded = GameState::load_from_str(&data);
        assert_eq!(loaded.units[0].id, unit.id);
    }
}


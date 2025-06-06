use crate::models::{AnimationType, Unit, Weapon, AbilityEffect, StatsModifier};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct AttackResult {
    pub hit: bool,
    pub damage: i32,
}

/// Resolve a weapon attack from attacker to defender.
pub fn resolve_attack(attacker: &mut Unit, weapon: &Weapon, defender: &mut Unit, roll: u8, cover_bonus: i32) -> AttackResult {
    if attacker.action_points < weapon.action_point_cost {
        return AttackResult { hit: false, damage: 0 };
    }
    attacker.action_points -= weapon.action_point_cost;

    let hit_chance = (attacker.current_stats.agility as f32 * 10.0 + weapon.accuracy * 100.0)
        - (defender.current_stats.agility as f32 * 10.0 + cover_bonus as f32);

    let mut hit = false;
    let mut damage = 0;
    if (roll as f32) <= hit_chance {
        hit = true;
        damage = (weapon.damage + attacker.current_stats.strength)
            - defender.current_stats.toughness;
        if damage < 0 {
            damage = 0;
        }
        if roll <= 10 {
            damage *= 2;
        }
        defender.health_points -= damage;
    }

    attacker.animation_state.current_animation = AnimationType::Attack;

    AttackResult { hit, damage }
}

/// Apply an ability effect to a single unit.
fn apply_ability_effect(effect: &AbilityEffect, target: &mut Unit) {
    if let Some(dmg) = effect.damage {
        target.health_points -= dmg;
    }
    if let Some(heal) = effect.healing {
        target.health_points += heal;
        if target.health_points > target.current_stats.max_health {
            target.health_points = target.current_stats.max_health;
        }
    }
    if let Some(buff) = &effect.buff {
        modify_stats(&mut target.current_stats, buff, 1);
    }
    if let Some(debuff) = &effect.debuff {
        modify_stats(&mut target.current_stats, debuff, -1);
    }
    if let Some(status) = effect.status_applied.clone() {
        target.status_effects.push(crate::models::StatusEffect {
            effect_type: status,
            remaining_turns: effect.duration.unwrap_or(1),
            magnitude: 0,
        });
    }
}

fn modify_stats(stats: &mut crate::models::Stats, modifier: &StatsModifier, sign: i32) {
    stats.strength += modifier.strength_mod * sign;
    stats.toughness += modifier.toughness_mod * sign;
    stats.agility += modifier.agility_mod * sign;
    stats.intellect += modifier.intellect_mod * sign;
    stats.willpower += modifier.willpower_mod * sign;
    stats.fellowship += modifier.fellowship_mod * sign;
}

/// Use an ability on one or more targets.
pub fn use_ability(
    user: &mut Unit,
    ability_index: usize,
    targets: &mut [&mut Unit],
) -> Result<(), &'static str> {
    let ability = user
        .abilities
        .get_mut(ability_index)
        .ok_or("invalid ability")?;

    if user.action_points < ability.action_point_cost {
        return Err("not enough AP");
    }
    if ability.current_cooldown > 0 {
        return Err("ability on cooldown");
    }

    user.action_points -= ability.action_point_cost;
    ability.current_cooldown = ability.cooldown;
    user.animation_state.current_animation = ability.animation.clone();

    if ability.area_of_effect.is_some() {
        for t in targets.iter_mut() {
            apply_ability_effect(&ability.effect, *t);
        }
    } else {
        if let Some(first) = targets.get_mut(0) {
            apply_ability_effect(&ability.effect, *first);
        }
    }

    Ok(())
}

/// Decrement cooldowns on all of a unit's abilities.
pub fn tick_cooldowns(unit: &mut Unit) {
    for ability in &mut unit.abilities {
        if ability.current_cooldown > 0 {
            ability.current_cooldown -= 1;
        }
    }
}

use std::collections::VecDeque;
use crate::models::Position;

#[derive(Debug, Clone)]
pub struct CombatEncounter {
    pub player_units: Vec<Unit>,
    pub enemy_units: Vec<Unit>,
    pub battlefield: crate::grid::GridMap,
    pub turn_order: TurnQueue,
    pub current_phase: CombatPhase,
    pub environmental_effects: Vec<EnvironmentalEffect>,
    pub camera_state: CameraState,
}

impl CombatEncounter {
    pub fn new(player_units: Vec<Unit>, enemy_units: Vec<Unit>, battlefield: crate::grid::GridMap) -> Self {
        let mut turn_order = TurnQueue::new();
        for u in player_units.iter().chain(enemy_units.iter()) {
            turn_order.add_unit(u.id.clone());
        }
        Self {
            player_units,
            enemy_units,
            battlefield,
            turn_order,
            current_phase: CombatPhase::Movement,
            environmental_effects: Vec::new(),
            camera_state: CameraState { x_offset: 0.0, y_offset: 0.0, zoom_level: 1.0 },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnQueue {
    pub initiative: VecDeque<String>,
    pub current_unit_id: Option<String>,
    pub round_number: u32,
}

impl TurnQueue {
    pub fn new() -> Self {
        Self { initiative: VecDeque::new(), current_unit_id: None, round_number: 1 }
    }

    pub fn add_unit(&mut self, id: String) {
        self.initiative.push_back(id);
    }

    pub fn next_turn(&mut self) -> Option<String> {
        if let Some(id) = self.initiative.pop_front() {
            self.current_unit_id = Some(id.clone());
            self.initiative.push_back(id.clone());
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatPhase {
    Movement,
    Action,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentalEffect {
    SmokeCloud { center: Position, radius: u32, turns_remaining: u32 },
    FirePatch { grid_cells: Vec<Position>, damage_per_turn: i32 },
    AcidPool { grid_cells: Vec<Position>, movement_penalty: f32 },
}

#[derive(Debug, Clone)]
pub struct CameraState {
    pub x_offset: f32,
    pub y_offset: f32,
    pub zoom_level: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{UnitType, Faction, WeaponTier};

    fn basic_units() -> (Unit, Unit, Weapon) {
        let mut attacker = Unit::new("a", "A", UnitType::Guardsman, Faction::Imperial);
        attacker.current_stats.agility = 4;
        attacker.current_stats.strength = 3;
        let mut defender = Unit::new("d", "D", UnitType::OrkBoy, Faction::Ork);
        defender.current_stats.toughness = 2;
        let weapon = Weapon {
            id: "w".into(),
            name: "Gun".into(),
            tier: WeaponTier::Basic,
            damage: 3,
            accuracy: 0.8,
            range: 5,
            armor_piercing: None,
            action_point_cost: 1,
            critical_chance: 0.1,
            abilities_granted: Vec::new(),
        };
        (attacker, defender, weapon)
    }

    #[test]
    fn attack_hits() {
        let (mut a, mut d, w) = basic_units();
        let result = resolve_attack(&mut a, &w, &mut d, 5, 0);
        assert!(result.hit);
        assert!(result.damage > 0);
    }
}


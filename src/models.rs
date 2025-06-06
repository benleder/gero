use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,
    pub toughness: i32,
    pub agility: i32,
    pub intellect: i32,
    pub willpower: i32,
    pub fellowship: i32,
    pub max_health: i32,
    pub max_action: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            strength: 0,
            toughness: 0,
            agility: 0,
            intellect: 0,
            willpower: 0,
            fellowship: 0,
            max_health: 0,
            max_action: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitType {
    SpaceMarine,
    Guardsman,
    Commissar,
    TechPriest,
    OrkBoy,
    OrkNob,
    Weirdboy,
    Cultist,
    ChaosMarine,
    Daemon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Faction {
    Imperial,
    Ork,
    Chaos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: EffectType,
    pub remaining_turns: u32,
    pub magnitude: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Poison,
    Stun,
    Shield,
    Suppression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub current_animation: AnimationType,
    pub frame_index: usize,
    pub timer: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    Idle,
    Move,
    Attack,
    AbilityCast,
    Death,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    pub accessory_slots: Vec<Accessory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub id: String,
    pub name: String,
    pub tier: WeaponTier,
    pub damage: i32,
    pub accuracy: f32,
    pub range: u32,
    pub armor_piercing: Option<f32>,
    pub action_point_cost: u32,
    pub critical_chance: f32,
    pub abilities_granted: Vec<AbilityType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponTier {
    Basic,
    Advanced,
    MasterCrafted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub id: String,
    pub name: String,
    pub tier: ArmorTier,
    pub toughness_bonus: i32,
    pub agility_penalty: i32,
    pub special_properties: Vec<ArmorProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorTier {
    Flak,
    Carapace,
    PowerArmor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorProperty {
    ReactivePlating,
    InoculatedCeramite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Accessory {
    Grenade { damage: i32, aoe_radius: u32 },
    Stimpack { heal_amount: i32, cooldown: u32 },
    Medkit { heal_over_time: i32, duration: u32 },
    Bionics { stat_bonus: StatsModifier, duration: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: String,
    pub name: String,
    pub ability_type: AbilityType,
    pub description: String,
    pub action_point_cost: u32,
    pub cooldown: u32,
    pub current_cooldown: u32,
    pub range: u32,
    pub area_of_effect: Option<AreaOfEffect>,
    pub effect: AbilityEffect,
    pub animation: AnimationType,
    pub sound_effect_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityType {
    RangedAttack,
    MeleeAttack,
    PsychicBlast,
    Healing,
    Buff,
    Debuff,
    Summon,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AreaOfEffect {
    Cone { radius: u32 },
    Line { length: u32 },
    Circle { radius: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityEffect {
    pub damage: Option<i32>,
    pub healing: Option<i32>,
    pub buff: Option<StatsModifier>,
    pub debuff: Option<StatsModifier>,
    pub status_applied: Option<EffectType>,
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsModifier {
    pub strength_mod: i32,
    pub toughness_mod: i32,
    pub agility_mod: i32,
    pub intellect_mod: i32,
    pub willpower_mod: i32,
    pub fellowship_mod: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub unit_type: UnitType,
    pub level: u32,
    pub base_stats: Stats,
    pub current_stats: Stats,
    pub equipment: Equipment,
    pub abilities: Vec<Ability>,
    pub grid_position: Position,
    pub experience: u32,
    pub health_points: i32,
    pub action_points: u32,
    pub faction: Faction,
    pub status_effects: Vec<StatusEffect>,
    pub animation_state: AnimationState,
    pub sprite_id: String,
    pub is_selected: bool,
}

impl Unit {
    /// Helper constructor for tests
    pub fn new(id: &str, name: &str, unit_type: UnitType, faction: Faction) -> Self {
        let stats = Stats { max_health: 10, max_action: 2, ..Default::default() };
        Self {
            id: id.to_string(),
            name: name.to_string(),
            unit_type,
            level: 1,
            base_stats: stats.clone(),
            current_stats: stats.clone(),
            equipment: Equipment { weapon: None, armor: None, accessory_slots: Vec::new() },
            abilities: Vec::new(),
            grid_position: Position { x: 0, y: 0 },
            experience: 0,
            health_points: stats.max_health,
            action_points: stats.max_action,
            faction,
            status_effects: Vec::new(),
            animation_state: AnimationState { current_animation: AnimationType::Idle, frame_index: 0, timer: 0.0 },
            sprite_id: String::new(),
            is_selected: false,
        }
    }

    /// Recalculate current_stats based on base_stats and all equipped items.
    pub fn apply_equipment(&mut self) {
        self.current_stats = self.base_stats.clone();
        if let Some(armor) = &self.equipment.armor {
            self.current_stats.toughness += armor.toughness_bonus;
            self.current_stats.agility += armor.agility_penalty;
        }
        // Weapons currently do not modify stats but are included for completeness.
        if let Some(_weapon) = &self.equipment.weapon {
            // Placeholder for future weapon stat modifiers
        }
    }

    /// Remove all equipment modifiers, returning stats to base values.
    pub fn remove_equipment(&mut self) {
        self.current_stats = self.base_stats.clone();
    }

    /// Equip a new weapon and update stats accordingly.
    pub fn equip_weapon(&mut self, weapon: Weapon) {
        self.remove_equipment();
        self.equipment.weapon = Some(weapon);
        self.apply_equipment();
    }

    /// Unequip the current weapon and update stats.
    pub fn unequip_weapon(&mut self) -> Option<Weapon> {
        let old = self.equipment.weapon.take();
        self.remove_equipment();
        self.apply_equipment();
        old
    }

    /// Equip new armor and update stats to include its bonuses.
    pub fn equip_armor(&mut self, armor: Armor) {
        self.remove_equipment();
        self.equipment.armor = Some(armor);
        self.apply_equipment();
    }

    /// Remove the current armor and revert its bonuses.
    pub fn unequip_armor(&mut self) -> Option<Armor> {
        let old = self.equipment.armor.take();
        self.remove_equipment();
        self.apply_equipment();
        old
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecruitmentChallenge {
    pub unit_name: String,
    pub questions: Vec<LoreQuestion>,
    pub required_correct_answers: u32,
    pub player_score: u32,
    pub is_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoreQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer_index: usize,
    pub explanation: String,
}

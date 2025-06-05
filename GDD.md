
# Warhammer 40K Grid RPG – Expanded Game Design Document

> **Note:** Sections marked with **(Added Details)** indicate new or heavily expanded content to cover features not yet implemented (graphics, audio, full equipment/ability systems, UI, etc.).

---

## TECHNICAL IMPLEMENTATION SPECIFICATIONS

### Core Architecture Requirements
```
Platform: Rust with wasm-pack for web deployment
Target: Web browser primary (WASM), native builds for desktop/mobile
Grid System: 2D array-based positioning (recommended 16×12 tiles)
Turn Management: Queue-based initiative system (VecDeque of unit IDs)
State Management: Arc<RwLock<GameState>> for thread-safe mutable state
Persistence: serde JSON serialization for save/load (GameState → JSON)
Rendering: wgpu (WebGPU) for WebAssembly; fallback to HTML5 Canvas 2D if WebGPU unavailable
Input Handling: winit or web-sys event listeners (mouse/tap/keyboard/joystick)
```

---

## DATA MODELS

### Unit Struct Definition (Existing base, implemented)
```rust
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
    pub status_effects: Vec<StatusEffect>,          // **(Added Details)**
    pub animation_state: AnimationState,            // **(Added Details)**
    pub sprite_id: String,                          // **(Added Details)**
    pub is_selected: bool,                          // **(Added Details)**
}
```

- `status_effects`: Track ongoing buffs/debuffs (poison, shield, suppression, etc.).
- `animation_state`: Encapsulates current sprite animation (idle, move, attack, death).
- `sprite_id`: Key into a centralized sprite/texture atlas.
- `is_selected`: Whether UI is highlighting this unit on-screen.

#### Supporting Enums and Structs (Expanded Details)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,
    pub toughness: i32,
    pub agility: i32,
    pub intellect: i32,
    pub willpower: i32,
    pub fellowship: i32,
    pub max_health: i32,    // **(Added Details)**
    pub max_action: u32,    // **(Added Details)**
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitType {
    SpaceMarine,
    Guardsman,
    Commissar,
    TechPriest, // Imperial
    OrkBoy,
    OrkNob,
    Weirdboy,   // Ork
    Cultist,
    ChaosMarine,
    Daemon,     // Chaos
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Faction {
    Imperial,
    Ork,
    Chaos,
}

// **(Added Details)**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// **(Added Details)**
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
    // … etc.
}

// **(Added Details)**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub current_animation: AnimationType,
    pub frame_index: usize,
    pub timer: f32,                  // time accumulator for frame switching
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    Idle,
    Move,
    Attack,
    AbilityCast,
    Death,
}
```

---

### Equipment Data Model (Existing framework + full added details)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    pub accessory_slots: Vec<Accessory>,      // e.g. grenades, prosthetics
}

// **Weapon Struct Definition (Added Details)**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub id: String,                            // Unique equipment ID
    pub name: String,
    pub tier: WeaponTier,
    pub damage: i32,
    pub accuracy: f32,                         // 0.0–1.0 range
    pub range: u32,                            // Attack range in tiles
    pub armor_piercing: Option<f32>,           // % of defense ignored
    pub action_point_cost: u32,
    pub critical_chance: f32,                  // 0.0–1.0
    pub abilities_granted: Vec<AbilityType>,   // Abilities this weapon unlocks
}

// **Weapon Tiers**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponTier {
    Basic,
    Advanced,
    MasterCrafted,
}

// **Armor Struct Definition (Added Details)**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub id: String,
    pub name: String,
    pub tier: ArmorTier,
    pub toughness_bonus: i32,
    pub agility_penalty: i32,
    pub special_properties: Vec<ArmorProperty>,
}

// **Armor Tiers**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorTier {
    Flak,
    Carapace,
    PowerArmor,
}

// **Armor Properties**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorProperty {
    ReactivePlating,         // +5% chance to reduce damage by half
    InoculatedCeramite,      // +10 against poison
    // … etc.
}

// **Accessory Struct Definition (Added Details)**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Accessory {
    Grenade { damage: i32, aoe_radius: u32 },
    Stimpack { heal_amount: i32, cooldown: u32 },
    Medkit { heal_over_time: i32, duration: u32 },
    Bionics { stat_bonus: StatsModifier, duration: u32 },
    // … etc.
}
```

---

### Ability Data Model (Added Details)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: String,
    pub name: String,
    pub ability_type: AbilityType,
    pub description: String,
    pub action_point_cost: u32,
    pub cooldown: u32,                         // Turns between uses
    pub current_cooldown: u32,                 // 0 if ready
    pub range: u32,                            // In tiles
    pub area_of_effect: Option<AreaOfEffect>,  // None = single-target
    pub effect: AbilityEffect,                 // See below
    pub animation: AnimationType,              // Which animation to play
    pub sound_effect_key: String,              // Which audio cue to play
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
    Special,       // Unique scripted effects (e.g., Commissar’s “Inspire”)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AreaOfEffect {
    Cone { radius: u32 },
    Line { length: u32 },
    Circle { radius: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityEffect {
    pub damage: Option<i32>,                   // Some abilities deal damage
    pub healing: Option<i32>,                  // Some heal
    pub buff: Option<StatsModifier>,           // Some buff target
    pub debuff: Option<StatsModifier>,         // Some debuff target
    pub status_applied: Option<EffectType>,    // e.g., Poison, Stun
    pub duration: Option<u32>,                 // How many turns effect lasts
}

// StatsModifier: positive or negative adjustments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsModifier {
    pub strength_mod: i32,
    pub toughness_mod: i32,
    pub agility_mod: i32,
    pub intellect_mod: i32,
    pub willpower_mod: i32,
    pub fellowship_mod: i32,
}
```

---

### Combat System Data (Existing + added details)

```rust
#[derive(Debug, Clone)]
pub struct CombatEncounter {
    pub player_units: Vec<Unit>,
    pub enemy_units: Vec<Unit>,
    pub battlefield: GridMap,
    pub turn_order: TurnQueue,
    pub current_phase: CombatPhase,
    pub environmental_effects: Vec<EnvironmentalEffect>, // **(Added Details)**
    pub camera_state: CameraState,                       // **(Added Details)**
}

#[derive(Debug, Clone)]
pub struct TurnQueue {
    pub initiative: VecDeque<String>, // Unit IDs sorted by initiative roll
    pub current_unit_id: Option<String>,
    pub round_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatPhase {
    Movement,
    Action,
    End,
}

// **Environmental Effects (Added Details)**
#[derive(Debug, Clone)]
pub enum EnvironmentalEffect {
    SmokeCloud { center: Position, radius: u32, turns_remaining: u32 },
    FirePatch { grid_cells: Vec<Position>, damage_per_turn: i32 },
    AcidPool { grid_cells: Vec<Position>, movement_penalty: f32 },
    // … etc.
}

// **Camera State (Added Details)**
#[derive(Debug, Clone)]
pub struct CameraState {
    pub x_offset: f32,
    pub y_offset: f32,
    pub zoom_level: f32,
}
```

- **EnvironmentalEffect**: Allows terrain hazards (smoke blocks vision, fire deals damage).
- **CameraState**: For panning/zooming logic, particularly on larger maps.

---

## GAMEPLAY MECHANICS IMPLEMENTATION

### Grid Movement System (Existing + additional details)

- **Movement Points Calculation:**
  - Each unit’s maximum movement points = `floor(Agility / 2)`.
  - Calculate movement cost per tile:
    - Orthogonal = 1 MP
    - Diagonal = 1.5 MP (rounded up to 2 when subtracting from integer MP)
    - Difficult Terrain (e.g., rubble, shallow water) = +1 MP
    - Hazardous Terrain (e.g., fire) = +2 MP + applies damage if end-of-turn.
- **Pathfinding:**
  - Use A* (manhattan distance heuristic) on the grid.
  - Heuristic adjustment for diagonal: `heuristic = max(Δx, Δy)`.
  - Nodes flagged as “blocked” by obstacles or impassable terrain cannot be included.
- **Line of Sight (LOS):**
  - Bresenham’s line algorithm between attacker’s tile center and target tile center.
  - If any intervening tile is high cover or blocked, apply cover bonus to target’s defense.
- **Positioning Bonuses:**
  - If allied unit is orthogonally adjacent (N/S/E/W), apply +1 bonus to attacker’s hit roll.
  - If enemy is flanked (unit on opposite sides), apply +2 bonus to hit.
- **Movement Animation (Added Details):**
  - On movement, set `unit.animation_state.current_animation = AnimationType::Move`.
  - Interpolate sprite position from tile-center-to-tile-center over `0.1s × tile_cost`.
  - At end of movement, set animation back to `Idle`.

### Combat Resolution Algorithm (Existing + additional details)

```
Attack Resolution (when Action Phase):
1. Verify attacker has enough action_points ≥ Weapon.action_point_cost.
2. Deduct AP: attacker.action_points -= Weapon.action_point_cost.
3. Calculate base hit chance:
   hit_chance = clamp(
       (attacker.current_stats.agility as f32 + weapon.accuracy * 100.0)
       - (defender.current_stats.agility as f32 + cover_bonus * 10.0),
       0.0..=100.0
   )
4. Roll d100: let roll_value = random in 1..=100.
5. If roll_value ≤ hit_chance:
     a. Base damage = weapon.damage + attacker.current_stats.strength.
     b. Effective toughness = defender.current_stats.toughness.
        - If armor is equipped, effective toughness += armor.toughness_bonus.
     c. If weapon.armor_piercing.is_some():
          effective_toughness = (effective_toughness as f32 * (1.0 - armor_piercing)).floor() as i32.
     d. Raw damage = max(1, base_damage - effective_toughness).
     e. Critical check: roll_cri = random in 1..=100.
        If roll_cri ≤ (weapon.critical_chance * 100.0),
          final_damage = raw_damage * 2; else final_damage = raw_damage.
     f. Subtract from defender.health_points: defender.health_points -= final_damage.
     g. Trigger hit animation & sound:
          - attacker.animation_state.current_animation = AnimationType::Attack.
          - Play audio: AudioManager::play_sound_effect(weapon_sound_key).
     h. If defender.health_points ≤ 0, queue defender for death phase (remove next turn), play death animation.
   Else:
     - Missed shot: show “miss” floating text, play “weapon_miss” sound effect.
```

- **Multi-Target/Area Abilities (Added Details):**
  - If ability has `area_of_effect = Some(...)`, gather all units within that AOЕ pattern from the target point.
  - Loop through each unit and apply the same damage/buff/debuff logic.
- **Ability Use Logic (Added Details):**
  1. Verify `ability.current_cooldown == 0` and `action_points ≥ ability.action_point_cost`.
  2. Deduct AP and set `ability.current_cooldown = ability.cooldown`.
  3. Determine all valid target tiles based on `range` and `area_of_effect`.
  4. Show targeting cursor (UI) over valid tiles; when player confirms target, apply effect.
  5. Subtract HP or apply status modifiers as defined in `AbilityEffect`.
  6. Trigger animation (`ability.animation`) and sound (`sound_effect_key`).
  7. On end of unit’s turn, decrement all `current_cooldown` by 1 (if > 0).

### Recruitment Mechanic Implementation (Existing)

```rust
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
```

- **Added Details:**
  - When the player interacts with a Recruitment NPC on the map, pause grid and display a pop-up window (UI) with `questions`.
  - The UI shows one question at a time; player selects an answer (via click/tap).
  - On correct, increment `player_score`, play “correct_answer” sound; on incorrect, play “wrong_answer” sound.
  - After all questions, if `player_score ≥ required_correct_answers`, set `is_completed = true` and add the new unit to the player’s roster:
    ```rust
    if challenge.player_score >= challenge.required_correct_answers {
        let new_unit = generate_unit_from_template(unit_name);
        player_units.push(new_unit);
    }
    ```

---

## CONTENT SPECIFICATIONS

### Imperial Units (Playable)

#### **Tier 1 (Starting Units)**
- **Guardsman** (Based on existing + added details)
  - ID: `"guardsman_t1"`
  - Name: `"Imperial Guardsman"`
  - Level: 1
  - Base Stats: 
    - HP: 25, Str 3, Tgh 3, Agi 3, Int 2, Wil 2, Fel 2
    - Max HP = 25, Max AP = floor(3/2) = 1
  - Equipment:
    - Weapon: Lasgun (`damage=2, accuracy=0.85, range=5, ap_cost=1, cri=0.05`)
    - Armor: Flak Armor (`toughness_bonus=1, agility_penalty=0`)
  - Abilities:
    - `RangedAttack`: Use Lasgun
    - `TakeCover`: AbilityType::Buff, `buff = StatsModifier { agility_mod: 1, … }`, `ap_cost=1`, `cooldown=2`
  - Sprite: `"guardsman_sprite_01"`
  - Portrait: `assets/sprites/portraits/guardsman.png`

- **Acolyte** (Tier 1)
  - ID: `"acolyte_t1"`
  - Name: `"Imperial Acolyte"`
  - Level: 1
  - Base Stats: HP: 20, Str 2, Tgh 2, Agi 3, Int 4, Wil 4, Fel 3
  - Equipment:
    - Weapon: Stub Pistol (`damage=1, accuracy=0.80, range=4, ap_cost=1, cri=0.03`)
    - Armor: Cloth Robe (`toughness_bonus=0, agility_penalty=0, special=InoculatedCeramite`)
  - Abilities:
    - `MinorHeal`: AbilityType::Healing, `healing=5`, `ap_cost=1`, `cooldown=3`, `animation=Healing`, `sound="heal_sfx"`
    - `LoreInsight`: AbilityType::Buff, `buff = StatsModifier { int_mod: 1, … }`, `ap_cost=1`, `cooldown=2`
  - Sprite: `"acolyte_sprite_01"`

#### **Tier 2 (Mid-Game Recruits)**
- **Space Marine**
  - ID: `"space_marine_t2"`
  - Name: `"Space Marine Tactical Marine"`
  - Level: 5
  - Base Stats: HP: 50, Str 5, Tgh 5, Agi 4, Int 3, Wil 3, Fel 2
  - Equipment:
    - Weapon: Bolter (`damage=4, accuracy=0.80, range=6, ap_cost=2, cri=0.10, armor_piercing=0.20`)
    - Armor: Power Armor (`toughness_bonus=3, agility_penalty=-1`)
  - Abilities:
    - `SuppressiveFire`: AbilityType::Debuff, `debuff = StatsModifier { agi_mod: -1, …}`, `ap_cost=2`, `cooldown=3`, `range=5`, `aoe=Line { length=3 }`
    - `FragGrenade`: AbilityType::AreaAttack, `damage=8`, `range=4`, `aoe=Circle { radius=1 }`, `ap_cost=2`, `cooldown=4`
  - Sprite: `"space_marine_sprite_01"`

- **Tech-Priest**
  - ID: `"tech_priest_t2"`
  - Name: `"Adeptus Mechanicus Tech-Priest"`
  - Level: 5
  - Base Stats: HP: 35, Str 3, Tgh 3, Agi 3, Int 5, Wil 4, Fel 1
  - Equipment:
    - Weapon: Mechadendrite (melee, `damage=3, accuracy=0.75, ap_cost=1, cri=0.05`)
    - Armor: Carapace Armor (`toughness_bonus=2, agility_penalty=-1`)
    - Accessory: Servo-Smith Forge (crafting device for on-map repairs)
  - Abilities:
    - `Repair`: AbilityType::Buff, `healing=15`, `ap_cost=2`, `cooldown=3`, `range=1`
    - `Overclock`: Buff/Temp AP regen, `buff = StatsModifier { agility_mod: 2, … }`, `ap_cost=1`, `cooldown=4`
  - Sprite: `"tech_priest_sprite_01"`

#### **Tier 3 (Late-Game Recruits)**
- **Space Marine Sergeant**
  - ID: `"space_marine_sergeant_t3"`
  - Name: `"Space Marine Sergeant"`
  - Level: 10
  - Base Stats: HP: 65, Str 6, Tgh 5, Agi 4, Int 3, Wil 4, Fel 3
  - Equipment:
    - Weapon: Power Sword (`damage=6, accuracy=0.75, ap_cost=2, cri=0.15, melee_range=1`)
    - Armor: Power Armor Mk II (`toughness_bonus=4, agility_penalty=-1, special=ReactivePlating`)
  - Abilities:
    - `RallyTroops`: Buff, `buff = StatsModifier { str_mod: 2, tgh_mod: 1 }`, `ap_cost=2`, `cooldown=5`, `range=2`, `aoe=Circle { radius=1 }`
    - `PrecisionStrike`: Single-target high-damage, `damage=10`, `accuracy=0.95`, `ap_cost=3`, `cooldown=3`
  - Sprite: `"sergeant_sprite_01"`

- **Commissar**
  - ID: `"commissar_t3"`
  - Name: `"Commissar Yarrick"` (or generic commissar template)
  - Level: 10
  - Base Stats: HP: 45, Str 4, Tgh 4, Agi 3, Int 2, Wil 6, Fel 5
  - Equipment:
    - Weapon: Bolt Pistol (`damage=3, accuracy=0.85, range=4, ap_cost=1, cri=0.10`)
    - Weapon: Chainsword (`damage=5, accuracy=0.80, ap_cost=2, cri=0.15, melee_range=1`)
    - Armor: Carapace Armor Mk II (`toughness_bonus=3, agility_penalty=-1`)
  - Abilities:
    - `Inspire`: Buff morale, `buff = StatsModifier { fel_mod: 2, wil_mod: 2 }`, `ap_cost=1`, `cooldown=4`, `range=3`
    - `SummaryExecute`: Single-target %chance to insta-kill if HP ≤ 20%, `ap_cost=2`, `cooldown=6`, `range=1`
  - Sprite: `"commissar_sprite_01"`

---

### Enemy Factions (Existing + added details)

#### **Ork Units**
- **Ork Boy** (Tier 1 enemy)
  - ID: `"ork_boy_t1"`
  - Name: `"Ork Boy"`
  - Level: 3
  - Stats: HP 30, Str 4, Tgh 4, Agi 3, Int 1, Wil 1, Fel 1
  - Equipment:
    - Weapon: Choppa (`damage=3, accuracy=0.70, ap_cost=1, cri=0.05, melee_range=1`)
    - Weapon: Slugga (`damage=2, accuracy=0.70, range=3, ap_cost=1, cri=0.05`)
    - Armor: Makeshift Armor (`toughness_bonus=1, agility_penalty=0`)
  - Abilities:
    - `Charge`: Move up to 3 tiles in straight line and melee attack on end tile, `ap_cost=2`, `cooldown=3`
  - AI Behavior:
    - Aggro Range = 5 tiles
    - If HP ≥ 15, use ranged Slugga; else, use Charge or Choppa.
  - Sprite: `"ork_boy_sprite_01"`

- **Ork Nob** (Tier 2 enemy)
  - ID: `"ork_nob_t2"`
  - Name: `"Ork Nob"`
  - Level: 6
  - Stats: HP 45, Str 5, Tgh 5, Agi 3, Int 1, Wil 1, Fel 1
  - Equipment:
    - Weapon: Power Klaw (`damage=6, accuracy=0.75, ap_cost=2, cri=0.10, melee_range=1`)
    - Armor: Scrap Armor (`toughness_bonus=2, agility_penalty=-1, special=ReactivePlating`)
  - Abilities:
    - `Stomp`: AOE melee, `damage=8, range=1, aoe=Circle { radius=1 }, ap_cost=2, cooldown=4`
    - `Roar`: Debuff morale, `debuff = StatsModifier { fel_mod: -2 }, ap_cost=1, cooldown=3, range=2`
  - Sprite: `"ork_nob_sprite_01"`

- **Weirdboy** (Tier 2 support)
  - ID: `"weirdboy_t2"`
  - Name: `"Weirdboy"`
  - Level: 6
  - Stats: HP 25, Str 2, Tgh 3, Agi 3, Int 5, Wil 5, Fel 1
  - Equipment:
    - Weapon: Glyph Scanner (psychic focus)
    - Armor: Tunic of Da Good Moons (`toughness_bonus=1, agility_penalty=0, special=InoculatedCeramite`)
  - Abilities:
    - `Smite`: Psychic Blast, `damage=10`, `range=6, aoe=None, ap_cost=3, cooldown=4`
    - `Waaagh! Flare`: Summon random Ork reinforcements within 3 tiles, `ap_cost=3, cooldown=6`
  - AI Behavior:
    - Maintain distance ≥ 4 tiles, cast Smite on highest-HP target; else, summon reinforcements.
  - Sprite: `"weirdboy_sprite_01"`

#### **Chaos Units**
- **Cultist** (Tier 1 enemy)
  - ID: `"cultist_t1"`
  - Name: `"Chaos Cultist"`
  - Level: 2
  - Stats: HP 20, Str 3, Tgh 3, Agi 2, Int 2, Wil 1, Fel 1
  - Equipment:
    - Weapon: Autogun (`damage=2, accuracy=0.75, range=5, ap_cost=1, cri=0.05`)
    - Armor: Shabby Robe (`toughness_bonus=0, agility_penalty=0`)
  - Abilities:
    - `ChaoticPrayer`: Debuff target’s willpower (`debuff StatsModifier { wil_mod: -1 }`), `ap_cost=1, cooldown=3, range=4`
  - AI Behavior:
    - If < 50% HP, use ChaoticPrayer on nearest player unit; else, shoot with Autogun.
  - Sprite: `"cultist_sprite_01"`

- **Chaos Space Marine** (Tier 2 enemy)
  - ID: `"chaos_marine_t2"`
  - Name: `"Chaos Space Marine"`
  - Level: 6
  - Stats: HP 50, Str 5, Tgh 5, Agi 4, Int 2, Wil 2, Fel 1
  - Equipment:
    - Weapon: Bolter (same stats as player’s Bolter)
    - Armor: Chaos Carapace Armor (`toughness_bonus=3, agility_penalty=-1, special=ReactivePlating`)
  - Abilities:
    - `DevastatingBolt`: Ranged high-damage, `damage=8, accuracy=0.75, ap_cost=2, cooldown=3, range=6`
    - `MutatedForm`: Temporary buff, `buff = StatsModifier { str_mod: 2, tgh_mod: 2 }`, `ap_cost=1, cooldown=5`
  - AI Behavior:
    - If has mutation buff available, cast it when HP < 30% then use melee; else, use DevastatingBolt.
  - Sprite: `"chaos_marine_sprite_01"`

- **Lesser Daemon** (Tier 2 elite enemy)
  - ID: `"lesser_daemon_t2"`
  - Name: `"Daemonic Host"`
  - Level: 7
  - Stats: HP 35, Str 4, Tgh 4, Agi 4, Int 4, Wil 4, Fel 2
  - Equipment:
    - (No physical weapons; pure psychic/punch)
  - Abilities:
    - `DaemonicResilience`: Passive: 20% chance to ignore incoming damage.
    - `HellfireBolt`: Ranged, `damage=9, accuracy=0.80, ap_cost=2, cooldown=3, range=5`
    - `Possession`: Debuff target, controlling their turn next round; `ap_cost=3, cooldown=6, range=4, aoe=None`
  - AI Behavior:
    - Use DaemonicResilience passively.
    - Cast HellfireBolt if AP≥2 and target in range, else or if HP < 50%, attempt Possession on highest-level player unit.
  - Sprite: `"lesser_daemon_sprite_01"`

---

### Chapter/Area Structure (Expanded Details)

1. **Chapter 1: Hive World Uprising**
   - **Environment:** Industrial sector maps (12×16 tiles). Contains high crates (2-tile high) and pipes (impassable).
   - **Primary Enemies:** Chaos Cultists (x6), 2 Chaos Space Marines (patrol).
   - **Recruitable Units:** 2 Guardsmen, 1 Acolyte appear after interrogation puzzle.
   - **Boss Fight:** Chaos Cult Leader (unique mini-boss):
     - HP 100, Str 5, Tgh 5, Agi 3, Wil 5, Fel 4
     - Abilities: “Dark Invocation” (AOE debuff), “Blade Flurry” (multi-hit melee).
   - **Special Mechanics:**
     - Conveyor belts move units 1 tile in designated direction at end of turn.
     - Industrial cranes drop crates as dynamic obstacles (triggered when units step on certain tiles).
   - **Objectives:**
     1. Survive 5 enemy waves.
     2. Rescue captured Guardsmen by solving a “lock-picking” minigame (UI: rotating dial overlay).
     3. Defeat Cult Leader.

2. **Chapter 2: Orbital Station Assault**
   - **Environment:** Space station corridors (10×14). Zero-gravity zones (move costs 2 MP per tile; if stationary for >1 turn, suffer -1 health per turn).
   - **Primary Enemies:** Mixed Cultists (x4), Chaos Space Marines (x2).
   - **Recruitable Units:** 
     - Tech-Priest: Hidden in maintenance bay; unlock by hacking terminal (UI: code mini-game).
     - Space Marine: Rescue after “airlock sealing” puzzle (UI: click sequence).
   - **Boss Fight:** Possessed Tech-Heretek
     - HP 120, Int 6, Wil 6, Str 4, Tgh 4
     - Abilities: “Electroshock” (stuns 1 tile radius), “Corrupted Hack” (debuffs player AP regen).
   - **Special Mechanics:**
     - Airlocks: Opening toggles gravity zones—player must manage movement accordingly.
     - Terminals: “Hack” UI uses a combination of button presses to disable traps.
   - **Objectives:**
     1. Secure docking bay (defeat enemies).
     2. Hack mainframe (complete hack mini-game under time limit).
     3. Exterminate Possessed Tech-Heretek.

3. **Chapter 3: Jungle Death World**
   - **Environment:** Dense forest terrain (14×18). Tall foliage (2-tile cover), quicksand patches (trap: if unit ends turn on quicksand, lose 1 AP next turn).
   - **Primary Enemies:** Ork warband (Ork Boys x8, Ork Nob x2, Weirdboy x1).
   - **Recruitable Units:**
     - Space Marine: Hidden in crashed drop pod—puzzle: align satellite beacon (UI grid puzzle).
     - Guardsman Veteran: Captured behind enemy lines; rescue by stealth (avoid line-of-sight tiles, indicated by shading).
   - **Boss Fight:** Ork Nob Warboss
     - HP 150, Str 7, Tgh 6, Agi 3, Wil 2
     - Abilities: “Warcry” (AOE buff to Ork allies), “Stomping Charge” (knockback effect).
   - **Special Mechanics:**
     - Foliage: Units in tall foliage receive +2 to dodge and are hidden unless adjacent.
     - Quicksand: Represented by darker tiles; stepping on them triggers trap.
   - **Objectives:**
     1. Navigate through jungle avoiding quicksand traps.
     2. Eliminate Weirdboy first to prevent reinforcements.
     3. Defeat Warboss to clear path.

4. **Chapter 4: Fortress Siege**
   - **Environment:** Fortified positions (16×20). Includes turret emplacements (enemy turrets have automated fire each enemy turn), barricades (impassable).
   - **Primary Enemies:** Large Ork army: Ork Boys x10, Ork Nobs x3, Ork vehicles (Looted Wagon turret) (vehicle occupies 2×2 tiles).
   - **Recruitable Units:**
     - Commissar: Found in command bunker; must convince via dialogue choice (branching conversation UI).
     - Space Marine Sergeant: Escort mission from reinforcement pod.
   - **Boss Fight:** Ork Warboss with Warbike  
     - HP 200, Str 7, Tgh 7, Agi 4
     - Abilities: “Bike Charge” (charge across 3 tiles in straight line, deal damage to all units in path), “Raining Shells” (AOE ranged lob).
   - **Special Mechanics:**
     - Turrets: Each turn, turrets fire on random player unit within 6 tiles (roll-to-hit like normal ranged).
     - Siege Ladders: Cover-providing objects that can be destroyed (weapon: rocket-propelled grenade, triggered by ability).
   - **Objectives:**
     1. Disable all turrets (destroy by moving to their adjacent tile and spending AP to “sabotage”).
     2. Breach fortress gates (complete “lock” minigame under fire).
     3. Defeat Warboss.

5. **Chapter 5: Chaos Stronghold**
   - **Environment:** Daemonic corruption zones (18×22). Corrupted ground deals 2 damage per turn; shadowy areas (tiles) grant Chaos units +10% dodge.
   - **Primary Enemies:** High-tier Chaos forces: Chaos Space Marines x4, Lesser Daemons x2, Greater Daemon (mid-boss).
   - **Final Recruits:** 
     - Elite Imperial units: Grand Master Space Marine and Chaplain appear after defeating Greater Daemon (automatically join).
   - **Boss Fight:** Chaos Lord with Daemonic Allies  
     - HP 250, Str 8, Tgh 8, Agi 5, Int 6
     - Abilities: “Warp Storm” (random meteor strikes, deals area damage), “Daemonic Reinforcements” (summon 2 Lesser Daemons), “Mark of Chaos” (mass debuff).
   - **Special Mechanics:**
     - Corruption Over Time: At end of every round, each unit on corrupted tile loses 2 HP; Chaos units receive +1 HP regen if on corrupted tiles.
     - Shadow Tiles: Appear/disappear every 3 rounds; need to track with timer.
   - **Objectives:**
     1. Navigate through corruption zones minimizing damage (use abilities or items to cleanse).
     2. Defeat Greater Daemon to open Chaos Lord’s portal.
     3. Defeat Chaos Lord to complete campaign.

---

## PROGRESSION SYSTEMS

### Experience and Leveling

```rust
pub struct ExperienceSystem;

impl ExperienceSystem {
    const LEVEL_THRESHOLDS: &'static [u32] = &[
        0, 100, 250, 450, 700, 1000, 1350, 1750, 2200, 2700, 3250
    ];

    pub fn gain_experience(unit: &mut Unit, amount: u32) {
        unit.experience += amount;
        Self::check_level_up(unit);
    }

    pub fn check_level_up(unit: &mut Unit) {
        let new_level = Self::calculate_level(unit.experience);
        if new_level > unit.level {
            Self::level_up(unit, new_level);
        }
    }

    fn calculate_level(experience: u32) -> u32 {
        Self::LEVEL_THRESHOLDS
            .iter()
            .position(|&threshold| experience < threshold)
            .map(|pos| pos as u32)
            .unwrap_or(Self::LEVEL_THRESHOLDS.len() as u32)
    }

    fn level_up(unit: &mut Unit, new_level: u32) {
        unit.level = new_level;
        // Apply stat increases based on unit type
        Self::apply_level_bonuses(unit);
        // Replenish HP and AP on level-up
        unit.current_stats.max_health += Self::health_increase(unit);
        unit.current_stats.max_action += Self::action_increase(unit);
        unit.health_points = unit.current_stats.max_health;
        unit.action_points = unit.current_stats.max_action;
        // Play "level_up" sound, show floating text “Level Up!”
    }

    fn apply_level_bonuses(unit: &mut Unit) {
        match unit.unit_type {
            UnitType::SpaceMarine => {
                unit.base_stats.strength += 1;
                unit.base_stats.toughness += 1;
                unit.base_stats.max_health += 5;
            }
            UnitType::Guardsman => {
                unit.base_stats.agility += 1;
                unit.base_stats.max_health += 3;
            }
            UnitType::Commissar => {
                unit.base_stats.willpower += 1;
                unit.base_stats.fellowship += 1;
                unit.base_stats.max_health += 4;
            }
            UnitType::TechPriest => {
                unit.base_stats.intellect += 1;
                unit.base_stats.willpower += 1;
                unit.base_stats.max_health += 4;
            }
            // … other unit types
            _ => {}
        }
    }

    fn health_increase(unit: &Unit) -> i32 {
        // Example: +5 HP for each level for Marines, +3 for Guardsmen, etc.
        match unit.unit_type {
            UnitType::SpaceMarine => 5,
            UnitType::Guardsman => 3,
            _ => 4,
        }
    }

    fn action_increase(unit: &Unit) -> u32 {
        // AP = floor(Agility / 2), so AP updates automatically when agility increases
        (unit.base_stats.agility / 2).max(1) as u32
    }
}
```

- **Added Details:**
  - After leveling up, present an interactive “Level Up” screen (UI panel) where the player can choose between two random bonuses (e.g., +1 Str, +1 Agi, or unlock a new ability from a small pool).
  - The random bonuses should respect unit type archetypes (e.g., Tech-Priest more likely to get +Int).

---

### Equipment System (Existing framework + full added details)

- **Item Database / Loot Tables (Added Details):**
  - A global `HashMap<String, ItemTemplate>` loaded at runtime (JSON or TOML).
    ```rust
    pub struct ItemTemplate {
        pub id: String,
        pub name: String,
        pub item_type: ItemType,
        pub tier: u8,
        pub base_damage: Option<i32>,
        pub base_toughness: Option<i32>,
        pub stat_modifiers: Option<StatsModifier>,
        pub abilities_unlocked: Vec<AbilityType>,
        pub gold_value: u32,
        pub sprite_key: String,
    }

    pub enum ItemType {
        Weapon,
        Armor,
        Accessory,
    }
    ```
  - When generating loot from enemies, roll on a weighted table keyed by chapter/difficulty.
  - Guarantee at least 25% drop chance per enemy kill; scale by enemy tier.

- **Inventory & Equipment UI (Added Details):**
  - **Inventory Screen Layout:**
    ```
    ┌─────────────────────────────────────────────┐
    │ Inventory (Grid 5×6 slots)                 │ <- scrollable if > slots
    │ ┌─────────────┐  ┌──────────┐  ┌────────┐   │
    │ │ [Slot 1]    │  │ [Slot 2] │  │ [Slot 3]│   │
    │ │  Icon + Qty │  │ Icon + Q │  │ Icon   │   │
    │ └─────────────┘  └──────────┘  └────────┘   │
    │  …                                        │
    │ Selected Item Details → [Name, Stats, Desc] │
    │ ┌───────────────────────────────────────┐  │
    │ │ Equip / Use / Drop / Compare Buttons │  │
    │ └───────────────────────────────────────┘  │
    └─────────────────────────────────────────────┘
    ```
  - **Equipment Screen Layout (for a given Unit):**
    ```
    ┌────────────────────┬─────────────────────┐
    │ Character Model    │ Equipment Slots:    │
    │ (Animated Sprite)  │  [Weapon]           │
    │                    │  [Armor]            │
    │                    │  [Accessory #1]     │
    │                    │  [Accessory #2]     │
    │                    │  [Accessory #3]     │
    │                    │                     │
    │                    │  Stats (with mods)  │
    └────────────────────┴─────────────────────┘
    ```
  - Drag-and-drop or click-to-equip interactions:
    1. Player selects a unit → opens Equipment Screen.
    2. Clicking an “Inventory” item of type “Weapon” in slots filters to show only “Weapon” items.
    3. Click on desired item → automatically populates `unit.equipment.weapon = Some(weapon)`.
    4. UI recalculates `unit.current_stats` = `unit.base_stats` + all `stat_modifiers` from equipment + any accessory modifiers.

- **Equipment Effects Application:**
  - Whenever a unit’s equipment changes:
    1. Recompute `unit.current_stats = unit.base_stats + equipped_armor_modifiers + accessory_modifiers`.
    2. Recompute `unit.current_stats.max_health` and `max_action` if relevant.
  - On weapon equip:
    - If `weapon.abilities_granted` is non-empty, add those abilities to `unit.abilities` (if not already present).
  - Ensure equipment durability if desired:
    - Add `durability: u32` field to `Weapon`/`Armor`, decrement on use, 0 = broken (force unequip).

---

## GRAPHICS SYSTEM IMPLEMENTATION (Added Details)

### Rendering Pipeline

1. **Initialization (WASM / Native)**
   - On startup, initialize `wgpu::Instance` (or `web-sys` Canvas2D fallback).
   - Load all sprite sheets and tile maps as textures (PNG/Atlas).
     ```rust
     let sprite_atlas = Texture::from_bytes(&device, &queue, include_bytes!("spritesheet.png")).unwrap();
     let tile_map = Texture::from_bytes(&device, &queue, include_bytes!("tileset.png")).unwrap();
     ```
   - Create vertex and index buffers for rendering quads (2D rectangles).
   - Load a JSON descriptor (e.g., `sprites.json`) mapping `"sprite_id" -> Rect { x, y, width, height }`.
2. **Frame Loop**
   - Each frame:
     1. Clear target (color and depth if using wgpu).
     2. Iterate through all visible tiles:
        - Compute world-coordinate position: `(tile_x as f32 * tile_size, tile_y as f32 * tile_size)`.
        - Issue draw call: sample the tile’s texture from tile atlas; draw at that position.
     3. Iterate through all units:
        - Compute screen position from `unit.grid_position`.
        - Select appropriate `unit.animation_state.current_animation`, `frame_index`.
        - Look up sprite rectangle in atlas (e.g., `"guardsman_idle_0"` vs `"guardsman_move_1"`).
        - Issue draw call layering on top of tiles.
     4. Draw UI layers (HUD, panels, floating text).
   - Present frame.
3. **Tile & Layer Prioritization**
   - **Tile Layer 0**: Ground tiles (floor, grass, metal plating).
   - **Tile Layer 1**: Obstacles (crates, trees, walls).
   - **Unit Layer 2**: Units (player and enemy) – draw in ascending `grid_position.y` so ones “further down” overlap properly.
   - **Projectile/Effect Layer 3**: Bullets, laser beams, particle effects (explosions).
   - **UI Layer 4**: Health bars, selection boxes, floating damage numbers.
   - **Overlay Layer 5**: Highlight grid (movement range, target indicators).
4. **Animation Management**
   - Each `Unit.animation_state.timer` increments by `delta_time` each frame; when ≥ `frame_duration` (e.g., 0.1 s), advance `frame_index += 1`, wrap if at end of animation sequence.
   - When action completes (e.g., movement path is exhausted), set animation back to `Idle`.
5. **Sprite & Tile Atlases**
   - **sprites.json** example entry:
     ```json
     {
       "guardsman_idle_0": { "x": 0,   "y": 0,   "w": 32, "h": 32 },
       "guardsman_idle_1": { "x": 32,  "y": 0,   "w": 32, "h": 32 },
       "guardsman_move_0": { "x": 0,   "y": 32,  "w": 32, "h": 32 },
       // … etc.
     }
     ```
   - **tiles.json** entry example:
     ```json
     {
       "grass":   { "x": 0,   "y": 0,   "w": 64, "h": 64 },
       "metal":   { "x": 64,  "y": 0,   "w": 64, "h": 64 },
       "crates":  { "x": 128, "y": 0,   "w": 64, "h": 64 },
       // … etc.
     }
     ```

### Physics / Collision (Grid-Based)

- No continuous physics; all movement is tile-based.  
- For VFX like projectiles or AoE indicators, simply interpolate from center of tile A to tile B using linear interpolation over time; on arrival, spawn particle effect.

### Particle Effects (Added Details)

- **ParticleSystem** struct:
  ```rust
  pub struct ParticleSystem {
      pub particles: Vec<Particle>,
  }

  pub struct Particle {
      pub position: Vec2,         // float-based world coords
      pub velocity: Vec2,
      pub lifetime: f32,          // seconds until expiration
      pub sprite_rect: Rect,      // sub-rect in atlas for particle
      pub color: [f32; 4],        // RGBA
      pub size: Vec2,             // width/height in pixels
  }
  ```
- **Use Cases:**
  - **Muzzle Flash:** Spawn particles at firing position; fade out over 0.2 s.
  - **Explosion:** On grenade detonation, spawn radial particles with random velocities.
  - **Healing Aura:** Small green particles swirling around unit when healing.
- Each frame, update all particles:  
  ```rust
  for p in &mut particles {
      p.position += p.velocity * delta_time;
      p.lifetime -= delta_time;
      p.color[3] = (p.lifetime / initial_lifetime).clamp(0.0, 1.0); // fade-out alpha
  }
  particles.retain(|p| p.lifetime > 0.0);
  ```

---

## AUDIO IMPLEMENTATION (Existing code + added integration details)

```rust
use std::collections::HashMap;

pub struct AudioManager {
    sound_effects: HashMap<&'static str, &'static str>,
    background_music: HashMap<&'static str, &'static str>,
    volume_settings: AudioSettings,     // **(Added Details)**
}

pub struct AudioSettings {
    pub master_volume: f32,  // 0.0–1.0
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub voice_volume: f32,
}

impl AudioManager {
    pub fn new() -> Self {
        let mut sound_effects = HashMap::new();
        sound_effects.insert("laser_shot", "assets/audio/laser.wav");
        sound_effects.insert("bolter_fire", "assets/audio/bolter.wav");
        sound_effects.insert("melee_hit", "assets/audio/sword.wav");
        sound_effects.insert("unit_death", "assets/audio/death.wav");
        sound_effects.insert("level_up", "assets/audio/levelup.wav");
        sound_effects.insert("button_click", "assets/audio/button.wav");
        sound_effects.insert("heal", "assets/audio/heal.wav");
        sound_effects.insert("grenade_explosion", "assets/audio/explosion.wav");
        // … other SFX

        let mut background_music = HashMap::new();
        background_music.insert("combat", "assets/audio/combat_theme.ogg");
        background_music.insert("exploration", "assets/audio/exploration_theme.ogg");
        background_music.insert("recruitment", "assets/audio/recruitment_theme.ogg");
        background_music.insert("main_menu", "assets/audio/menu_theme.ogg");

        Self {
            sound_effects,
            background_music,
            volume_settings: AudioSettings {
                master_volume: 1.0,
                sfx_volume: 0.8,
                music_volume: 0.5,
                voice_volume: 1.0,
            },
        }
    }

    pub fn play_sound_effect(&self, effect_name: &str) {
        if let Some(path) = self.sound_effects.get(effect_name) {
            // Web Audio API (WASM): create AudioBufferSourceNode, set volume * sfx_volume * master_volume, play
            // Native: use rodio or cpal to play.
        }
    }

    pub fn play_background_music(&self, track_name: &str) {
        if let Some(path) = self.background_music.get(track_name) {
            // Stop any currently playing BGM, then load new AudioBuffer, set volume * music_volume * master_volume, loop.
        }
    }

    pub fn set_volume(&mut self, channel: AudioChannel, volume: f32) {
        match channel {
            AudioChannel::Master => self.volume_settings.master_volume = volume.clamp(0.0, 1.0),
            AudioChannel::SFX    => self.volume_settings.sfx_volume = volume.clamp(0.0, 1.0),
            AudioChannel::Music  => self.volume_settings.music_volume = volume.clamp(0.0, 1.0),
            AudioChannel::Voice  => self.volume_settings.voice_volume = volume.clamp(0.0, 1.0),
        }
    }
}

pub enum AudioChannel {
    Master,
    SFX,
    Music,
    Voice,
}
```

- **Added Details:**
  - **Event-Driven Triggers:**
    - **On Unit Spawn**: `AudioManager::play_sound_effect("spawn_unit");`
    - **On Attack**: within `CombatSystem`, after computing hit/miss, call the appropriate SFX (`"bolter_fire"`, `"melee_hit"`, `"laser_shot"`).
    - **On Ability Cast**: each `Ability` has `sound_effect_key`; call `AudioManager::play_sound_effect(ability.sound_effect_key)`.
    - **On Damage**: if damage > 0, play `"damage_hit"` SFX; if damage > 10, play `"heavy_hit"` SFX.
    - **On Death**: `AudioManager::play_sound_effect("unit_death")`, then fade out sprite.
    - **On Menu Navigation**: `AudioManager::play_sound_effect("button_click")`.
  - **Music Transitions:**
    - When entering combat, crossfade from `exploration.ogg` to `combat_theme.ogg` over 1 second.
    - On entering recruitment screen, fade to `recruitment_theme.ogg`.
    - On returning to world map, fade to `exploration.ogg`.
  - **Volume Settings Menu (UI):**
    - Sliders for Master, Music, SFX, Voice (range 0–100%).  
    - When player adjusts slider, call `AudioManager::set_volume(...)` immediately.

---

## USER INTERFACE SPECIFICATIONS

### Screen Layout Requirements (Existing + added details)

```
Combat Screen Layout:
┌────────────────────────────────────────────────────┐
│ Top Bar (5% height):                               │
│  - Turn Order Display (icons with HP bars)         │
│  - Round # indicator                                │
│  - End Turn button                                  │
│                                                    │
│ ┌───────────────────────────┬────────────────────┐ │
│ │ Grid Battlefield (70%)    │ Unit Info Panel    │ │
│ │ - Tiles rendered under AI  │  (15% width)       │ │
│ │ - Units (animated sprites)│  ┌───────────────┐ │ │
│ │ - Cover/highlight overlay │  │ Selected Unit  │ │ │
│ │ - Particle effects        │  │ Portrait (64×64)│ │ │
│ │ - Floating text (damage)  │  │ Name/Level/HP   │ │ │
│ │                           │  │ AP, Status FX   │ │ │
│ │                           │  │ Equipped Icon(s)│ │ │
│ │                           │  │ Quick Action Buttons (Abilities) │
│ │                           │  └───────────────┘ │ │
│ └───────────────────────────┴────────────────────┘ │
│                                                    │
│ Bottom Bar (10% height):                            │
│  - Action Buttons: Move, Attack, Ability, Wait      │
│  - Contextual Buttons: (e.g., “Use Item”, “Target”) │
│  - Mini-map toggle (if map > viewport)              │
│                                                    │
└────────────────────────────────────────────────────┘
```

- **Turn Order Display:**
  - Horizontal row of unit portrait icons (with small HP bars underneath).
  - Current active unit highlighted (glow effect).
  - Clicking on other portrait scrolls camera to that unit.
- **Unit Info Panel (Added Details):**
  - Shows equipped weapon icon; clicking opens “Equipment” sub-panel.
  - Shows ability icons (on cooldown tinted grayscale with cooldown timer overlay).
  - Shows mouse-over tooltips for each stat (hover → detailed stat breakdown).
- **Floating Damage Text:**
  - Whenever a unit takes damage/healing, spawn a floating text object above unit:  
    - If damage: red text “–X” that floats up 16 pixels over 0.5 s and fades out.
    - If heal: green text “+X”.

### Menu Hierarchy (Existing + added details)

```
Main Menu → [Button: New Game] [Button: Load Game] [Button: Options] [Button: Quit]
   ↳ Options → [Audio Settings] [Video Settings] [Controls] [Keybindings]

Load Game:
   ↳ List of Save Slots (thumbnail + date/time stamp) → [Load] [Delete]

New Game:
   ↳ Character Creation Screen (select starting unit templates? minimal, since pre-set squads)

Game Screen (Top Bar):
   ↳ Combat → (Active when in battle)
       ↳ Options (Pause, Audio, Video, Controls)
   ↳ Party Management → (Outside Combat)
       ↳ Unit Roster (grid of unit cards; click to view details)
       ↳ Unit Level Up (when applicable; interactive “choose bonus” UI)
   ↳ Equipment → (for selected unit via Party Management or in-combat “Inspect”)
       ↳ Inventory interface (as above)
   ↳ Ability → (view/assign ability loadouts; only outside combat)
   ↳ Save → (serialize `GameState` to JSON, prompt confirmation popup)
```

### Input Controls (Added Details)

- **Grid Navigation:**
  - **Mouse (PC/Web):** Click on tile to move or target attack.
  - **Touch (Mobile):** Tap tile; long-press to bring up context menu (move/attack/ability).
  - **Keyboard (PC):** Arrow keys or WASD to move a selection cursor; Enter to confirm.
- **Unit Selection:**
  - **Mouse/Touch:** Click/tap on unit’s sprite or on their icon in turn order.
- **Action Confirmation:**
  - **Double-Click** on target; or click “Confirm” button in bottom bar.
- **Camera Control:**
  - **Mouse Drag** (PC) or **Two-Finger Swipe** (Mobile) to pan.
  - **Mouse Scroll** or **Pinch‐Zoom** (Mobile) to zoom in/out.
- **Context Menus:**
  - Right-click (PC) or two-finger tap (Mobile) on unit/tile to show contextual options (“Move Here”, “Attack”, “Use Ability”, “Inspect”).
- **Hotkeys (PC):**
  - `M` = Move, `A` = Attack, `Q/W/E/R` = Abilities 1–4, `Space` = End Turn, `I` = Inventory/Equipment, `Esc` = Pause/Options

---

## TECHNICAL CONSTRAINTS

### Performance Requirements (Existing + additional details)

- **Frame Rate:** Maintain ≥60 fps during combat animations on mid-range hardware.
- **Memory Usage:** <100 MB RAM for WebAssembly builds; <200 MB for native desktop.
- **Loading Times:** 
  - **Between Combat Encounters:** <2 s (stream tiles and unit data lazily).
  - **Initial Asset Load:** <3 s (optimize sprite atlas to single PNG).
- **Save File Size:** JSON format, <1 MB per save (compressible via gzip if desired).
- **Network (Web):** All assets hosted on CDN with proper caching headers.
- **Mobile (Native):** Implement texture atlas paging to keep VRAM <80 MB on low-end devices.

### Art Asset Specifications (Existing + additional details)

```
Sprite Dimensions:
- Unit sprites: 32×32 px (per frame)
- Tile textures: 64×64 px
- UI elements: Variable, power-of-2 dims (e.g., 128×128, 256×64)
- Animation frames: 4–8 frames per action (Idle, Move, Attack, Death, etc.)
- Particle sprites: 16×16 or 32×32 textures within separate atlas
```

- **Sprite Atlases:**
  - Each faction: separate atlas file (e.g., `imperial_spritesheet.png`, `ork_spritesheet.png`, `chaos_spritesheet.png`).
  - UI icons: `ui_icons.png` (normalized α-channel for crisp edges).
- **Color Palette:**
  - **Imperial:** Blue (#005f8b), Gold (#d4af37), Red accents (#8b0000)
  - **Orks:** Green (#4b8b3b), Brown (#8b4513), Metallic grays (#555555)
  - **Chaos:** Black (#0f0f0f), Red (#800000), Purple (#5d478b)
  - **UI:** High-contrast (white #ffffff text on dark backgrounds #202020); tooltips have semi-transparent dark (α = 0.8).
- **Animation Specs:**
  - Idle: 4 frames at 0.25 s per frame (loops).
  - Move: 6 frames at 0.15 s per frame (loops).
  - Attack: 8 frames at 0.1 s per frame (non-looping; return to Idle after).
  - Death: 8 frames at 0.2 s per frame (after final frame, remove sprite).

### Audio Implementation (Existing + additional details)

- **File Formats:**
  - **Sound Effects:** WAV (uncompressed) or OGG (variable compression, low latency).
  - **Music:** OGG or MP3 (loopable tracks, ~128 kbps to minimize download size).
- **Sound Channels:**
  - SFX channels: up to 16 simultaneous SFX playback (fade out old ones if maxed).
  - Music channel: 1 channel for BGM; allow crossfades.
  - Voice channel: reserved for voiceover (not heavily used).
- **Audio Priority:**
  1. UI SFX (button clicks, confirmations)
  2. Combat SFX (shots, explosions)
  3. Ability SFX
  4. Music
  - If too many sounds play concurrently, low‐priority ones are culled first (e.g., footsteps, ambient).

---

## BALANCING PARAMETERS

### Combat Balance (Existing + additional details)

- **Average Combat Duration:** 8–12 turns  
- **Player Advantage:** 10–15% edge (weapon accuracy + cover/tile bonuses vs AI).
- **Difficulty Scaling:**
  - `enemy_level = f(player_avg_level, chapter_index) = player_avg_level + (chapter_index - 1)*2`.
  - For each chapter, enemy stats are boosted by +10% for HP and +5% for damage.
- **Critical Hit Rate:** 10% base, modified by weapon tier (Basic +0, Advanced +5%, MasterCrafted +10%).
- **Cover Bonuses:**
  - Half Cover (low crates, fences): +10 to defender’s dodge.
  - Full Cover (walls, trenches): +20 to defender’s dodge.
- **Health and AP Pools:**
  - Tanks (Space Marines) have HP: 50–150; DPS (Guardsman, Cultist) have lower HP but higher mobility.
  - AP per turn: floor(Agility / 2). Rounding down; minimum 1 AP.
- **Enemy AI Tactics (Added Details):**
  - Use a simple behavior tree:
    1. **Check Priority 1:** If another ally is within 3 tiles and HP < 30%, use heal/support ability.
    2. **Check Priority 2:** If high‐value target (“squishy” low‐armor unit) in range, attempt to shoot.
    3. **Check Priority 3:** If no target in range, move toward nearest player unit.
    4. **Check Priority 4:** If blocked, use alternative path or use cover.
  - For bosses, add special scripted behaviors (e.g., trigger “enrage” at 50% HP).

### Recruitment Difficulty (Existing)

- **Questions per Challenge:** 5 questions
- **Success Threshold:** 4/5 correct answers required
- **Question Pool:** 50+ questions per faction, randomized
- **Retry Mechanism:** Unlimited attempts, but question pool rotates (no repeats in same attempt)

---

## TESTING REQUIREMENTS

### Functional Testing Checklist (Existing + expanded)

- [ ] **Grid Movement Validation**
  - Check movement cost for orthogonal vs diagonal vs difficult terrain.
  - Ensure pathfinding finds shortest valid path (A*).
  - Validate that obstacles block path and LOS correctly.
- [ ] **Combat Resolution Accuracy**
  - Edge Cases:
    - 0 AP → cannot perform action.
    - Attack from range > weapon.range → disallowed.
    - Critical hits apply exactly double damage.
    - Armor piercing correctly reduces effective toughness.
  - Test multi-target abilities (Cone, Circle, Line AoE).
- [ ] **Save/Load Functionality**
  - Serialize/Deserialize `GameState` fully (units, map state, turn queue, inventory).
  - Verify that after load, unit positions, HP, AP, cooldowns, and statuses persist.
- [ ] **Recruitment System**
  - Prompt displays correct question text and options.
  - Answer selection recognized; `player_score` increments accordingly.
  - Unit spawns if `player_score ≥ required_correct_answers`; else no spawn.
- [ ] **Equipment Effects**
  - Equipping a new weapon/armor updates unit’s `current_stats`.
  - Stats properly recalc when swapping equipment.
  - Inventory UI interactions (drag/drop or click‐to‐equip) function without disconnect.
- [ ] **Ability Effects**
  - Cooldowns applied correctly (decrement at end of turn).
  - AP deduction correct; cannot use ability if insufficient AP.
  - AoE shapes highlight correct tiles in UI before confirmation.
- [ ] **Unit AI Decision Making**
  - Enemy uses heal/support when appropriate.
  - Boss scripted phases trigger at designated HP thresholds.
- [ ] **UI/UX**
  - Turn order display updates accurately at start/end of each turn.
  - Selection cursor moves smoothly; confirmation ensures correct unit/tile.
  - Floating damage/heal text appears, floats, and fades as designed.
- [ ] **Cross-Platform Input Handling**
  - Mouse/Keyboard on desktop: movement, selection, UI navigation all bind correctly.
  - Touch controls on mobile: tap, long-press, pinch-zoom all responsive.
- [ ] **Graphics & Animation**
  - Sprite animations play at the right times (attack, move, idle, death).
  - Tile and sprite layering correct (no Z-fighting or incorrect overlap).
  - Particle effects spawn and despawn correctly.
- [ ] **Audio**
  - All SFX (shots, hits, UI clicks) play on corresponding events.
  - BGM crossfades smoothly.
  - Volume sliders adjust volumes in real-time.

### Performance Benchmarks (Existing + added details)

- **Combat Encounter Loading:** <1 s from “Start Encounter” to first frame.
- **Unit Animation Smoothness:** Consistent 60 fps during max‐sized encounters (20 units + 10 particle systems).
- **Memory Leak Detection:**
  - Play for 30 minutes without restarting; memory usage should plateau (no >5 MB increase over time).
- **Battery Usage (Mobile):**
  - Under standard conditions (mid‐range device), maintain ≥30 fps and discharge ≤10% battery/hour.

---

## SUCCESS METRICS FOR IMPLEMENTATION

### Code Quality Metrics

- **Unit Test Coverage:** ≥80% for core gameplay loops (combat, movement, leveling, equipment).
- **Linting/Formatting:** All code runs `cargo fmt` and `clippy` without warnings.
- **Error Handling:** None of the game-critical paths (`load`, `save`, `encounter initiation`) should panic. Use `Result<T,GameError>`.
- **Documentation:** All public structs/enums have Rustdoc comments.

### Gameplay Metrics

- **Average Player Session:** 45–60 minutes per play‐through (campaign mode).
- **Combat Encounter Completion Rate:** ≥95% (i.e., <5% of encounters bug‐out or crash).
- **Recruitment Challenge Success Rate:** 60–70% (tuning: if too low, adjust question difficulty).
- **Chapter Progression Retention:** 70% of players who finish Chapter 1 reach Chapter 3 (via telemetry/analytics).

---

## ADDITIONAL IMPLEMENTATION CONSIDERATIONS (Added Details)

1. **Localization / I18N:**
   - Store all displayed text (UI labels, ability names, lore questions, dialogues) in external JSON/YAML files keyed by language code (`en_US.json`, `zh_CN.json`, etc.).
   - Implement a `Localizer::get_string(key: &str) -> String` that looks up the current language table.
   - UI should handle dynamic text length changes (resizable panels).
2. **Accessibility:**
   - **Color Blind Mode:** Alternative palette for color‐coded elements (e.g., ability icons, team indicators).
   - **Text Size Adjustment:** In Options, allow UI font scaling (100%–200%).
   - **Audio Subtitles:** For each SFX, provide subtitle text (e.g., “[Bolter Fire]”) toggled on/off.
3. **Mod Support (Late Phase Consideration):**
   - Expose a scripting interface (e.g., hot‐reloaded Lua scripts) to define new units, abilities, or events without recompiling core engine.
   - Provide documentation and a template mod folder (units.toml, items.toml, abilities.toml).
4. **Networked Co-op / PvP (Future Work):**
   - Abstract `GameState` so that, in the future, turn data (`TurnQueue`, `unit_actions`) can be serialized and sent over WebSockets for peer-to-peer or client‐server play.
   - Ensure all randomness (dice rolls) use a shared RNG seed or a verified commit‐reveal to prevent cheating.
5. **Analytics / Telemetry (Web Only):**
   - Optionally integrate lightweight telemetry (e.g., capture JSON logs of “encounter_completed”, “chapter_failed”) to refine difficulty tuning.
   - Respect user privacy (opt‐in/out prompts) and send anonymized data.

---

## FINAL NOTES

- **Codex Feedback Addressed:**  
  - A full **Graphics System** has been outlined with sprite atlasing, animation loops, tile layering, particle effects, and camera controls.  
  - An expanded **Audio Implementation** shows event-driven triggers, volume controls, and crossfades.  
  - A **complete Equipment Data Model** (weapons, armor, accessories) plus **Inventory/Equipment UI** is now specified.  
  - A comprehensive **Ability Data Model** (effects, cooldowns, AOE, animations, sounds) is added.  
  - **UI mockups** (ASCII art) describe how combat, menus, and inventory screens should look.  
  - **Performance, testing, and balancing** sections now include deeper, concrete benchmarks and validation steps.  
  - Miscellaneous improvements (particle systems, localization, accessibility) clarify future‐proofing.  

All of the above fills in the missing “advanced features” that Codex noted were not yet implemented. Implementers can now use this as a detailed blueprint to build out graphics, audio, equipment, abilities, UI, and support systems, beyond the original basic framework.  

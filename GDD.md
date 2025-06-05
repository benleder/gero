# Warhammer 40K Grid RPG - Game Design Document

## TECHNICAL IMPLEMENTATION SPECIFICATIONS

### Core Architecture Requirements
```
Platform: Rust with wasm-pack for web deployment
Target: Web browser primary (WASM), native builds for mobile
Grid System: 2D array-based positioning (recommended 16x12 tiles)
Turn Management: Queue-based initiative system
State Management: Arc<RwLock<GameState>> for thread-safe state
Persistence: serde JSON serialization for save/load
Rendering: wgpu or canvas-based 2D rendering
```

### Data Models

#### Unit Struct Definition
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,
    pub toughness: i32,
    pub agility: i32,
    pub intellect: i32,
    pub willpower: i32,
    pub fellowship: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitType {
    SpaceMarine,
    Guardsman,
    Commissar,
    TechPriest, // Imperial
    OrkBoy,
    OrkNob,
    Weirdboy, // Ork
    Cultist,
    ChaosMarine,
    Daemon, // Chaos
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Faction {
    Imperial,
    Ork,
    Chaos,
}
```

#### Combat System Data
```rust
#[derive(Debug, Clone)]
pub struct CombatEncounter {
    pub player_units: Vec<Unit>,
    pub enemy_units: Vec<Unit>,
    pub battlefield: GridMap,
    pub turn_order: TurnQueue,
    pub current_phase: CombatPhase,
}

#[derive(Debug, Clone)]
pub struct TurnQueue {
    pub initiative: VecDeque<String>, // Unit IDs
    pub current_unit_id: Option<String>,
    pub round_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatPhase {
    Movement,
    Action,
    End,
}
```

## GAMEPLAY MECHANICS IMPLEMENTATION

### Grid Movement System
- **Movement Rules:** Units move up to Agility/2 tiles per turn
- **Diagonal Movement:** Allowed, costs 1.5 movement points
- **Obstacles:** Terrain tiles block movement and line of sight
- **Positioning Bonuses:** Adjacent allies provide +1 to hit rolls

### Combat Resolution Algorithm
```
Attack Resolution:
1. Calculate base hit chance: (Attacker.Agility + Weapon.Accuracy) vs (Defender.Agility + Cover.Bonus)
2. Roll d100, success if roll <= hit chance
3. If hit: Calculate damage = (Weapon.Damage + Attacker.Strength) - Defender.Toughness
4. Apply damage to Defender.healthPoints
5. Check for critical hits (roll <= 10): Double damage
```

### Recruitment Mechanic Implementation
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

## CONTENT SPECIFICATIONS

### Imperial Units (Playable)
**Tier 1 (Starting Units):**
- Guardsman: HP 25, Str 3, Tgh 3, Agi 3, Equipment: Lasgun
- Acolyte: HP 20, Int 4, Wil 4, Agi 3, Equipment: Stub gun

**Tier 2 (Mid-game Recruits):**
- Space Marine: HP 50, Str 5, Tgh 5, Agi 4, Equipment: Bolter
- Tech-Priest: HP 35, Int 5, Wil 4, Equipment: Mechadendrite

**Tier 3 (Late-game Recruits):**
- Space Marine Sergeant: HP 65, Str 6, Tgh 5, Agi 4, Equipment: Power sword
- Commissar: HP 45, Fel 5, Wil 6, Equipment: Bolt pistol, Chainsword

### Enemy Factions
**Ork Units:**
- Ork Boy: HP 30, Str 4, Tgh 4, Equipment: Choppa, Slugga
- Ork Nob: HP 45, Str 5, Tgh 5, Equipment: Power Klaw
- Weirdboy: HP 25, Wil 5, Abilities: Psychic powers

**Chaos Units:**
- Cultist: HP 20, Str 3, Tgh 3, Equipment: Autogun
- Chaos Space Marine: HP 50, Str 5, Tgh 5, Equipment: Bolter
- Lesser Daemon: HP 35, Str 4, Abilities: Daemonic resilience

### Chapter/Area Structure
**Chapter 1: Hive World Uprising**
- Area: Industrial sector (12x16 grid maps)
- Primary Enemies: Cultists, few Chaos Marines
- Recruitable Units: 2 Guardsmen, 1 Acolyte
- Boss: Chaos Cult Leader

**Chapter 2: Orbital Station Assault**
- Area: Space station corridors (10x14 grid maps)
- Primary Enemies: Mixed Cultists and Chaos Marines
- Recruitable Units: 1 Tech-Priest, 1 Space Marine
- Boss: Possessed Tech-Heretek

**Chapter 3: Jungle Death World**
- Area: Dense forest terrain (14x18 grid maps)
- Primary Enemies: Ork warband introduction
- Recruitable Units: 1 Space Marine, 1 Guardsman veteran
- Boss: Ork Nob with warband

**Chapter 4: Fortress Siege**
- Area: Fortified positions (16x20 grid maps)
- Primary Enemies: Ork army with vehicles
- Recruitable Units: 1 Commissar, 1 Space Marine Sergeant
- Boss: Ork Warboss

**Chapter 5: Chaos Stronghold**
- Area: Daemonic corruption zones (18x22 grid maps)
- Primary Enemies: High-tier Chaos forces and daemons
- Final Recruits: Elite Imperial units
- Boss: Chaos Lord with daemon allies

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
    }
    
    fn apply_level_bonuses(unit: &mut Unit) {
        // Implementation specific to unit type and level
        match unit.unit_type {
            UnitType::SpaceMarine => {
                unit.base_stats.strength += 1;
                unit.base_stats.toughness += 1;
            }
            UnitType::Guardsman => {
                unit.base_stats.agility += 1;
            }
            // ... other unit types
            _ => {}
        }
    }
}
```

### Equipment System
**Weapon Tiers:**
- Basic: Lasgun (+2 damage, 85% accuracy)
- Advanced: Bolter (+4 damage, 80% accuracy, armor piercing)
- Master-crafted: Power weapons (+6 damage, 75% accuracy, special abilities)

**Armor Progression:**
- Flak armor: +1 Toughness
- Carapace armor: +2 Toughness, -1 Agility
- Power armor: +3 Toughness, +1 Strength (Space Marines only)

## USER INTERFACE SPECIFICATIONS

### Screen Layout Requirements
```
Combat Screen Layout:
- Grid battlefield: 70% of screen (centered)
- Unit info panel: 15% (right side)
- Action buttons: 10% (bottom)
- Turn order display: 5% (top)

Menu Hierarchy:
- Main Menu → New Game / Load Game / Options
- Game Screen → Combat / Party Management / Equipment / Save
- Recruitment Screen → Lore Challenge Interface
```

### Input Controls
- **Grid Navigation:** Click/tap to select tiles
- **Unit Selection:** Click/tap unit sprites
- **Action Confirmation:** Double-click or confirm button
- **Menu Access:** Right-click or menu button
- **Camera Control:** WASD or swipe gestures (mobile)

## TECHNICAL CONSTRAINTS

### Performance Requirements
- **Frame Rate:** Maintain 60fps during combat animations
- **Memory Usage:** <100MB RAM for web deployment
- **Loading Times:** <2 seconds between combat encounters
- **Save File Size:** JSON format, <1MB per save

### Art Asset Specifications
```
Sprite Dimensions:
- Unit sprites: 32x32 pixels
- Tile textures: 64x64 pixels
- UI elements: Variable, power-of-2 dimensions
- Animation frames: 4-8 frames per action

Color Palette:
- Imperial: Blue, gold, red accents
- Orks: Green, brown, metallic
- Chaos: Black, red, purple corruption
- UI: High contrast for readability
```

### Audio Implementation
```rust
use std::collections::HashMap;

pub struct AudioManager {
    sound_effects: HashMap<&'static str, &'static str>,
    background_music: HashMap<&'static str, &'static str>,
}

impl AudioManager {
    pub fn new() -> Self {
        let mut sound_effects = HashMap::new();
        sound_effects.insert("laser_shot", "assets/audio/laser.wav");
        sound_effects.insert("bolter_fire", "assets/audio/bolter.wav");
        sound_effects.insert("melee_hit", "assets/audio/sword.wav");
        sound_effects.insert("unit_death", "assets/audio/death.wav");
        sound_effects.insert("level_up", "assets/audio/levelup.wav");
        
        let mut background_music = HashMap::new();
        background_music.insert("combat", "assets/audio/combat_theme.ogg");
        background_music.insert("exploration", "assets/audio/exploration_theme.ogg");
        background_music.insert("recruitment", "assets/audio/recruitment_theme.ogg");
        
        Self {
            sound_effects,
            background_music,
        }
    }
    
    pub fn play_sound_effect(&self, effect_name: &str) {
        // Web Audio API integration for WASM
        if let Some(path) = self.sound_effects.get(effect_name) {
            // Implementation depends on web audio bindings
        }
    }
    
    pub fn play_background_music(&self, track_name: &str) {
        if let Some(path) = self.background_music.get(track_name) {
            // Implementation for background music
        }
    }
}
```

## BALANCING PARAMETERS

### Combat Balance
- **Average Combat Duration:** 8-12 turns
- **Player Advantage:** 10-15% statistical edge to account for AI limitations
- **Difficulty Scaling:** Enemy level = Player average level + (Chapter - 1)
- **Critical Hit Rate:** 10% base, modified by weapon type

### Recruitment Difficulty
- **Questions per Challenge:** 5 questions
- **Success Threshold:** 4/5 correct answers required
- **Question Pool:** 50+ questions per faction, randomized selection
- **Retry Mechanism:** Unlimited attempts with different question sets

### Resource Economy
- **Equipment Drops:** 25% chance per enemy defeated
- **Experience Scaling:** Higher level enemies give exponentially more XP
- **Healing Items:** Limited availability, encourage tactical play

## TESTING REQUIREMENTS

### Functional Testing Checklist
- [ ] Grid movement validation for all unit types
- [ ] Combat resolution accuracy with edge cases
- [ ] Save/load functionality across all game states
- [ ] Recruitment system with all question types
- [ ] Equipment effects properly applied
- [ ] Level progression and stat increases
- [ ] Enemy AI decision making
- [ ] Cross-platform input handling

### Performance Benchmarks
- Combat encounter loading: <1 second
- Unit animation smoothness: 60fps maintained
- Memory leak detection during extended play
- Battery usage optimization for mobile platforms

## SUCCESS METRICS FOR IMPLEMENTATION

### Code Quality Metrics
- Unit test coverage: >80% for core systems
- No game-breaking bugs in critical path
- Consistent frame rate across target devices
- Successful cross-platform deployment

### Gameplay Metrics
- Average player session: 45-60 minutes
- Combat encounter completion rate: >95%
- Recruitment challenge success rate: 60-70%
- Chapter progression retention: 70% reach Chapter 3

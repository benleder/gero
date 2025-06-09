use crate::frontend::{Renderer, DrawCall};
use crate::input::GameAction;
use crate::localization::Localizer;

pub mod options;

#[derive(Debug, Clone)]
pub struct Panel {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct UiButton {
    pub id: String,
    pub bounds: Panel,
}

#[derive(Debug, Clone)]
pub struct FloatingText {
    pub value: i32,
    pub position: (u32, u32),
    pub is_heal: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiTab {
    Abilities,
    Inventory,
}

impl UiTab {
    pub fn label(&self, loc: &Localizer) -> String {
        match self {
            UiTab::Abilities => loc.get("ui.tab.abilities"),
            UiTab::Inventory => loc.get("ui.tab.inventory"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent {
    AbilityPressed(String),
    InventoryPressed(String),
}

#[derive(Debug)]
pub struct UiManager {
    pub top_bar: Panel,
    pub battlefield: Panel,
    pub info_panel: Panel,
    pub bottom_bar: Panel,
    pub ability_buttons: Vec<UiButton>,
    pub inventory_buttons: Vec<UiButton>,
    pub floating_texts: Vec<FloatingText>,
    pub current_tab: UiTab,
    pub selected_index: usize,
}

impl UiManager {
    pub fn new(screen_width: u32, screen_height: u32, abilities: Vec<String>, items: Vec<String>) -> Self {
        let top_h = (screen_height as f32 * 0.10) as u32;
        let bottom_h = top_h;
        let info_w = (screen_width as f32 * 0.15) as u32;
        let battlefield_w = (screen_width as f32 * 0.70) as u32;
        let battlefield_h = screen_height - top_h - bottom_h;
        let battlefield_x = 0;
        let info_x = battlefield_w;

        let ability_buttons = abilities
            .into_iter()
            .enumerate()
            .map(|(i, id)| UiButton {
                id,
                bounds: Panel {
                    x: info_x + 4,
                    y: top_h + 4 + (i as u32) * 36,
                    width: info_w - 8,
                    height: 32,
                },
            })
            .collect();

        let inventory_buttons = items
            .into_iter()
            .enumerate()
            .map(|(i, id)| UiButton {
                id,
                bounds: Panel {
                    x: 4 + (i as u32) * 36,
                    y: screen_height - bottom_h + 4,
                    width: 32,
                    height: bottom_h - 8,
                },
            })
            .collect();

        Self {
            top_bar: Panel { x: 0, y: 0, width: screen_width, height: top_h },
            battlefield: Panel { x: battlefield_x, y: top_h, width: battlefield_w, height: battlefield_h },
            info_panel: Panel { x: info_x, y: top_h, width: info_w, height: battlefield_h },
            bottom_bar: Panel { x: 0, y: screen_height - bottom_h, width: screen_width, height: bottom_h },
            ability_buttons,
            inventory_buttons,
            floating_texts: Vec::new(),
            current_tab: UiTab::Abilities,
            selected_index: 0,
        }
    }

    pub fn handle_input(&mut self, action: GameAction) -> Option<UiEvent> {
        match action {
            GameAction::SelectUp => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                None
            }
            GameAction::SelectDown => {
                let len = match self.current_tab {
                    UiTab::Abilities => self.ability_buttons.len(),
                    UiTab::Inventory => self.inventory_buttons.len(),
                };
                if self.selected_index + 1 < len {
                    self.selected_index += 1;
                }
                None
            }
            GameAction::Activate => match self.current_tab {
                UiTab::Abilities => self.ability_buttons.get(self.selected_index).map(|b| UiEvent::AbilityPressed(b.id.clone())),
                UiTab::Inventory => self.inventory_buttons.get(self.selected_index).map(|b| UiEvent::InventoryPressed(b.id.clone())),
            },
        }
    }

    pub fn spawn_floating_text(&mut self, value: i32, position: (u32, u32)) {
        self.floating_texts.push(FloatingText { value, position, is_heal: value > 0 });
    }

    pub fn render(&mut self, renderer: &mut Renderer, loc: &Localizer) {
        renderer.draw_log.push(DrawCall { sprite_id: loc.get("panel.top_bar"), position: (self.top_bar.x, self.top_bar.y), frame_index: 0 });
        renderer.draw_log.push(DrawCall { sprite_id: loc.get("panel.battlefield"), position: (self.battlefield.x, self.battlefield.y), frame_index: 0 });
        renderer.draw_log.push(DrawCall { sprite_id: loc.get("panel.info_panel"), position: (self.info_panel.x, self.info_panel.y), frame_index: 0 });
        renderer.draw_log.push(DrawCall { sprite_id: loc.get("panel.bottom_bar"), position: (self.bottom_bar.x, self.bottom_bar.y), frame_index: 0 });

        for btn in &self.ability_buttons {
            renderer.draw_log.push(DrawCall { sprite_id: format!("button:ability:{}", btn.id), position: (btn.bounds.x, btn.bounds.y), frame_index: 0 });
        }
        for btn in &self.inventory_buttons {
            renderer.draw_log.push(DrawCall { sprite_id: format!("button:inventory:{}", btn.id), position: (btn.bounds.x, btn.bounds.y), frame_index: 0 });
        }

        for ft in &self.floating_texts {
            let kind_key = if ft.is_heal { "float.heal" } else { "float.damage" };
            let prefix = loc.get(kind_key);
            renderer.draw_log.push(DrawCall { sprite_id: format!("{}:{}", prefix, ft.value.abs()), position: ft.position, frame_index: 0 });
        }
    }
}

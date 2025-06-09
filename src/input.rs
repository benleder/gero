use winit::event::{Event, WindowEvent, DeviceEvent, ElementState, MouseButton, TouchPhase};
use winit::keyboard::{KeyCode, PhysicalKey};

/// High level actions used by the game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameAction {
    SelectUp,
    SelectDown,
    Activate,
}

/// Maps winit events to high level [`GameAction`]s.
/// In tests the handler records all actions that were produced.
pub struct InputHandler {
    pub action_log: Vec<GameAction>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self { action_log: Vec::new() }
    }

    /// Process an event, returning an action if one was recognized.
    pub fn process_event<T>(&mut self, event: &Event<T>) -> Option<GameAction> {
        use GameAction::*;
        let action = match event {
            Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button, .. }, .. } => {
                if *button == MouseButton::Left { Some(Activate) } else { None }
            }
            Event::WindowEvent { event: WindowEvent::Touch(touch), .. } => {
                if touch.phase == TouchPhase::Started { Some(Activate) } else { None }
            }
            Event::DeviceEvent { event: DeviceEvent::Key(raw), .. } => {
                if raw.state == ElementState::Pressed {
                    match raw.physical_key {
                        PhysicalKey::Code(KeyCode::ArrowUp) => Some(SelectUp),
                        PhysicalKey::Code(KeyCode::ArrowDown) => Some(SelectDown),
                        PhysicalKey::Code(KeyCode::Enter) => Some(Activate),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(ref a) = action { self.action_log.push(a.clone()); }
        action
    }

    /// Process an event and play a menu navigation sound when an action occurs.
    pub fn process_event_with_audio<T>(&mut self, event: &Event<T>, mut audio: Option<&mut crate::audio::AudioSystem>) -> Option<GameAction> {
        let action = self.process_event(event);
        if action.is_some() {
            if let Some(sys) = audio.as_deref_mut() {
                sys.play("button_click");
            }
        }
        action
    }
}

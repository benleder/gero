use gero::input::{InputHandler, GameAction};
use winit::event::{Event, WindowEvent, DeviceEvent, ElementState, MouseButton, TouchPhase, Touch};
use winit::event::DeviceId;
use winit::keyboard::KeyCode;
use winit::window::WindowId;
use winit::event::RawKeyEvent;
use winit::keyboard::PhysicalKey;
use winit::dpi::PhysicalPosition;

#[test]
fn keyboard_event_triggers_action() {
    let mut handler = InputHandler::new();
    let event = Event::<()>::DeviceEvent {
        device_id: unsafe { DeviceId::dummy() },
        event: DeviceEvent::Key(RawKeyEvent {
            physical_key: PhysicalKey::Code(KeyCode::ArrowUp),
            state: ElementState::Pressed,
        }),
    };
    assert_eq!(handler.process_event(&event), Some(GameAction::SelectUp));
    assert_eq!(handler.action_log, vec![GameAction::SelectUp]);
}

#[test]
fn mouse_event_triggers_action() {
    let mut handler = InputHandler::new();
    let event = Event::<()>::WindowEvent {
        window_id: unsafe { WindowId::dummy() },
        event: WindowEvent::MouseInput {
            device_id: unsafe { DeviceId::dummy() },
            state: ElementState::Pressed,
            button: MouseButton::Left,
        },
    };
    assert_eq!(handler.process_event(&event), Some(GameAction::Activate));
}

#[test]
fn touch_event_triggers_action() {
    let mut handler = InputHandler::new();
    let touch = Touch {
        device_id: unsafe { DeviceId::dummy() },
        phase: TouchPhase::Started,
        location: PhysicalPosition { x: 0.0, y: 0.0 },
        force: None,
        id: 1,
    };
    let event = Event::<()>::WindowEvent {
        window_id: unsafe { WindowId::dummy() },
        event: WindowEvent::Touch(touch),
    };
    assert_eq!(handler.process_event(&event), Some(GameAction::Activate));
}

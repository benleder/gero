use gero::ui::{UiManager, UiTab, UiEvent};
use gero::localization::Localizer;
use gero::frontend::Renderer;
use gero::input::{InputHandler, GameAction};
use winit::event::{Event, DeviceEvent, WindowEvent, ElementState, MouseButton};
use winit::event::DeviceId;
use winit::window::WindowId;

#[test]
fn layout_panels_from_gdd() {
    let ui = UiManager::new(100, 100, vec![], vec![]);
    assert_eq!(ui.top_bar.height, 10);
    assert_eq!(ui.bottom_bar.height, 10);
    assert_eq!(ui.info_panel.width, 15);
    assert_eq!(ui.battlefield.width, 70);
}

#[test]
fn ability_button_activation_via_input_handler() {
    let mut ui = UiManager::new(80, 80, vec!["fire".into()], vec![]);
    let mut handler = InputHandler::new();
    let event = Event::<()>::WindowEvent {
        window_id: unsafe { WindowId::dummy() },
        event: WindowEvent::MouseInput {
            device_id: unsafe { DeviceId::dummy() },
            state: ElementState::Pressed,
            button: MouseButton::Left,
        },
    };
    let action = handler.process_event(&event).unwrap();
    ui.current_tab = UiTab::Abilities;
    let res = ui.handle_input(action);
    assert_eq!(res, Some(UiEvent::AbilityPressed("fire".into())));
}

#[test]
fn floating_text_draws_using_renderer() {
    let mut ui = UiManager::new(50, 50, vec![], vec![]);
    ui.spawn_floating_text(-5, (10, 10));
    let mut renderer = Renderer::new_headless(50, 50);
    let loc = Localizer::new("en").unwrap();
    ui.render(&mut renderer, &loc);
    assert!(renderer
        .draw_log
        .iter()
        .any(|c| c.sprite_id == "float:damage:5" && c.position == (10, 10)));
}

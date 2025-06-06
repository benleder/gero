# Gero

This crate contains the core logic for a small tactical RPG. It exposes a 
minimal renderer, combat rules and data models. Examples below show how to set 
up an event loop with `winit` and enable sound playback using `rodio`.

```rust
use gero::{frontend::Renderer, input::{InputHandler, GameAction}, audio::AudioSystem};
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

fn main() {
    // Create window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    // Renderer and systems
    let mut renderer = pollster::block_on(Renderer::new(&window));
    let mut input = InputHandler::new();
    let mut audio = AudioSystem::new();

    event_loop.run(move |event, _, control| {
        if let Some(action) = input.process_event(&event) {
            match action {
                GameAction::Activate => println!("activate"),
                GameAction::SelectUp => println!("up"),
                GameAction::SelectDown => println!("down"),
            }
        }
        match event {
            Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => *control = winit::event_loop::ControlFlow::Exit,
            Event::MainEventsCleared => {
                // update and render here
                renderer.draw_log.clear();
            }
            _ => {}
        }
    });
}
```
```

The audio system loads raw bytes with `load_sound_from_bytes` and plays them via
`play`. During tests a headless variant is used which simply records played
sound keys.
```

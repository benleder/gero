use std::collections::HashMap;

use wgpu::SurfaceConfiguration;

use crate::state::GameState;
use crate::models::Position;

/// A very small renderer skeleton following the GDD specifications.
/// In a real implementation this would handle sprite atlases and draw calls
/// using wgpu. Here we only set up the device and basic state so that
/// integration with the backend can be tested.
pub struct Renderer<'a> {
    pub width: u32,
    pub height: u32,
    surface: Option<wgpu::Surface<'a>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    config: Option<SurfaceConfiguration>,
    /// mapping from sprite_id -> atlas rectangle
    pub sprites: HashMap<String, (u32, u32, u32, u32)>,
}

impl<'a> Renderer<'a> {
    /// Create a new renderer tied to a window. This is async because wgpu device
    /// creation is async.
    #[cfg(not(test))]
    pub async fn new(window: &'a winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window) }.expect("create surface");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("request adapter");
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("request device");
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        Self {
            width: size.width,
            height: size.height,
            surface: Some(surface),
            device: Some(device),
            queue: Some(queue),
            config: Some(config),
            sprites: HashMap::new(),
        }
    }

    /// Headless constructor used in tests or non-graphical environments.
    pub fn new_headless(width: u32, height: u32) -> Self {
        Self { width, height, surface: None, device: None, queue: None, config: None, sprites: HashMap::new() }
    }

    /// Render the game state. In this skeleton this only iterates over the units
    /// to demonstrate integration with the backend data structures.
    pub fn render_state(&mut self, state: &GameState) {
        for unit in &state.units {
            let Position { x, y } = unit.grid_position;
            // In a full implementation we would issue draw calls based on
            // unit.sprite_id and position here. For now we simply store the
            // last seen position in the sprite map for verification.
            self.sprites.insert(unit.id.clone(), (x as u32, y as u32, 0, 0));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Unit, UnitType, Faction};

    #[test]
    fn dummy_renderer_creates() {
        let renderer = Renderer::new_headless(800, 600);
        assert_eq!(renderer.width, 800);
        assert_eq!(renderer.height, 600);
    }

    #[test]
    fn render_updates_sprite_positions() {
        let mut renderer = Renderer::new_headless(800, 600);
        let mut unit = Unit::new("u1", "Test", UnitType::Guardsman, Faction::Imperial);
        unit.grid_position = Position { x: 2, y: 3 };
        let state = GameState::new(vec![unit]);
        renderer.render_state(&state);
        assert_eq!(renderer.sprites.get("u1"), Some(&(2, 3, 0, 0)));
    }
}

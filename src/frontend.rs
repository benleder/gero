use std::collections::HashMap;

use wgpu::SurfaceConfiguration;

use crate::state::GameState;
use crate::models::{Position, AnimationState, AnimationType};

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
    /// loaded sprite textures (each sprite may have multiple frames)
    pub sprite_textures: HashMap<String, Vec<Vec<u8>>>,
    /// record of draw calls issued during the last render
    pub draw_log: Vec<DrawCall>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawCall {
    pub sprite_id: String,
    pub position: (u32, u32),
    pub frame_index: usize,
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
            sprite_textures: HashMap::new(),
            draw_log: Vec::new(),
        }
    }

    /// Headless constructor used in tests or non-graphical environments.
    pub fn new_headless(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            surface: None,
            device: None,
            queue: None,
            config: None,
            sprites: HashMap::new(),
            sprite_textures: HashMap::new(),
            draw_log: Vec::new(),
        }
    }

    /// Load a sprite with one or more animation frames from raw byte data.
    /// The renderer stores the bytes so tests can verify loading without a GPU.
    pub fn load_sprite_from_bytes(&mut self, id: &str, frames: Vec<Vec<u8>>) {
        self.sprite_textures.insert(id.to_string(), frames);
    }

    /// Render the game state. In this skeleton this only iterates over the units
    /// to demonstrate integration with the backend data structures.
    pub fn render_state(&mut self, state: &GameState) {
        self.draw_log.clear();
        for unit in &state.units {
            let Position { x, y } = unit.grid_position;
            if let Some(frames) = self.sprite_textures.get(&unit.sprite_id) {
                let frame = if !frames.is_empty() {
                    unit.animation_state.frame_index % frames.len()
                } else {
                    0
                } as usize;
                self.draw_log.push(DrawCall {
                    sprite_id: unit.sprite_id.clone(),
                    position: (x as u32, y as u32),
                    frame_index: frame,
                });
                self.sprites
                    .insert(unit.id.clone(), (x as u32, y as u32, frame as u32, frames.len() as u32));
            } else {
                // no sprite loaded; record position only
                self.sprites.insert(unit.id.clone(), (x as u32, y as u32, 0, 0));
            }
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

    #[test]
    fn render_records_draw_calls() {
        let mut renderer = Renderer::new_headless(100, 100);
        renderer.load_sprite_from_bytes("s", vec![vec![1, 2, 3]]);
        let mut unit = Unit::new("u", "U", UnitType::Guardsman, Faction::Imperial);
        unit.sprite_id = "s".into();
        unit.grid_position = Position { x: 1, y: 1 };
        renderer.render_state(&GameState::new(vec![unit]));
        assert_eq!(renderer.draw_log.len(), 1);
        assert_eq!(renderer.draw_log[0].sprite_id, "s");
        assert_eq!(renderer.draw_log[0].position, (1, 1));
        assert_eq!(renderer.draw_log[0].frame_index, 0);
    }
}

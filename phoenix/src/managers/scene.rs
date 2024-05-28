use super::entity::{Entity, EntityManager};
use crate::components::color::RGBA;
use crate::components::Component;
use crate::renderer::Render;
use crate::window::{WinError, Window};

pub type Result<T> = std::result::Result<T, SceneError>;

#[derive(thiserror::Error, Debug)]
pub enum SceneError {
    #[error("Window error: {0}")]
    WinError(#[from] WinError),
}

pub struct Scene {
    entity_manager: EntityManager,
    window: Window,
    renderer: Box<dyn Render>,
    background_color: RGBA,
}

impl Scene {
    pub fn new(window: Window, renderer: Box<dyn Render>) -> Self {
        Scene {
            entity_manager: EntityManager::new(),
            window,
            renderer,
            background_color: RGBA::default(),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if !self.window.is_current() {
            self.window.set_current()?;
        }

        while self.window.is_running() {
            self.renderer.set_background_color(&self.background_color);

            let keys = self.entity_manager.get_keys();
            for key in keys {
                if let Ok(id) = self
                    .renderer
                    .init_entity(self.entity_manager.as_ref_entity(key))
                {
                    self.renderer.draw_entity(id);
                }
            }

            self.window.swap_buffers();
            Window::poll_events();
        }

        Ok(())
    }

    pub fn set_background_color(&mut self, color: RGBA) {
        self.background_color = color
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entity_manager.add_entity(entity);
    }
}
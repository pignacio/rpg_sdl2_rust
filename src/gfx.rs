use std::path::Path;

use sdl2::{render::TextureCreator, surface::Surface, image::LoadSurface};



pub struct Texture<'r> {
    texture: sdl2::render::Texture<'r>,
    height: u32,
    width: u32,
}

impl<'r> Texture<'r> {
    pub fn texture(&self) -> &sdl2::render::Texture {
        &self.texture
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

pub struct TextureLoader<T> {
    texture_creator: TextureCreator<T>
}

impl<T> TextureLoader<T> {
    pub fn new(texture_creator: TextureCreator<T>) -> TextureLoader<T>{
        TextureLoader{ texture_creator }
    }

    pub fn load< P: AsRef<Path>>(&self, path: P) -> Result<Texture, String> {

        let surface = Surface::from_file(path)?;
        let height = surface.height();
        let width = surface.width();
        let texture = self.texture_creator.create_texture_from_surface(surface).map_err(|e| e.to_string())?;

        Ok(Texture{
            texture,
            height,
            width,
        })
    }
}
use std::path::Path;

use sdl2::{image::LoadSurface, render::TextureCreator, surface::Surface};

use crate::Error;

pub struct Texture<'r> {
    texture: sdl2::render::Texture<'r>,
    height: u32,
    width: u32,
}

impl<'r> Texture<'r> {
    pub fn texture(&self) -> &sdl2::render::Texture {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &'r mut sdl2::render::Texture {
        &mut self.texture
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

pub struct TextureLoader<'ttf, T> {
    texture_creator: &'ttf TextureCreator<T>,
}

impl<'tx, T> TextureLoader<'tx, T> {
    pub fn new(texture_creator: &'tx TextureCreator<T>) -> TextureLoader<'tx, T> {
        TextureLoader { texture_creator }
    }

    pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<Texture<'tx>, Error> {
        println!("Loading Texture @{} ", path.as_ref().to_str().unwrap_or("<<invalid file>>"));
        self.texture_from_surface(Surface::from_file(path)?)
    }

    pub fn texture_from_surface(&self, surface: Surface) -> Result<Texture<'tx>, Error> {
        let height = surface.height();
        let width = surface.width();
        let texture = self.texture_creator.create_texture_from_surface(surface)?;

        Ok(Texture {
            texture,
            height,
            width,
        })
    }

    pub fn create_target_texture(&self, width: u32, height: u32) -> Result<Texture<'tx>, Error> {
        let texture = self.texture_creator.create_texture_target(None, width, height)?;

        Ok(Texture{
            texture,
            width,
            height,
        })

    }
}
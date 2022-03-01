use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use sdl2::surface::Surface;

use crate::{Error, TextureLoader};
use crate::gfx::texture::Texture;
use crate::gfx::tileset::Tileset;

pub trait Resources<'ttf> {
    fn load_texture(&mut self, path: &Path) -> Result<Rc<Texture<'ttf>>, Error>;
    fn get_texture(&mut self, id: &str) -> Result<Rc<Texture<'ttf>>, Error>;

    fn get_tileset(&mut self, id: &str) -> Result<Rc<Tileset<'ttf>>, Error>;

    fn texture_from_surface(&self, surface: Surface) -> Result<Texture<'ttf>, Error>;
}

pub struct CachedResources<'ttf, T> {
    texture_loader: TextureLoader<'ttf, T>,
    textures: HashMap<String, Rc<Texture<'ttf>>>,
}

impl<'ttf, T> CachedResources<'ttf, T> {
    pub fn new(texture_loader: TextureLoader<'ttf, T>) -> Self {
        CachedResources { texture_loader, textures: HashMap::new() }
    }
}

impl<'ttf, T> Resources<'ttf> for CachedResources<'ttf, T> {
    fn load_texture(&mut self, path: &Path) -> Result<Rc<Texture<'ttf>>, Error> {
        let key = path.to_str().ok_or_else(|| Error::simple(format!("Path was not valid unicode: {:?}", path)))?.to_owned();
        if !self.textures.contains_key(&key) {
            self.textures.insert(key.clone(), Rc::new(self.texture_loader.load(path)?));
        }

        self.textures.get(&key).map(|rc| rc.clone()).ok_or_else(|| Error::simple("Could not insert texture into cache"))
    }

    fn get_texture(&mut self, id: &str) -> Result<Rc<Texture<'ttf>>, Error> {
        Err(Error::simple("Resource ds are not supported"))
    }

    fn get_tileset(&mut self, id: &str) -> Result<Rc<Tileset<'ttf>>, Error> {
        Err(Error::simple("Resource ds are not supported"))
    }

    fn texture_from_surface(&self, surface: Surface) -> Result<Texture<'ttf>, Error> {
        self.texture_loader.texture_from_surface(surface)
    }
}
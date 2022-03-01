use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::{Error, TextureLoader};
use crate::gfx::texture::Texture;
use crate::gfx::tileset::Tileset;

pub type LoadResult<T> = Result<Rc<T>, Error>;

pub trait Resources<'ttf> {
    fn load_texture(&mut self, path: &Path) -> LoadResult<Texture<'ttf>>;
    fn get_texture(&mut self, id: &str) -> LoadResult<Texture<'ttf>>;

    fn get_tileset(&mut self, id: &str) -> LoadResult<Tileset<'ttf>>;

    fn load_font(&mut self, path: &Path, size: u16) -> LoadResult<Font<'ttf, 'static>>;
    fn get_font(&mut self, id: &str) -> LoadResult<Font<'ttf, 'static>>;

    fn texture_from_surface(&self, surface: Surface) -> Result<Texture<'ttf>, Error>;
}

pub struct CachedResources<'ttf, T> {
    texture_loader: TextureLoader<'ttf, T>,
    ttf: &'ttf Sdl2TtfContext,
    textures: HashMap<String, Rc<Texture<'ttf>>>,
    fonts: HashMap<(String, u16), Rc<Font<'ttf, 'static>>>,
}

impl<'ttf, T> CachedResources<'ttf, T> {
    pub fn new(texture_loader: TextureLoader<'ttf, T>, ttf: &'ttf Sdl2TtfContext) -> Self {
        CachedResources { texture_loader, ttf, textures: HashMap::new(), fonts: HashMap::new() }
    }

    fn path_to_string(&self, path: &Path) -> Result<String, Error> {
        Ok(path.to_str().ok_or_else(|| Error::simple(format!("Path was not valid unicode: {:?}", path)))?.to_owned())
    }
}

impl<'ttf, T> Resources<'ttf> for CachedResources<'ttf, T> {
    fn load_texture(&mut self, path: &Path) -> LoadResult<Texture<'ttf>> {
        let key = self.path_to_string(path)?;

        Ok(self.textures.entry(key)
            .or_insert_with(|| Rc::new(self.texture_loader.load(path).unwrap()))
            .clone())
    }

    fn get_texture(&mut self, _id: &str) -> LoadResult<Texture<'ttf>> {
        Err(Error::simple("Resource ids are not supported"))
    }

    fn get_tileset(&mut self, _id: &str) -> LoadResult<Tileset<'ttf>> {
        Err(Error::simple("Resource ids are not supported"))
    }

    fn load_font(&mut self, path: &Path, size: u16) -> LoadResult<Font<'ttf, 'static>> {
        Ok(self.fonts.entry((self.path_to_string(path)?, size))
            .or_insert_with(|| {
                println!("Loading Font @{} with size {}", path.to_str().unwrap_or("<<invalid path>>"), size);
                Rc::new(self.ttf.load_font(path, size).unwrap())
            })
            .clone())
    }

    fn get_font(&mut self, _id: &str) -> LoadResult<Font<'ttf, 'static>> {
        Err(Error::simple("Resource ids are not supported"))
    }


    fn texture_from_surface(&self, surface: Surface) -> Result<Texture<'ttf>, Error> {
        self.texture_loader.texture_from_surface(surface)
    }
}
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::error::Error;
use crate::gfx::texture::{Texture};
use crate::point::IntPoint;

pub struct Renderer<'canvas, T: RenderTarget> {
    canvas: &'canvas mut Canvas<T>,
    offset: IntPoint,
}

impl<'canvas, T: RenderTarget> Renderer<'canvas, T> {
    pub fn new(canvas: &'canvas mut Canvas<T>) -> Self {
        Renderer { canvas, offset: IntPoint::new(0, 0) }
    }

    pub fn set_draw_color<C: Into<Color>>(&mut self, color: C) {
        self.canvas.set_draw_color(color)
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn copy<R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), Error>
        where
            R1: Into<Option<Rect>>,
            R2: Into<Option<Rect>>,
    {
        let new_dst = dst.into().map(|mut rect| {
            rect.set_x(rect.x() + self.offset.x);
            rect.set_y(rect.y() + self.offset.y);
            rect
        });

        Ok(self.canvas.copy(texture.texture(), src, new_dst)?)
    }


    pub fn with_offset<F>(&mut self, offset: IntPoint, func: F) -> Result<(), Error>
        where F: FnOnce(&mut Renderer<T>) -> Result<(), Error>,
    {
        self.offset += offset;
        let result = func(self);
        self.offset -= offset;
        result
    }

    pub fn with_target_texture<'r, F>(&mut self, texture: &mut sdl2::render::Texture<'r>, render_function: F) -> Result<(), Error>
        where F: FnOnce(&mut Renderer<T>) -> Result<(), Error> {
        self.canvas.with_texture_canvas(texture, |canvas| {
            let mut renderer = Renderer::new(canvas);
            render_function(&mut renderer).unwrap();
        })?;
        Ok(())
    }
}

pub struct BackBuffer<'sdl> {
    canvas: Canvas<Window>,
    back_buffer: sdl2::render::Texture<'sdl>,
}

impl<'sdl> BackBuffer<'sdl> {
    pub fn new(canvas: Canvas<Window>, creator: &'sdl TextureCreator<WindowContext>) -> Result<Self, Error> {
        println!("logical size: {:?}, output size: {:?}", canvas.logical_size(), canvas.output_size());
        let size = canvas.output_size().unwrap();

        Ok(BackBuffer {
            canvas,
            back_buffer: creator.create_texture_target(None, size.0, size.1)?,
        })
    }

    pub fn render_and_flip<F>(&mut self, render_function: F) -> Result<(), Error>
    where F: FnOnce(&mut Renderer<Window>) -> Result<(), Error>
    {
        let mut renderer = Renderer::new(&mut self.canvas);

        renderer.with_target_texture(&mut self.back_buffer, render_function)?;

        // self.canvas.with_texture_canvas(&mut self.back_buffer_2, |canvas| {
        //    let mut renderer = Renderer::new(canvas);
        //     render_function(&mut renderer).unwrap();
        // });

        self.canvas.copy(&self.back_buffer, None, None)?;
        self.canvas.present();
        Ok(())
    }
}
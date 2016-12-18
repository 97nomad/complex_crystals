use phi::data::Rectangle;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use ::sdl2::render::{Renderer, Texture};
use ::sdl2_image::LoadTexture;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle);
}

#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

impl Sprite {
    pub fn new(texture: Texture) -> Self {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: tex_query.width as f64,
                h: tex_query.height as f64,
                x: 0.0,
                y: 0.0,
            },
        }
    }

    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }

    pub fn width(&self) -> f64 {
        self.src.w
    }

    pub fn height(&self) -> f64 {
        self.src.h
    }
}

impl Renderable for Sprite {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl())
    }
}

pub trait CopySprite {
    fn copy_sprite(&mut self, renderable: &Renderable, dest: Rectangle);
}

impl<'window> CopySprite for Renderer<'window> {
    fn copy_sprite(&mut self, renderable: &Renderable, dest: Rectangle) {
        renderable.render(self, dest);
    }
}
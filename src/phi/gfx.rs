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

#[derive(Clone)]
pub struct AnimatedSprite {
    sprites: Rc<Vec<Sprite>>,
    frame_delay: f64,
    current_time: f64,
}

impl AnimatedSprite {
    pub fn new(sprites: Vec<Sprite>, frame_delay: f64) -> Self {
        AnimatedSprite {
            sprites: Rc::new(sprites),
            frame_delay: frame_delay,
            current_time: 0.0,
        }
    }

    pub fn with_fps(sprites: Vec<Sprite>, fps: f64) -> Self {
        AnimatedSprite::new(sprites, 1.0 / fps)
    }

    pub fn frames(&self) -> usize {
        self.sprites.len()
    }

    pub fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }

    pub fn set_fps(&mut self, fps: f64) {
        self.set_frame_delay(1.0 / fps);
    }

    pub fn add_time(&mut self, dt: f64) {
        self.current_time += dt;

        if self.current_time < 0.0 {
            self.current_time = (self.frames() - 1) as f64 * self.frame_delay;
        }
    }
}

impl Renderable for AnimatedSprite {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        let current_frame = (self.current_time / self.frame_delay) as usize % self.frames();

        let sprite = &self.sprites[current_frame];
        sprite.render(renderer, dest);
    }
}
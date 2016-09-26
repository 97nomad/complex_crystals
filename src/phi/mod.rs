#[macro_use]
mod events;
pub mod data;
pub mod gfx;

use ::sdl2::render::Renderer;
use ::sdl2::pixels::Color;
use phi::gfx::Sprite;
use std::path::Path;
use std::collections::HashMap;
use phi::events::Events;

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
    pub fps: u16,

    pub ttf_context: ::sdl2_ttf::Sdl2TtfContext,
    cached_fonts: HashMap<(&'static str, u16), ::sdl2_ttf::Font>,
}

impl<'window> Phi<'window> {
    pub fn new(events: Events,
               renderer: Renderer<'window>,
               ttf_context: ::sdl2_ttf::Sdl2TtfContext)
               -> Self {
        Phi {
            events: events,
            renderer: renderer,
            fps: 0,
            ttf_context: ttf_context,
            cached_fonts: HashMap::new(),
        }
    }
    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

    pub fn ttf_str_sprite(&mut self,
                          text: &str,
                          font_path: &'static str,
                          size: u16,
                          color: Color)
                          -> Option<Sprite> {

        if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
            return font.render(text)
                .blended(color)
                .ok()
                .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
                .map(Sprite::new);
        }
        self.ttf_context
            .load_font(Path::new(font_path), size)
            .ok()
            .and_then(|font| {
                font.render(text)
                    .blended(color)
                    .ok()
                    .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
                    .map(Sprite::new)
            })
    }
}

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View>
{
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _image_context = ::sdl2_image::init(::sdl2_image::INIT_PNG).unwrap();
    let ttf_context = ::sdl2_ttf::init().unwrap();

    let window = video.window(title, 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut context = Phi::new(Events::new(sdl_context.event_pump().unwrap()),
                               window.renderer().accelerated().build().unwrap(),
                               ttf_context);

    let mut current_view = init(&mut context);

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    'running: loop {
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        if dt < interval {
            timer.delay(interval - dt);
            continue 'running;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            context.fps = fps;
            last_second = now;
            fps = 0;
        }

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break 'running,
            ViewAction::ChangeView(new_view) => current_view = new_view,
        }
    }
}
use ::piston::input::*;
use ::piston_window::{G2dTexture, PistonWindow};
use ::piston_window::{Window, Size};
use ::conrod;
use ::conrod::Ui;
use ::engine::{Scene, SceneAction};
use ::ui as UI;

widget_ids! {
    pub struct Ids {
        canvas,
        background_image,
        title_text, 
        addr_text_box,
    }
}

pub struct MainMenuScene {
    ui: Ui,
    ids: Ids,
    image_map: conrod::image::Map<G2dTexture>,
    glyph_cache: conrod::text::GlyphCache,
    text_texture_cache: G2dTexture,
    size: Size,

    addr_tb: String,
}

impl MainMenuScene {
    pub fn new(window: &mut PistonWindow) -> Self {
        let size = window.draw_size();

        let mut ui = UI::build_ui(size.width as f64, size.height as f64);
        let (glyph_cache, text_texture_cache) =
            UI::get_glyph_and_texture_cache(size.width, size.height, window);
        let ids = Ids::new(ui.widget_id_generator());
        let image_map = conrod::image::Map::new();

        MainMenuScene {
            ui: ui,
            ids: ids,
            image_map: image_map,
            glyph_cache: glyph_cache,
            text_texture_cache: text_texture_cache,
            size: size,

            addr_tb: String::new(),
        }
    }
}

impl Scene for MainMenuScene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs) {
        window.draw_2d(&Input::Render(args), |context, graphics| {
            if let Some(primitives) = self.ui.draw_if_changed() {
                conrod::backend::piston::draw::primitives(primitives,
                                                          context,
                                                          graphics,
                                                          &mut self.text_texture_cache,
                                                          &mut self.glyph_cache,
                                                          &self.image_map,
                                                          UI::cache_queued_glyphs,
                                                          UI::texture_from_image);
            }

        });
    }
    fn update(&mut self, _: &UpdateArgs) -> SceneAction {
        use conrod::{widget, Widget, Positionable};
        const MARGIN: conrod::Scalar = 30.0;

        let mut ui = self.ui.set_widgets();

        widget::Canvas::new().pad(MARGIN).set(self.ids.canvas, &mut ui);

        widget::Text::new("Complex Crystals")
            .mid_top_of(self.ids.canvas)
            .set(self.ids.title_text, &mut ui);

        for ev in widget::TextBox::new(&self.addr_tb)
            .center_justify()
            .mid_bottom_with_margin_on(self.ids.title_text, 20.0)
            .set(self.ids.addr_text_box, &mut ui) {
            use conrod::widget::text_box::Event;
            match ev {
                Event::Update(s) => self.addr_tb = s,
                Event::Enter => return SceneAction::ToGameScene(self.addr_tb.clone()),
            }
        }

        SceneAction::None
    }

    fn event(&mut self, event: Input) {
        UI::handle_event(&mut self.ui,
                         event,
                         self.size.width as f64,
                         self.size.height as f64);
    }
}

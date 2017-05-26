extern crate find_folder;

use ::piston::input::*;
use ::piston_window::{G2d, G2dTexture, TextureSettings, PistonWindow};
use ::piston_window::Window;
use ::piston_window::texture::UpdateTexture;
use ::conrod;
use ::conrod::Ui;
use ::engine::{Scene, SceneAction};
use std::time::Duration;

widget_ids! {
    pub struct Ids {
        canvas,
        background_image,
        title_text, 
    }
}

pub struct MainMenuScene {
    ui: Ui,
    ids: Ids,
    image_map: conrod::image::Map<G2dTexture>,
    glyph_cache: conrod::text::GlyphCache,
    text_texture_cache: G2dTexture,
}

impl MainMenuScene {
    pub fn new(window: &mut PistonWindow) -> Self {
        use conrod::position::{Align, Direction, Padding, Position, Relative};
        let size = window.draw_size();

        let mut ui = conrod::UiBuilder::new([size.width as f64, size.height as f64])
            .theme(conrod::Theme {
                name: "Demo Theme".to_string(),
                padding: Padding::none(),
                x_position: Position::Relative(Relative::Align(Align::Start), None),
                y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0),
                                               None),
                background_color: conrod::color::DARK_CHARCOAL,
                shape_color: conrod::color::LIGHT_CHARCOAL,
                border_color: conrod::color::BLACK,
                border_width: 0.0,
                label_color: conrod::color::WHITE,
                font_id: None,
                font_size_large: 26,
                font_size_medium: 18,
                font_size_small: 12,
                widget_styling: conrod::theme::StyleMap::default(),
                mouse_drag_threshold: 0.0,
                double_click_threshold: Duration::from_millis(500),
            })
            .build();

        // load fonts
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/slkscr.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // text caching
        let (glyph_cache, text_texture_cache) = {
            const SCALE_TOLERANCE: f32 = 0.1;
            const POSITION_TOLERANCE: f32 = 0.1;
            let cache = conrod::text::GlyphCache::new(size.width,
                                                      size.height,
                                                      SCALE_TOLERANCE,
                                                      POSITION_TOLERANCE);
            let buffer_len = size.width * size.height;
            let init = vec![128; buffer_len as usize];
            let settings = TextureSettings::new();
            let factory = &mut window.factory;
            let texture =
                G2dTexture::from_memory_alpha(factory, &init, size.width, size.height, &settings)
                    .unwrap();
            (cache, texture)
        };

        // list of widget identifiers
        let ids = Ids::new(ui.widget_id_generator());

        // widget->image mapping
        let image_map = conrod::image::Map::new();

        MainMenuScene {
            ui: ui,
            ids: ids,
            image_map: image_map,
            glyph_cache: glyph_cache,
            text_texture_cache: text_texture_cache,
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
                                                          cache_queued_glyphs,
                                                          texture_from_image);
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

        SceneAction::None
    }
}

fn cache_queued_glyphs(graphics: &mut G2d,
                       cache: &mut G2dTexture,
                       rect: ::conrod::text::rt::Rect<u32>,
                       data: &[u8]) {

    let offset = [rect.min.x, rect.min.y];
    let size = [rect.width(), rect.height()];
    let format = ::piston_window::texture::Format::Rgba8;
    let encoder = &mut graphics.encoder;
    let mut text_vertex_data = Vec::new();
    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
    UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
        .expect("failed to update texture")
}
fn texture_from_image<T>(img: &T) -> &T {
    img
}

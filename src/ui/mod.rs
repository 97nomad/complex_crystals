use ::conrod;
use ::conrod::{Theme, Ui};
use ::conrod::text::GlyphCache;
use ::find_folder;
use ::piston_window::{G2d, G2dTexture, TextureSettings, PistonWindow, Input};
use ::piston_window::texture::UpdateTexture;

pub fn build_ui(width: f64, height: f64) -> conrod::Ui {
    let mut ui = conrod::UiBuilder::new([width, height])
        .theme(get_theme())
        .build();

    // load fonts
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/slkscr.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    ui
}

pub fn get_theme() -> Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    use std::time::Duration;
    conrod::Theme {
        name: "Main Menu Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
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
    }
}

pub fn handle_event(ui: &mut Ui, event: Input, width: f64, height: f64) {
    if let Some(ev) = conrod::backend::piston::event::convert(event, width, height) {
        ui.handle_event(ev);
    }
}

pub fn get_glyph_and_texture_cache(width: u32,
                                   height: u32,
                                   window: &mut PistonWindow)
                                   -> (GlyphCache, G2dTexture) {
    const SCALE_TOLERANCE: f32 = 0.1;
    const POSITION_TOLERANCE: f32 = 0.1;
    let cache = conrod::text::GlyphCache::new(width, height, SCALE_TOLERANCE, POSITION_TOLERANCE);
    let buffer_len = width * height;
    let init = vec![128; buffer_len as usize];
    let settings = TextureSettings::new();
    let factory = &mut window.factory;
    let texture = G2dTexture::from_memory_alpha(factory, &init, width, height, &settings).unwrap();
    (cache, texture)
}

pub fn cache_queued_glyphs(graphics: &mut G2d,
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
pub fn texture_from_image<T>(img: &T) -> &T {
    img
}

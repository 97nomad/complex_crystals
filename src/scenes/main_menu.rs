use piston::input::*;
use piston_window::{G2dTexture, PistonWindow};
use piston_window::{Window, Size};
use conrod;
use conrod::Ui;
use engine::{Scene, SceneAction};
use ui as UI;
use std::str::FromStr;

widget_ids! {
    pub struct Ids {
        canvas,
        background_image,
        title_text, 
        addr_text_box,
        connect_button,
        start_server_text,
        server_width,
        server_height,
        server_players,
        create_button,
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
    server_width: String,
    server_height: String,
    players_count: String,
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
            server_width: String::new(),
            server_height: String::new(),
            players_count: String::new(),
        }
    }
}

impl Scene for MainMenuScene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs) {
        window.draw_2d(&Input::Render(args),
                       |context, graphics| if let Some(primitives) = self.ui.draw_if_changed() {
                           conrod::backend::piston::draw::primitives(primitives,
                                                                     context,
                                                                     graphics,
                                                                     &mut self.text_texture_cache,
                                                                     &mut self.glyph_cache,
                                                                     &self.image_map,
                                                                     UI::cache_queued_glyphs,
                                                                     UI::texture_from_image);
                       });
    }
    fn update(&mut self, _: &UpdateArgs) -> SceneAction {
        use conrod::{widget, Widget, Positionable};
        const MARGIN: conrod::Scalar = 30.0;

        let mut ui = self.ui.set_widgets();

        widget::Canvas::new()
            .pad(MARGIN)
            .set(self.ids.canvas, &mut ui);

        widget::Text::new("Complex Crystals")
            .mid_top_of(self.ids.canvas)
            .set(self.ids.title_text, &mut ui);

        for ev in widget::TextBox::new(&self.addr_tb)
                .center_justify()
                .mid_top_with_margin_on(self.ids.title_text, 20.0)
                .set(self.ids.addr_text_box, &mut ui) {
            use conrod::widget::text_box::Event;
            match ev {
                Event::Update(s) => self.addr_tb = s,
                Event::Enter => return SceneAction::ConnectToServer(self.addr_tb.clone()),
            }
        }

        widget::Text::new("Start Server")
            .mid_top_with_margin_on(self.ids.addr_text_box, 50.0)
            .set(self.ids.start_server_text, &mut ui);


        for ev in widget::TextBox::new(&self.server_width)
                .center_justify()
                .mid_top_with_margin_on(self.ids.start_server_text, 20.0)
                .set(self.ids.server_width, &mut ui) {
            use conrod::widget::text_box::Event;
            match ev {
                Event::Update(s) => self.server_width = s,
                _ => {}
            }
        }

        for ev in widget::TextBox::new(&self.server_height)
                .center_justify()
                .mid_top_with_margin_on(self.ids.server_width, 20.0)
                .set(self.ids.server_height, &mut ui) {
            use conrod::widget::text_box::Event;
            match ev {
                Event::Update(s) => self.server_height = s,
                _ => {}
            }
        }

        for ev in widget::TextBox::new(&self.players_count)
                .center_justify()
                .mid_top_with_margin_on(self.ids.server_height, 20.0)
                .set(self.ids.server_players, &mut ui) {
            use conrod::widget::text_box::Event;
            match ev {
                Event::Update(s) => self.players_count = s,                
                Event::Enter => {
                    return SceneAction::StartServer((isize::from_str(&self.server_width).unwrap(),
                                                     isize::from_str(&self.server_height).unwrap(),
                                                     isize::from_str(&self.players_count).unwrap()))
                }
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

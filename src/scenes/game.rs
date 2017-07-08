use piston::input::*;
use piston_window::{PistonWindow, Window, G2dTexture, Size};
use engine::{Scene, SceneAction};
use piston_window::Rectangle;
use piston_window::Button::{Keyboard, Mouse};
use conrod;
use conrod::{Ui, Colorable};
use ui as UI;

use network::ServerConnection;
use data_types::{ObjectResponse, ObjectType};
use utils::camera::{Camera, Direction};

const NETWORK_UPDATE_TIMER: f64 = 1.0;
const SPRITE_SIZE: f64 = 10.0;

widget_ids! {
    pub struct Ids {
        canvas,
        text,
        selected_object_text,
    }
}
pub struct GameScene {
    server: Box<ServerConnection>,

    camera: Camera,
    cursor_pos: [f64; 2],
    selected_object: Option<ObjectResponse>,

    ui: Ui,
    ids: Ids,
    image_map: conrod::image::Map<G2dTexture>,
    glyph_cache: conrod::text::GlyphCache,
    text_texture_cache: G2dTexture,
    draw_size: Size,
}

impl GameScene {
    pub fn new(window: &mut PistonWindow, connection: Box<ServerConnection>) -> Option<Self> {
        let draw_size = window.draw_size();

        let mut ui = UI::build_ui(draw_size.width as f64, draw_size.height as f64);
        let (glyph_cache, text_texture_cache) =
            UI::get_glyph_and_texture_cache(draw_size.width, draw_size.height, window);
        let ids = Ids::new(ui.widget_id_generator());
        let image_map = conrod::image::Map::new();

        if let Some(info) = connection.check_connection() {
            println!("Server name: {}\nServer status: {}\nTPS: {}",
                     info.name,
                     info.status,
                     info.tps);
            Some(GameScene {
                     server: connection,
                     camera: Camera::new((draw_size.width as f64, draw_size.height as f64),
                                         (1000.0, 1000.0)),
                     cursor_pos: [0.0, 0.0],
                     selected_object: None,
                     ui,
                     ids,
                     image_map,
                     glyph_cache,
                     text_texture_cache,
                     draw_size,
                 })
        } else {
            println!("Can't connect to server");
            None
        }
    }
}

impl Scene for GameScene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs) {
        window.draw_2d(&Input::Render(args), |context, graphics| {
            ::piston_window::clear([0.8, 1.0, 0.8, 1.0], graphics);

            // draw objects
            for (_, obj) in self.server.get_objects().iter() {
                let rectangle = match obj.otype {
                    ObjectType::Asteroid => Rectangle::new([0.2, 0.2, 0.2, 1.0]),
                    ObjectType::Builder => Rectangle::new([0.2, 0.8, 0.2, 1.0]),
                    ObjectType::Harvester => Rectangle::new([0.2, 0.2, 0.8, 1.0]),
                    ObjectType::Battlecruiser => Rectangle::new([0.8, 0.2, 0.2, 1.0]),
                };
                rectangle.draw(self.camera
                                   .transform([obj.x - (SPRITE_SIZE / 2.0),
                                               obj.y - (SPRITE_SIZE / 2.0),
                                               SPRITE_SIZE,
                                               SPRITE_SIZE]),
                               &context.draw_state,
                               context.transform,
                               graphics);
            }

            // draw UI
            let primitives = self.ui.draw();
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
    fn update(&mut self, args: &UpdateArgs) -> SceneAction {
        use conrod::{widget, Widget, Positionable};

        // server update
        self.server.update(args.dt);

        // UI update
        let mut ui = self.ui.set_widgets();
        widget::Canvas::new()
            .align_top()
            .rgba(0.0, 0.0, 0.0, 0.0)
            .set(self.ids.canvas, &mut ui);

        widget::Text::new("Complex Crystals")
            .align_top_of(self.ids.canvas)
            .align_middle_x_of(self.ids.canvas)
            .set(self.ids.text, &mut ui);

        if let Some(ref object) = self.selected_object {
            widget::Text::new(&format!("name: {}\nowner: {}\ntype: {:?}\npos_x: {}\npos_y: {}",
                                      object.name,
                                      object.owner,
                                      object.otype,
                                      object.x,
                                      object.y))
                    .rgba(0.0, 0.0, 0.0, 1.0)
                    .align_bottom_of(self.ids.canvas)
                    .align_left_of(self.ids.canvas)
                    .set(self.ids.selected_object_text, &mut ui);
        }

        SceneAction::None
    }
    fn event(&mut self, event: Input) {
        // game event
        match event {
            Input::Press(Keyboard(Key::Up)) => self.camera.shift(Direction::Up, 1.0),
            Input::Press(Keyboard(Key::Down)) => self.camera.shift(Direction::Down, 1.0),
            Input::Press(Keyboard(Key::Left)) => self.camera.shift(Direction::Left, 1.0),
            Input::Press(Keyboard(Key::Right)) => self.camera.shift(Direction::Right, 1.0),
            Input::Press(Mouse(MouseButton::Left)) => {
                let objects = self.server.get_objects();
                if let Some((_, object)) =
                    objects
                        .iter()
                        .find(|&(_, obj)| {
                            intersect(self.camera
                                          .transform_reverse([self.cursor_pos[0] -
                                                              SPRITE_SIZE / 2.0,
                                                              self.cursor_pos[1] -
                                                              SPRITE_SIZE / 2.0,
                                                              SPRITE_SIZE,
                                                              SPRITE_SIZE]),
                                      [obj.x, obj.y])
                        }) {
                    self.selected_object = Some(object.clone());
                }
            }
            Input::Move(Motion::MouseCursor(x, y)) => self.cursor_pos = [x, y],
            _ => {
                // ui event
                if let Some(ev) = conrod::backend::piston::event::convert(event.clone(),
                                                                          self.draw_size.width as
                                                                          f64,
                                                                          self.draw_size.height as
                                                                          f64) {
                    self.ui.handle_event(ev);
                }
            }
        }
    }
}

fn intersect(rect: [f64; 4], point: [f64; 2]) -> bool {
    point[0] >= rect[0] && point[0] <= rect[0] + rect[2] && point[1] >= rect[1] &&
    point[1] <= rect[1] + rect[3]
}

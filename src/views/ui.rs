use ::sdl2::pixels::Color;
use ::phi::gfx::{Sprite, CopySprite};
use ::phi::data::Rectangle;
use ::phi::Phi;
use ::phi::WIDTH;
use ::network::sampleobject::ServerInfo;

const DOWNUI_PATH: &'static str = "assets/downui.png";
// const DOWNUI_MINIMAP_PATH: &'static str = "assets/downuiminimap.png";
// const LEFT_UPUI_PATH: &'static str = "assets/leftupui.png";
// const RIGHT_UPUI_PATH: &'static str = "assets/rightupui.png";
const CENTER_UPUI_PATH: &'static str = "assets/centerupui.png";
const FONT_PATH: &'static str = "assets/slkscr.ttf";

#[derive(Clone)]
pub struct UpUI {
    background_center: Sprite,
    data: Vec<Sprite>,
}

impl UpUI {
    pub fn new(phi: &mut Phi) -> Self {
        let data = vec![
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap(),
            phi.ttf_str_sprite("Player", FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap(),
            phi.ttf_str_sprite("ServerName", FONT_PATH, 16, Color::RGB(255, 255, 255))
                .unwrap(),
            phi.ttf_str_sprite(" ", FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap(),
            phi.ttf_str_sprite("0", FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap(),
            phi.ttf_str_sprite("0", FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap(),
            phi.ttf_str_sprite("0", FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap(),
            phi.ttf_str_sprite("TPS", FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap(),
        ];
        UpUI {
            background_center: Sprite::load(&phi.renderer, CENTER_UPUI_PATH).unwrap(),
            data: data,
        }
    }

    pub fn set_data(&mut self, phi: &mut Phi, data: ServerInfo) {
        self.data[2] = phi.ttf_str_sprite(&data.name.to_string(),
                            FONT_PATH,
                            24,
                            Color::RGB(255, 255, 255))
            .unwrap();
        self.data[3] = phi.ttf_str_sprite(&data.status.to_string(),
                            FONT_PATH,
                            24,
                            Color::RGB(255, 255, 255))
            .unwrap();
        self.data[7] = phi.ttf_str_sprite(&data.tps.to_string(),
                            FONT_PATH,
                            24,
                            Color::RGB(255, 255, 255))
            .unwrap();
    }

    pub fn set_fps(&mut self, phi: &mut Phi, fps: u16) {
        self.data[0] =
            phi.ttf_str_sprite(&fps.to_string(), FONT_PATH, 24, Color::RGB(255, 255, 255))
                .unwrap();
    }

    pub fn render(&mut self, phi: &mut Phi) {
        let (center_w, center_h) = self.background_center.size();
        let start_pos = (WIDTH - (self.data.len() as f64 * self.background_center.width())) / 2.0 *
                        phi.width_coeff;
        for (i, sprite) in self.data.iter().enumerate() {
            let rect = Rectangle {
                x: start_pos + (i as f64 * center_w * phi.width_coeff),
                y: 0.0,
                w: center_w * phi.width_coeff,
                h: center_h * phi.height_coeff,
            };
            phi.renderer.copy_sprite(&self.background_center, rect);
            let (sprite_w, sprite_h) = sprite.size();
            phi.renderer.copy_sprite(sprite,
                                     rect.in_center(sprite_w * phi.width_coeff,
                                                    sprite_h * phi.height_coeff));
        }
    }
}

#[derive(Clone)]
pub struct DownUI {
    background: Sprite,
    data: Vec<Sprite>,
}

impl DownUI {
    pub fn new(phi: &mut Phi) -> Self {
        DownUI {
            background: Sprite::load(&phi.renderer, DOWNUI_PATH).unwrap(),
            data: vec![],
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn add_data(&mut self, phi: &mut Phi, data: String) {
        self.data
            .push(phi.ttf_str_sprite(&data, FONT_PATH, 18, Color::RGB(255, 255, 255)).unwrap());
    }

    pub fn render(&mut self, phi: &mut Phi, rect: Rectangle) {
        phi.renderer.copy_sprite(&self.background,
                                 Rectangle {
                                     x: rect.x,
                                     y: rect.y,
                                     w: rect.w,
                                     h: rect.h,
                                 });

        self.draw_unit_info(phi,
                            Rectangle {
                                x: rect.w * 0.25,
                                y: rect.y + rect.h * 0.08,
                                w: rect.w - rect.w * 0.3,
                                h: rect.h - (rect.y + rect.h * 0.2),
                            });
    }

    fn draw_unit_info(&mut self, phi: &mut Phi, dest: Rectangle) {
        const MAX_ELEMENTS_IN_COLUMN: u16 = 7;
        const COLUMN_WIDTH: f64 = 300.0;
        const BORDER: f64 = 10.0;
        let mut element = 0;
        let mut rect = Rectangle {
            x: dest.x,
            y: dest.y,
            w: 0.0,
            h: 0.0,
        };
        for sprite in &self.data {
            let mut dest_rect = Rectangle {
                x: rect.x,
                y: rect.y + rect.h + (BORDER * phi.height_coeff),
                w: sprite.width() * phi.width_coeff,
                h: sprite.height() * phi.height_coeff,
            };

            phi.renderer.copy_sprite(sprite, dest_rect.clone());
            element += 1;
            if element == MAX_ELEMENTS_IN_COLUMN {
                dest_rect = Rectangle {
                    x: dest_rect.x + (COLUMN_WIDTH * phi.width_coeff),
                    y: dest.y,
                    w: 0.0,
                    h: 0.0,
                };
                element = 0;
            }
            rect = dest_rect;
        }
    }
}
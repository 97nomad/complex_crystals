use ::sdl2::render::Renderer;
use ::sdl2::pixels::Color;
use ::phi::gfx::{Sprite, CopySprite};
use ::phi::data::Rectangle;
use ::phi::Phi;

const DOWNUI_PATH: &'static str = "assets/downui.png";
const DOWNUI_MINIMAP_PATH: &'static str = "assets/downuiminimap.png";
const LEFT_UPUI_PATH: &'static str = "assets/leftupui.png";
const RIGHT_UPUI_PATH: &'static str = "assets/rightupui.png";
const CENTER_UPUI_PATH: &'static str = "assets/centerupui.png";
const FONT_PATH: &'static str = "assets/belligerent.ttf";

#[derive(Clone)]
pub struct UpUI {
    background_left: Sprite,
    background_right: Sprite,
    background_center: Sprite,
    data: Vec<Sprite>,
}

impl UpUI {
    pub fn new(phi: &mut Phi) -> Self {
        let data = vec![
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
            phi.ttf_str_sprite("FPS", FONT_PATH, 24, Color::RGB(0, 0, 255))
                .unwrap(),
        ];
        UpUI {
            background_left: Sprite::load(&phi.renderer, LEFT_UPUI_PATH).unwrap(),
            background_right: Sprite::load(&phi.renderer, RIGHT_UPUI_PATH).unwrap(),
            background_center: Sprite::load(&phi.renderer, CENTER_UPUI_PATH).unwrap(),
            data: data,
        }
    }

    pub fn set_fps(&mut self, phi: &mut Phi, fps: u16) {
        self.data[0] = phi.ttf_str_sprite(&fps.to_string(), FONT_PATH, 24, Color::RGB(0, 0, 255))
            .unwrap();
    }

    pub fn render(&mut self, renderer: &mut Renderer, dest: Rectangle) {
        let (left_w, left_h) = self.background_left.size();
        let (center_w, center_h) = self.background_center.size();
        let (right_w, right_h) = self.background_right.size();
        let window_w = renderer.viewport().width() as f64;
        let window_h = renderer.viewport().height() as f64;
        // let start_x = window_w + (left_w + (center_w * self.data.len() as f64) - 2.0) + right_w;
        for (i, sprite) in self.data.iter().enumerate() {
            let rect = Rectangle {
                x: (i as f64 * center_w),
                y: 0.0,
                w: center_w,
                h: center_h,
            };
            renderer.copy_sprite(&self.background_center, rect);
            let (sprite_w, sprite_h) = sprite.size();
            renderer.copy_sprite(sprite, rect.in_center(sprite_w, sprite_h));
        }
    }
}

#[derive(Clone)]
pub struct DownUI {
    background: Sprite,
    minimap_background: Sprite,
    data: Vec<Sprite>,
}

impl DownUI {
    pub fn new(phi: &mut Phi) -> Self {
        DownUI {
            background: Sprite::load(&phi.renderer, DOWNUI_PATH).unwrap(),
            minimap_background: Sprite::load(&phi.renderer, DOWNUI_MINIMAP_PATH).unwrap(),
            data: vec![],
        }
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn add_data(&mut self, phi: &mut Phi, data: String) {
        self.data.push(phi.ttf_str_sprite(&data, FONT_PATH, 18, Color::RGB(255, 0, 0)).unwrap());
    }

    pub fn render(&mut self, phi: &mut Phi) {
        let (w, h) = phi.output_size();
        let width_coeff = phi.width_coeff;
        let height_coeff = phi.height_coeff;

        let minimap_w = self.minimap_background.width() * width_coeff;
        let minimap_h = self.minimap_background.height() * height_coeff;
        phi.renderer.copy_sprite(&self.minimap_background,
                                 Rectangle {
                                     x: 0.0,
                                     y: h - minimap_h,
                                     w: minimap_w,
                                     h: minimap_h,
                                 });
        let bg_w = self.background.width() * width_coeff;
        let bg_h = self.background.height() * height_coeff;
        phi.renderer.copy_sprite(&self.background,
                                 Rectangle {
                                     x: minimap_w,
                                     y: h - bg_h,
                                     w: bg_w,
                                     h: bg_h,
                                 });

        self.draw_unit_info(phi,
                            Rectangle {
                                x: minimap_w + (275.0 * width_coeff),
                                y: h - bg_h + (125.0 * height_coeff),
                                w: 100.0,
                                h: 100.0,
                            });
    }

    fn draw_unit_info(&mut self, phi: &mut Phi, dest: Rectangle) {
        const MAX_ELEMENTS_IN_COLUMN: u16 = 5;
        const COLUMN_WIDTH: f64 = 200.0;
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
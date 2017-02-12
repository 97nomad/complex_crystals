use ::phi::{Phi, View, ViewAction};
use ::phi::gfx::{Sprite, CopySprite};
use ::sdl2::pixels::Color;
use ::sdl2::rect::Point;
use ::phi::data::Rectangle;
use ::network::sampleobject::ObjectType;
use ::network::Network;
use ::views::ui::{DownUI, UpUI};
use ::views::camera::Camera;

const OBJECT_SIZE: f64 = 16.0;

const FONTNAME: &'static str = "slkscr";

pub struct GameView {
    network: Network,
    camera: Camera,

    one_second_timer: f64,
    one_minute_timer: f64,

    up_ui: UpUI,
    down_ui: DownUI,

    asteroid_sprite: Sprite,
    builder_sprite: Sprite,
    harvester_sprite: Sprite,
    battlecruiser_sprite: Sprite,
}

impl GameView {
    pub fn new(phi: &mut Phi) -> Self {
        let (width, height) = phi.output_size();
        GameView {
            network: Network::new(),
            one_minute_timer: 60.0,
            one_second_timer: 1.0,
            camera: Camera::new(0.0, 0.0, 0.0, 0.0, width, height),
            up_ui: UpUI::new(phi),
            down_ui: DownUI::new(phi),
            asteroid_sprite: phi.ttf_str_sprite("A", FONTNAME, 12, Color::RGB(128, 128, 128))
                .unwrap(),
            builder_sprite: phi.ttf_str_sprite("B", FONTNAME, 12, Color::RGB(0, 0, 200)).unwrap(),
            harvester_sprite: phi.ttf_str_sprite("H", FONTNAME, 12, Color::RGB(0, 200, 0))
                .unwrap(),
            battlecruiser_sprite: phi.ttf_str_sprite("C", FONTNAME, 12, Color::RGB(200, 0, 0))
                .unwrap(),
        }
    }
}

impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape {
            return ViewAction::Quit;
        }

        // Передвижение камеры и зум
        self.camera.handle_input(phi.events.now.key_up,
                                 phi.events.now.key_down,
                                 phi.events.now.key_left,
                                 phi.events.now.key_right,
                                 phi.events.now.mouse_wheel,
                                 elapsed);

        // Работа с сетью тут
        self.one_second_timer += elapsed;
        self.one_minute_timer += elapsed;
        if self.one_second_timer >= 1.0 {
            self.network.update_objects();
            self.one_second_timer = 0.0;
        }
        if self.one_minute_timer >= 60.0 {
            self.network.update_info();
            self.network.update_world_size();
            self.one_minute_timer = 0.0;
        }

        let fps = phi.fps;
        self.up_ui.set_fps(phi, fps); // Обновление FPS

        // Обновление информации о сервере
        let info;
        {
            info = self.network.server_info.lock().unwrap().clone();
        }
        self.up_ui.set_data(phi, info);

        // Чистим экран
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Рисуем объекты
        for (_, obj) in self.network.objects.lock().unwrap().iter() {
            let sprite = match obj.otype {
                ObjectType::Asteroid => &self.asteroid_sprite,
                ObjectType::Builder => &self.builder_sprite,
                ObjectType::Harvester => &self.harvester_sprite,
                ObjectType::Battlecruiser => &self.battlecruiser_sprite,
            };

            phi.renderer.copy_sprite(sprite,
                                     self.camera
                                         .translate_rect(Rectangle {
                                             x: obj.x - OBJECT_SIZE / 2.0,
                                             y: obj.y - OBJECT_SIZE / 2.0,
                                             w: OBJECT_SIZE,
                                             h: OBJECT_SIZE,
                                         }))
        }

        // Обработка тычка мышкой
        match phi.events.now.left_mouse_click {
            None => {}
            Some((x, y)) => {
                let cursor_rect = Rectangle {
                    x: x as f64 - OBJECT_SIZE / 2.0 * self.camera.zoom,
                    y: y as f64 - OBJECT_SIZE / 2.0 * self.camera.zoom,
                    w: OBJECT_SIZE * self.camera.zoom,
                    h: OBJECT_SIZE * self.camera.zoom,
                };

                self.down_ui.clear_data();
            }
        }

        // Отрисовка выбранного объекта в случае изменения грязного флага
        macro_rules! print_to_downui {  // Макрос для нижнего UI
                ($str1: expr, $str2: expr) => {
                        self.down_ui.add_data(phi, $str1.to_string() + &$str2.to_string());
                    }
                }

        // Рисуем рамку вокруг мира
        phi.renderer.set_draw_color(Color::RGB(255, 255, 255));
        let world_size = self.network.world_size.lock().unwrap();
        phi.renderer
            .draw_rect(self.camera
                .translate_rect(Rectangle {
                    x: 0.0,
                    y: 0.0,
                    w: world_size.width,
                    h: world_size.height,
                })
                .to_sdl()
                .unwrap())
            .unwrap();
        self.camera.resize(world_size.width, world_size.height);

        // Рисуем UI
        self.up_ui.render(phi);
        let (width, height) = phi.output_size();
        let down_ui_height = height * 0.3;
        self.down_ui.render(phi,
                            Rectangle {
                                x: 0.0,
                                y: height - down_ui_height,
                                w: width,
                                h: down_ui_height,
                            });

        ViewAction::None
    }
}
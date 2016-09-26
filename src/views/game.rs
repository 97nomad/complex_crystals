use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;
use ::phi::data::Rectangle;
use ::phi::gfx::{Sprite, CopySprite};
use ::network::sampleobject::ObjectType;
use ::network::Network;
use ::views::ui::{DownUI, UpUI};
use ::views::camera::Camera;

const CAMERA_SENSITIVITY: f64 = 1000.0;
const ZOOM_SENSITIVITY: f64 = 10.0;

pub struct GameView {
    network: Network,
    network_timer: f64,
    camera: Camera,

    up_ui: UpUI,
    down_ui: DownUI,
}

impl GameView {
    pub fn new(phi: &mut Phi) -> Self {
        GameView {
            network: Network::new(),
            network_timer: 0.0,
            camera: Camera::new(),
            up_ui: UpUI::new(phi),
            down_ui: DownUI::new(phi),
        }
    }
}

impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape {
            return ViewAction::Quit;
        }

        // Передвижение камеры и зум
        if phi.events.now.key_up {
            self.camera.move_up(CAMERA_SENSITIVITY * elapsed);
        }
        if phi.events.now.key_down {
            self.camera.move_down(CAMERA_SENSITIVITY * elapsed);
        }
        if phi.events.now.key_left {
            self.camera.move_left(CAMERA_SENSITIVITY * elapsed);
        }
        if phi.events.now.key_right {
            self.camera.move_right(CAMERA_SENSITIVITY * elapsed);
        }
        if phi.events.now.key_a {
            self.camera.zoom_in(ZOOM_SENSITIVITY * elapsed);
        }
        if phi.events.now.key_z {
            self.camera.zoom_out(ZOOM_SENSITIVITY * elapsed);
        }

        // Работа с сетью тут
        self.network_timer += elapsed;
        if self.network_timer >= 1.0 {
            self.network.update("http://localhost:3000/objects");
            self.network_timer = 0.0;
        }

        let fps = phi.fps;
        self.up_ui.set_fps(phi, fps); // Обновление FPS

        // Чистим экран
        phi.renderer
            .set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Рисуем объекты
        phi.renderer.set_draw_color(Color::RGB(0, 0, 255));
        for obj in self.network.objects.lock().unwrap().iter() {
            match obj.otype {
                ObjectType::Harvester => phi.renderer.set_draw_color(Color::RGB(0, 255, 0)),
                ObjectType::Battlecruiser => phi.renderer.set_draw_color(Color::RGB(255, 0, 0)),
            }

            let name = ObjectName::new(phi, &obj.name);
            let (w, h) = name.sprite.size();
            phi.renderer.copy_sprite(&name.sprite,
                                     self.camera.translate_rect(Rectangle {
                                         w: w,
                                         h: h,
                                         x: obj.x - (h * 1.5),
                                         y: obj.y + (w / 2.0),
                                     }));

            draw_object(phi, &self.camera, obj.x, obj.y);
        }

        self.down_ui.clear_data();
        self.down_ui.add_data(phi, "Object1".to_owned());
        self.down_ui.add_data(phi, "Object1".to_owned());
        self.down_ui.add_data(phi, "Object1".to_owned());
        self.down_ui.add_data(phi, "Object1".to_owned());
        self.down_ui.add_data(phi, "Object1".to_owned());
        self.down_ui.add_data(phi, "Object1".to_owned());

        // Рисуем UI
        let (w, h) = phi.output_size();
        self.up_ui.render(&mut phi.renderer,
                          Rectangle {
                              x: 0.0,
                              y: 0.0,
                              w: w,
                              h: h,
                          });
        self.down_ui.render(&mut phi.renderer,
                            Rectangle {
                                x: 0.0,
                                y: 0.0,
                                w: w,
                                h: h,
                            });

        ViewAction::None
    }
}

struct ObjectName {
    sprite: Sprite,
}

impl ObjectName {
    pub fn new(phi: &mut Phi, label: &String) -> Self {
        ObjectName {
            sprite: phi.ttf_str_sprite(&label, "assets/belligerent.ttf", 16, Color::RGB(0, 0, 255))
                .unwrap(),
        }
    }
}

fn draw_object(phi: &mut Phi, camera: &Camera, x: f64, y: f64) {
    phi.renderer
        .fill_rect(camera.translate_rect(Rectangle {
                x: x,
                y: y,
                w: 16.0,
                h: 16.0,
            })
            .to_sdl()
            .unwrap())
        .unwrap();
}
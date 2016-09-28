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
const OBJECT_SIZE: f64 = 16.0;

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
        if phi.events.now.mouse_wheel != 0 {
            self.camera.zoom(ZOOM_SENSITIVITY * elapsed * phi.events.now.mouse_wheel as f64);
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

                macro_rules! print_to_downui {  // Макрос для нижнего UI
                ($str1: expr, $str2: expr) => {
                        self.down_ui.add_data(phi, $str1.to_string() + &$str2.to_string());
                    }
                }
                for obj in self.network.objects.lock().unwrap().iter() {
                    if cursor_rect.contains_point((obj.x - self.camera.pos_x) * self.camera.zoom,
                                                  (obj.y - self.camera.pos_y) * self.camera.zoom) {
                        print_to_downui!("owner: ", obj.owner);
                        print_to_downui!("name: ", obj.name);
                        print_to_downui!("otype: ", obj.otype);
                        print_to_downui!("x:", obj.x);
                        print_to_downui!("y: ", obj.y);

                        print_to_downui!("drive_speed: ", obj.drive_speed);
                        print_to_downui!("drive_dest_x: ", obj.drive_dest_x);
                        print_to_downui!("drive_dest_y: ", obj.drive_dest_y);
                        print_to_downui!("radar_radius: ", obj.radar_radius);
                        print_to_downui!("radar_type: ", obj.radar_type);

                        print_to_downui!("weapon_active: ", obj.weapon_active);
                        print_to_downui!("weapon_type: ", obj.weapon_type);
                        print_to_downui!("weapon_radius: ", obj.weapon_radius);
                        print_to_downui!("weapon_target_x: ", obj.weapon_target_x);
                        print_to_downui!("weapon_target_y: ", obj.weapon_target_y);

                        print_to_downui!("cargo_type: ", obj.cargo_type);
                        print_to_downui!("cargo_max: ", obj.cargo_max);
                        print_to_downui!("cargo_current: ", obj.cargo_current);
                        print_to_downui!("shell_health: ", obj.shell_health);
                        print_to_downui!("shell_type: ", obj.shell_type);
                        break;
                    }
                }
            }
        }

        // Рисуем UI
        self.up_ui.render(phi);
        self.down_ui.render(phi);

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
                x: x - OBJECT_SIZE / 2.0,
                y: y - OBJECT_SIZE / 2.0,
                w: OBJECT_SIZE,
                h: OBJECT_SIZE,
            })
            .to_sdl()
            .unwrap())
        .unwrap();
}
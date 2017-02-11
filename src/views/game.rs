use ::phi::{Phi, View, ViewAction};
use ::phi::gfx::{Sprite, CopySprite};
use ::sdl2::pixels::Color;
use ::sdl2::rect::Point;
use ::phi::data::Rectangle;
use ::network::sampleobject::ObjectType;
use ::network::Network;
use ::views::ui::{DownUI, UpUI};
use ::views::camera::Camera;

const CAMERA_SENSITIVITY: f64 = 10000.0;
const ZOOM_SENSITIVITY: f64 = 10.0;
const OBJECT_SIZE: f64 = 16.0;

const FONT_PATH: &'static str = "assets/slkscr.ttf";

pub struct GameView {
    network: Network,
    network_timer: f64,
    camera: Camera,

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
            network_timer: 0.0,
            camera: Camera::new(0.0, 0.0, 0.0, 0.0, width, height),
            up_ui: UpUI::new(phi),
            down_ui: DownUI::new(phi),
            asteroid_sprite: phi.ttf_str_sprite("A", FONT_PATH, 12, Color::RGB(128, 128, 128))
                .unwrap(),
            builder_sprite: phi.ttf_str_sprite("B", FONT_PATH, 12, Color::RGB(0, 0, 200)).unwrap(),
            harvester_sprite: phi.ttf_str_sprite("H", FONT_PATH, 12, Color::RGB(0, 200, 0))
                .unwrap(),
            battlecruiser_sprite: phi.ttf_str_sprite("C", FONT_PATH, 12, Color::RGB(200, 0, 0))
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
            self.network.update_objects();
            self.network.update_info();
            self.network.update_world_size();
            self.network_timer = 0.0;
        }

        let fps = phi.fps;
        self.up_ui.set_fps(phi, fps); // Обновление FPS
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

        // match phi.events.now.left_mouse_click {
        //     None => {}
        //     Some((x, y)) => {
        //         let cursor_rect = Rectangle {
        //             x: x as f64 - OBJECT_SIZE / 2.0 * self.camera.zoom,
        //             y: y as f64 - OBJECT_SIZE / 2.0 * self.camera.zoom,
        //             w: OBJECT_SIZE * self.camera.zoom,
        //             h: OBJECT_SIZE * self.camera.zoom,
        //         };

        //         self.down_ui.clear_data();

        //         macro_rules! print_to_downui {  // Макрос для нижнего UI
        //         ($str1: expr, $str2: expr) => {
        //                 self.down_ui.add_data(phi, $str1.to_string() + &$str2.to_string());
        //             }
        //         }
        //         for obj in self.network.objects.lock().unwrap().iter() {
        //             if cursor_rect.contains_point((obj.x - self.camera.pos_x) * self.camera.zoom,
        //                                           (obj.y - self.camera.pos_y) * self.camera.zoom) {
        //                 print_to_downui!("owner: ", obj.owner);
        //                 print_to_downui!("name: ", obj.name);
        //                 print_to_downui!("otype: ", obj.otype);
        //                 print_to_downui!("x: ", obj.x);
        //                 print_to_downui!("y: ", obj.y);

        //                 print_to_downui!("drive_speed: ", obj.drive_speed);
        //                 print_to_downui!("drive_dest_x: ", obj.drive_dest_x);
        //                 print_to_downui!("drive_dest_y: ", obj.drive_dest_y);
        //                 print_to_downui!("radar_radius: ", obj.radar_radius);
        //                 print_to_downui!("radar_type: ", obj.radar_type);

        //                 print_to_downui!("weapon_active: ", obj.weapon_active);
        //                 print_to_downui!("weapon_type: ", obj.weapon_type);
        //                 print_to_downui!("weapon_radius: ", obj.weapon_radius);
        //                 print_to_downui!("weapon_target_x: ", obj.weapon_target_x);
        //                 print_to_downui!("weapon_target_y: ", obj.weapon_target_y);

        //                 print_to_downui!("cargo_type: ", obj.cargo_type);
        //                 print_to_downui!("cargo_max: ", obj.cargo_max);
        //                 print_to_downui!("cargo_current: ", obj.cargo_current);
        //                 print_to_downui!("shell_health: ", obj.shell_health);
        //                 print_to_downui!("shell_type: ", obj.shell_type);
        //                 break;
        //             }
        //         }
        //     }
        // }

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

fn draw_path(phi: &mut Phi, camera: &Camera, x1: f64, y1: f64, x2: f64, y2: f64) {
    let start = camera.create_point(x1, y1);
    let end = camera.create_point(x2, y2);
    phi.renderer.draw_line(start, end);
}
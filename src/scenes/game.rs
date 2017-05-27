use ::piston::input::*;
use ::piston_window::{PistonWindow, Window};
use ::engine::{Scene, SceneAction};
use ::piston_window::Rectangle;
use ::piston_window::Button::Keyboard;

use ::network::Network;
use ::utils::camera::{Camera, Direction};

const NETWORK_UPDATE_TIMER: f64 = 1.0;
const SPRITE_SIZE: f64 = 10.0;

pub struct GameScene {
    network: Network,
    network_timer: f64,

    camera: Camera,
}

impl GameScene {
    pub fn new(window: &mut PistonWindow, addr: String) -> Self {
        println!("Connecting to {}", addr);
        let draw_size = window.draw_size();
        GameScene {
            network: Network::new(addr),
            network_timer: 0.0,
            camera: Camera::new((draw_size.width as f64, draw_size.height as f64),
                                (1000.0, 1000.0)),
        }
    }
}

impl Scene for GameScene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs) {
        window.draw_2d(&Input::Render(args), |context, graphics| {
            ::piston_window::clear([0.8, 1.0, 0.8, 1.0], graphics);

            for (_, obj) in self.network.objects.lock().unwrap().iter() {
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw(self.camera.transform([obj.x - (SPRITE_SIZE / 2.0),
                                                 obj.y - (SPRITE_SIZE / 2.0),
                                                 SPRITE_SIZE,
                                                 SPRITE_SIZE]),
                          &context.draw_state,
                          context.transform,
                          graphics);
            }
        });
    }
    fn update(&mut self, args: &UpdateArgs) -> SceneAction {
        self.network_timer += args.dt;
        if self.network_timer >= NETWORK_UPDATE_TIMER {
            self.network.update_objects();
            self.network_timer = 0.0;
        }
        SceneAction::None
    }
    fn event(&mut self, event: Input) {
        match event {
            Input::Press(Keyboard(Key::Up)) => self.camera.shift(Direction::Up, 1.0),
            Input::Press(Keyboard(Key::Down)) => self.camera.shift(Direction::Down, 1.0),
            Input::Press(Keyboard(Key::Left)) => self.camera.shift(Direction::Left, 1.0),
            Input::Press(Keyboard(Key::Right)) => self.camera.shift(Direction::Right, 1.0),
            _ => {}
        }
    }
}

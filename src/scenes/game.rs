use ::piston::input::*;
use ::piston_window::PistonWindow;
use ::engine::{Scene, SceneAction};
use ::piston_window::Rectangle;

use ::network::Network;

const NETWORK_UPDATE_TIMER: f64 = 1.0;

pub struct GameScene {
    network: Network,
    network_timer: f64,
}

impl GameScene {
    pub fn new(window: &mut PistonWindow, addr: String) -> Self {
        println!("Connecting to {}", addr);
        GameScene {
            network: Network::new(addr),
            network_timer: 0.0,
        }
    }
}

impl Scene for GameScene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs) {
        window.draw_2d(&Input::Render(args), |context, graphics| {
            ::piston_window::clear([0.8, 1.0, 0.8, 1.0], graphics);

            for (_, obj) in self.network.objects.lock().unwrap().iter() {
                Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([obj.x, obj.y, 10.0, 10.0],
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
    fn event(&mut self, event: Input) {}
}

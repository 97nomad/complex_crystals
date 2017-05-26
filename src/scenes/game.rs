use ::piston::input::*;
use ::piston_window::PistonWindow;
use ::engine::{Scene, SceneAction};

pub struct GameScene;

impl GameScene {
    pub fn new(window: &mut PistonWindow, addr: String) -> Self {
        println!("Connecting to {}", addr);
        GameScene {}
    }
}

impl Scene for GameScene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs) {
        window.draw_2d(&Input::Render(args), |context, graphics| {
            ::piston_window::clear([0.8, 1.0, 0.8, 1.0], graphics);
        });
    }
    fn update(&mut self, args: &UpdateArgs) -> SceneAction {
        SceneAction::None
    }
    fn event(&mut self, event: Input) {}
}

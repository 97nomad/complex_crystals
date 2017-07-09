use piston_window::{PistonWindow, WindowSettings, OpenGL};
use piston::event_loop::*;
use piston::input::*;

use scenes::main_menu::MainMenuScene;
use scenes::game::GameScene;
use network::{ServerClient, ServerManager};

pub struct Engine {
    pub window: PistonWindow,
    pub scene: Box<Scene>,
    pub events: Events,
}

impl Engine {
    pub fn new(mut window: PistonWindow) -> Self {
        let scene = Box::new(MainMenuScene::new(&mut window));
        Engine {
            window: window,
            scene: scene,
            events: Events::new(EventSettings::new()),
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        self.scene.render(&mut self.window, args);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.scene.update(args) {
            SceneAction::None => {}
            SceneAction::ConnectToServer(addr) => {
                self.scene = if let Some(scene) =
                    GameScene::new(&mut self.window, Box::new(ServerClient::new(addr))) {
                    Box::new(scene)
                } else {
                    Box::new(MainMenuScene::new(&mut self.window))
                }
            }
            SceneAction::StartServer(opt) => {
                self.scene = if let Some(scene) =
                    GameScene::new(&mut self.window,
                                   Box::new(ServerManager::new(1000.0, 1000.0, Vec::new()))) {
                    Box::new(scene)
                } else {
                    Box::new(MainMenuScene::new(&mut self.window))
                }
            }
        }
    }

    pub fn event(&mut self, event: Input) {
        self.scene.event(event);
    }

    pub fn start_loop(&mut self) {
        while let Some(event) = self.events.next(&mut self.window) {
            match event {
                Input::Render(args) => self.render(args),
                Input::Update(args) => self.update(&args),
                _ => self.event(event),
            }
        }
    }
}

pub enum SceneAction {
    None,
    ConnectToServer(String),
    StartServer((isize, isize, isize)),
}

pub trait Scene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs);
    fn update(&mut self, args: &UpdateArgs) -> SceneAction;
    fn event(&mut self, event: Input);
}

pub fn spawn() {
    let width = 800;
    let height = 600;

    let opengl = OpenGL::V3_2;

    let window: PistonWindow = WindowSettings::new("Complex Crystals Client", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut engine = Engine::new(window);

    engine.start_loop();
}

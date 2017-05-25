use ::piston_window::WindowSettings;
use ::piston::event_loop::*;
use ::piston::input::*;
use ::piston_window::PistonWindow;
use ::piston_window::OpenGL;

use ::scenes::main_menu::MainMenuScene;

pub struct Engine {
    pub window: PistonWindow,
    pub scene: Box<Scene>,
    pub events: Events,
}

impl Engine {
    pub fn new(window: PistonWindow, scene: Box<Scene>) -> Self {
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
            SceneAction::Quit => {}
            SceneAction::ChangeScene(scene) => self.scene = scene,
        }
    }

    pub fn start_loop(&mut self) {
        while let Some(event) = self.events.next(&mut self.window) {
            match event {
                Input::Render(args) => self.render(args),
                Input::Update(args) => self.update(&args),
                _ => {}
            }
        }
    }
}

pub enum SceneAction {
    None,
    Quit,
    ChangeScene(Box<Scene>),
}

pub trait Scene {
    fn render(&mut self, window: &mut PistonWindow, args: RenderArgs);
    fn update(&mut self, args: &UpdateArgs) -> SceneAction;
}

pub fn spawn() {
    let width = 800;
    let height = 600;

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Complex Crystals Client", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let scene = Box::new(MainMenuScene::new(&mut window));
    let mut engine = Engine::new(window, scene);

    engine.start_loop();
}

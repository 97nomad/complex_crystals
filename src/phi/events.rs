use sdl2::mouse::Mouse;
use sdl2::EventPump;

pub struct ImmediateEvents {
    pub key_escape: bool,
    pub key_up: bool,
    pub key_down: bool,
    pub key_left: bool,
    pub key_right: bool,
    pub key_space: bool,
    pub key_a: bool,
    pub key_z: bool,
    pub quit: bool,
    pub resize: Option<(u32, u32)>,
    pub left_mouse_click: Option<(i32, i32)>,
    pub mouse_wheel: i32,
}

impl ImmediateEvents {
    pub fn new() -> ImmediateEvents {
        ImmediateEvents {
            key_escape: false,
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,
            key_space: false,
            key_a: false,
            key_z: false,
            quit: false,
            resize: None,
            left_mouse_click: None,
            mouse_wheel: 0,
        }
    }
}


pub struct Events {
    pump: EventPump,
    pub now: ImmediateEvents,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,
            now: ImmediateEvents::new(),
        }
    }

    pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
        self.now = ImmediateEvents::new();

        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Window { win_event_id: _, .. } => {
                    self.now.resize = Some(renderer.output_size().unwrap());
                }

                MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == Mouse::Left {
                        self.now.left_mouse_click = Some((x, y));
                    }
                }

                MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == Mouse::Left {
                        self.now.left_mouse_click = None;
                    }
                }

                MouseWheel { y, .. } => {
                    self.now.mouse_wheel = y;
                }

                KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Escape) => self.now.key_escape = true,
                        Some(Up) => self.now.key_up = true,
                        Some(Down) => self.now.key_down = true,
                        Some(Left) => self.now.key_left = true,
                        Some(Right) => self.now.key_right = true,
                        Some(Space) => self.now.key_space = true,
                        Some(A) => self.now.key_a = true,
                        Some(Z) => self.now.key_z = true,
                        _ => {}
                    }
                }

                KeyUp { keycode, .. } => {
                    match keycode {
                        Some(Escape) => self.now.key_escape = false,
                        Some(Up) => self.now.key_up = false,
                        Some(Down) => self.now.key_down = false,
                        Some(Left) => self.now.key_left = false,
                        Some(Right) => self.now.key_right = false,
                        Some(Space) => self.now.key_space = false,
                        Some(A) => self.now.key_a = false,
                        Some(Z) => self.now.key_z = false,
                        _ => {}
                    }
                }

                Quit { .. } => self.now.quit = true,

                _ => {}
            }
        }
    }
}
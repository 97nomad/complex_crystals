const CAMERA_SPEED: f64 = 10.0;

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

pub struct Camera {
    viewport: (f64, f64),
    position: (f64, f64),
    world_size: (f64, f64),
    speed: f64,
}

impl Camera {
    pub fn new(viewport: (f64, f64), world_size: (f64, f64)) -> Self {
        Camera { viewport, 
            position: (0.0,0.0), 
            world_size,
            speed: CAMERA_SPEED, }
    }

    pub fn transform(&self, input: [f64; 4]) -> [f64; 4] {
        [input[0] - self.position.0,
        input[1] - self.position.1,
        input[2],
        input[3]]
    }

    pub fn transform_cursor(&self, input: [f64; 2]) -> [f64; 2] {
        [input[0] + self.position.0,
        input[1] + self.position.1]
    }

    pub fn shift(&mut self, dir: Direction, dt: f64) {
        match dir {
            Direction::Left => self.position.0 -= dt * self.speed,
            Direction::Right => self.position.0 += dt * self.speed,
            Direction::Up => self.position.1 -= dt * self.speed,
            Direction::Down => self.position.1 += dt * self.speed,
        }
        self.check_position();
    }

    fn check_position(&mut self) {
        if self.position.0 < 0.0 { self.position.0 = 0.0 }
        if self.position.1 < 0.0 { self.position.1 = 0.0 }
        if self.position.0 > self.world_size.0 - self.viewport.0 { 
            self.position.0 = self.world_size.0 - self.viewport.0 
        }
        if self.position.1 > self.world_size.1 - self.viewport.1 { 
            self.position.1 =  self.world_size.1 - self.viewport.1 
        }
    }
}

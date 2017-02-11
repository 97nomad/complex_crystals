use ::phi::data::Rectangle;
use ::sdl2::rect::Point;

const ZOOM_MIN: f64 = 0.01;
const BORDER: f64 = 50.0;

pub struct Camera {
    pub pos_x: f64,
    pub pos_y: f64,
    pub width: f64,
    pub height: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub zoom: f64,
}

impl Camera {
    pub fn new(x: f64, y: f64, max_x: f64, max_y: f64, width: f64, height: f64) -> Self {
        Camera {
            pos_x: x,
            pos_y: y,
            width: width,
            height: height,
            max_x: max_x,
            max_y: max_y,
            zoom: 1.0,
        }
    }
    pub fn resize(&mut self, width: f64, height: f64) {
        self.max_x = width;
        self.max_y = height;
        self.check();
    }

    pub fn move_left(&mut self, d: f64) {
        self.pos_x -= d;
        self.check();
    }
    pub fn move_right(&mut self, d: f64) {
        self.pos_x += d;
        self.check();
    }
    pub fn move_up(&mut self, d: f64) {
        self.pos_y -= d;
        self.check();
    }
    pub fn move_down(&mut self, d: f64) {
        self.pos_y += d;
        self.check();
    }

    pub fn zoom(&mut self, d: f64) {
        self.zoom += d;
        if self.zoom < ZOOM_MIN {
            self.zoom = ZOOM_MIN;
        }
        self.check();
    }

    fn check(&mut self) {
        // Нижняя граница
        if self.pos_y > self.max_y - self.height / self.zoom + BORDER {
            self.pos_y = self.max_y - self.height / self.zoom + BORDER;
        }

        // Правая граница
        if self.pos_x * self.zoom > self.max_x - self.width / self.zoom + BORDER {
            self.pos_x = self.max_x - self.width / self.zoom + BORDER;
        }

        // Верхняя граница
        if self.pos_y < 0.0 - BORDER {
            self.pos_y = 0.0 - BORDER;
        }

        // Левая граница
        if self.pos_x < 0.0 - BORDER {
            self.pos_x = 0.0 - BORDER;
        }
    }

    pub fn translate_rect(&self, rect: Rectangle) -> Rectangle {
        Rectangle {
            x: (rect.x - self.pos_x) * self.zoom,
            y: (rect.y - self.pos_y) * self.zoom,
            w: rect.w * self.zoom,
            h: rect.h * self.zoom,
        }
    }
    pub fn create_point(&self, x: f64, y: f64) -> Point {
        Point::new(((x - self.pos_x) * self.zoom) as i32,
                   ((y - self.pos_y) * self.zoom) as i32)
    }
}
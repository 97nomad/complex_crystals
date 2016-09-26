use ::phi::data::Rectangle;

const ZOOM_MIN: f64 = 0.01;

pub struct Camera {
    pub pos_x: f64,
    pub pos_y: f64,
    pub zoom: f64,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos_x: 0.0,
            pos_y: 0.0,
            zoom: 1.0,
        }
    }

    pub fn move_left(&mut self, d: f64) {
        self.pos_x -= d;
        if self.pos_x < 0.0 {
            self.pos_x = 0.0;
        }
    }
    pub fn move_right(&mut self, d: f64) {
        self.pos_x += d;
    }
    pub fn move_up(&mut self, d: f64) {
        self.pos_y -= d;
        if self.pos_y < 0.0 {
            self.pos_y = 0.0;
        }
    }
    pub fn move_down(&mut self, d: f64) {
        self.pos_y += d;
    }

    pub fn zoom_in(&mut self, d: f64) {
        self.zoom += d;
    }

    pub fn zoom_out(&mut self, d: f64) {
        self.zoom -= d;
        if self.zoom < ZOOM_MIN {
            self.zoom = ZOOM_MIN;
        }
    }

    pub fn translate_rect(&self, rect: Rectangle) -> Rectangle {
        Rectangle {
            x: (rect.x + self.pos_x) * self.zoom,
            y: (rect.y + self.pos_y) * self.zoom,
            w: rect.w * self.zoom,
            h: rect.h * self.zoom,
        }
    }
}
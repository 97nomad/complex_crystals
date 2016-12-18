use ::sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn to_sdl(self) -> Option<SdlRect> {
        assert!(self.w >= 0.0 && self.h >= 0.0);

        Some(SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32))
    }

    pub fn in_center(self, width: f64, height: f64) -> Rectangle {
        Rectangle {
            x: self.x + (self.w / 2.0) - (width / 2.0),
            y: self.y + (self.h / 2.0) - (height / 2.0),
            w: width,
            h: height,
        }
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}
use wasm_bindgen::prelude::*;

use crate::types::Rect;

/// A type representing the state of Mouse events
#[wasm_bindgen]
#[derive(Debug)]
pub struct Mouse {
    rect: Rect,
    down: bool,
    up: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0.0, 0.0, 1.0, 1.0),
            down: false,
            up: false,
        }
    }

    pub fn x(&self) -> f64 {
        self.rect.x()
    }
    pub fn y(&self) -> f64 {
        self.rect.y()
    }
    pub fn down(&self) -> bool {
        self.down
    }
    pub fn up(&self) -> bool {
        self.up
    }

    pub fn update(&mut self, x: f64, y: f64, mouse_down: bool, mouse_up: bool) {
        self.rect.set_pos(x, y);
        self.down = mouse_down;
        self.up = mouse_up;
    }

    /// Returns true if the mouse xy coordinates are inside of the referenced Rect
    pub fn inside(&self, rect: &Rect) -> bool {
        self.x() > rect.x()
            && self.y() > rect.y()
            && self.x() < rect.x() + rect.w()
            && self.y() < rect.y() + rect.h()
    }
}

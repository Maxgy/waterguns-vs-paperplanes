use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::types::Rect;

#[wasm_bindgen]
pub struct PaperPlane {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    center_x: f64,
    center_y: f64,
    hp: i32,
    max_hp: u32,
}

#[wasm_bindgen]
impl PaperPlane {
    pub fn new(rect: Rect, hp: i32) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: rect.h,
            center_x: rect.x + rect.w / 2.0,
            center_y: rect.y + rect.h / 2.0,
            hp,
            max_hp: hp as u32,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn w(&self) -> f64 {
        self.w
    }
    pub fn h(&self) -> f64 {
        self.h
    }
    pub fn center_x(&self) -> f64 {
        self.center_x
    }
    pub fn center_y(&self) -> f64 {
        self.center_y
    }

    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn hp_percent(&self) -> f64 {
        self.hp as f64 / self.max_hp as f64
    }

    pub fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    pub fn fly(&mut self) {
        self.x += 1.7;
    }

    pub fn draw(
        &self,
        ctx: &CanvasRenderingContext2d,
        img: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element_and_dw_and_dh(img, self.x, self.y, self.w, self.h)?;
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("#00ff00"));
        ctx.fill_rect(self.x, self.y, self.w * self.hp_percent(), 3.0);
        ctx.close_path();
        Ok(())
    }
}

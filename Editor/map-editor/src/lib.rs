mod utils;
mod canvas;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct Offset {
    pub x: u32,
    pub y: u32
}

impl From<(u32, u32)> for Offset {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct CanvasContext {
    pub offset: Offset,
    pub scale: f32
}

impl Default for CanvasContext {
    fn default() -> Self {
        Self {
            scale: 1.0,
            offset: Offset::default()
        }
    }
}

#[wasm_bindgen]
pub struct Map {
    map: map::Map,
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    pub view: CanvasContext
}

#[wasm_bindgen]
impl Map {
    pub fn move_view(&mut self, move_x: u32, move_y: u32) {
        self.view.offset.x += move_x;
        self.view.offset.y += move_y;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.view.scale = scale;
    }
    
    pub fn get_scale(&self) -> f32 {
        self.view.scale
    }

    pub fn update_canvas(&self) {
        canvas::draw(self);
    }
}

#[wasm_bindgen]
pub fn init() -> Map {
    let document = web_sys::window()
        .expect("WHERE THE FUCK IS THIS WINDOW")
        .document()
        .expect("No document? Go fuck your web browser");

    let canvas = document.get_element_by_id("map_canvas").expect("No canvas found, you're fucked mate");

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("Well, that didn't worked as expected");

    let ctx = canvas
        .get_context("2d")
        .expect("Cannot define the context for the canvas")
        .expect("Cannot build the context an Object")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Cannot convert the Object into a 2D Canvas Rendering Context");

    Map {
        canvas, ctx,
        map: map::Map::default(),
        view: CanvasContext::default()
    }
}
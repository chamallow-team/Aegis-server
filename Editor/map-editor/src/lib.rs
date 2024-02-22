mod utils;
mod canvas;

use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use map::map::{Node, NodeType};
use uuid::Uuid;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Default)]
pub struct Offset {
    pub x: i32,
    pub y: i32
}

impl From<(i32, i32)> for Offset {
    fn from(value: (i32, i32)) -> Self {
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
    pub fn move_view(&mut self, move_x: i32, move_y: i32) {
        self.view.offset.x += move_x;
        self.view.offset.y += move_y;
    }

    pub fn set_offset(&mut self, x: i32, y: i32) {
        self.view.offset = (x, y).into();
    }

    pub fn get_offset(&self) -> Offset {
        self.view.offset
    }

    pub fn set_scale(&mut self, scale: f32) {
        match scale {
            s if s > 0.1 && s < 1.5 => self.view.scale = s,
            _ => ()
        };

        if self.view.scale < 0.2 {
            self.view.scale = 0.2;
        } else if self.view.scale > 1.5 {
            self.view.scale = 1.5;
        }
    }
    
    pub fn get_scale(&self) -> f32 {
        self.view.scale
    }

    pub fn update_canvas(&self) {
        canvas::draw(self);
    }

    pub fn add_node(&mut self, node_type: u32, x: i32, y: i32) -> bool {
        match NodeType::try_from(node_type) {
            Ok(nt) => {
                self.map
                    .get_map_mut()
                    .add_node(
                        Node::new(
                            nt,
                            (x, y),
                            Uuid::new_v4()
                        )
                    );
                true
            }
            _ => false
        }
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
#![allow(unused)]

mod utils;
mod vehicle_480;
mod state;

use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};
use serde_json::{Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::{Arc, Mutex};
use cgmath::prelude::*;
use cgmath::Rad;
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::{TryInto};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use gloo_console::log;
use std::f32::consts::PI;

use crate::utils::time_polyfill::Instant;

const AMORTIZATION: f32 = 0.95;
const LOCALIZED_SCALE : f32 = 0.001;
const CORRECTION : f32 = LOCALIZED_SCALE / 2.0;
const RESOLUTION : f32 = 8.0;
const SCALE : f32 = 0.08;
const HALF : f32 = SCALE / 2.0;
const STEP : f32 = SCALE / RESOLUTION;
const NUM_PARTICLES : u32 = 9680;

fn main()
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas33").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let gl: GL = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<GL>()
        .unwrap();
    let gl : Arc<GL> = Arc::new(gl);

    let draw_stuff = vehicle_480::prepare_draw(gl.clone()).unwrap();
    
    let state = Arc::new(Mutex::new(state::State {
        x_rot: cgmath::Rad(0.0),
        y_rot: cgmath::Rad(0.0),
        z_rot: cgmath::Rad(0.0),
    }));

    state::set_events(state.clone());
       
    let start_time = Instant::now();
    let mut cursor: u128 = start_time.elapsed().as_millis();

    gl.clear_color(0.993, 0.9833, 0.952, 1.0);
    gl.enable(GL::DEPTH_TEST);

    let render_loop_closure = Rc::new(RefCell::new(None));
    let alias_rlc = render_loop_closure.clone();
    *alias_rlc.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        let now = start_time.elapsed().as_millis();  // total elapsed time from start
        let frame_delta = now - cursor;
        cursor = now;

        gl.clear_depth(1.0); // Clear everything
        gl.enable(GL::DEPTH_TEST); // Enable depth testing
        gl.depth_func(GL::LEQUAL);

        gl.clear(GL::COLOR_BUFFER_BIT);

        vehicle_480::draw(
            gl.clone(),
            draw_stuff.clone(),
            state.clone(),
        );

        request_animation_frame(render_loop_closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(alias_rlc.borrow().as_ref().unwrap());    
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}



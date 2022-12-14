#![allow(unused)]

mod utils;
mod vehicle_480;
mod state;
mod terrain;
mod grid;

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
use cgmath::{Rad, Vector3, Vector4, Point3, Matrix4};
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::{TryInto};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use gloo_console::log;
use std::f32::consts::PI;

use crate::utils::time_polyfill::Instant;

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

    let vehicle_draw_stuff = vehicle_480::prepare_draw(gl.clone()).unwrap();
    
    let terrain_draw_stuff = terrain::prepare_draw(gl.clone()).unwrap();

    let grid_draw_stuff = grid::prepare_draw(gl.clone()).unwrap();


    let state = Arc::new(Mutex::new(state::State {

        model_scale: 2.0,

        // model choose spear front to be in the positive x axis to start
        model_rot: Vector4::new(Rad(0.0), Rad(0.0), Rad(0.0), Rad(0.0)),
        model_trans: Vector4::new(0.0, 0.0, -0.2, 1.0),


        camera_rot: Vector4::new(Rad(0.0), Rad(0.0), Rad(0.0), Rad(0.0)),
        camera_trans: Vector4::new(0.0, 0.0, 0.5, 1.0),

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

        grid::draw(
            gl.clone(),
            grid_draw_stuff.clone(),
            state.clone(),
        );

        vehicle_480::draw(
            gl.clone(),
            vehicle_draw_stuff.clone(),
            state.clone(),
        );

        terrain::draw(
            gl.clone(),
            terrain_draw_stuff.clone(),
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



#![allow(unused)]

use crate::utils;
use crate::utils::time_polyfill::Instant;

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
use cgmath::{Vector3, Vector4, Matrix4, Rad, Point3};

use std::time::*;
use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::{TryInto};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::f32::consts::PI;

use gloo_console::log;

pub fn set_events
(
    state: Arc<Mutex<State>>,
)
{

    let document = web_sys::window().unwrap().document().unwrap();
    let et_keys : EventTarget = document.into();
    let keypress_cb = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        // log!("key code", event.key_code());
        match event.key_code() {
            39 => state.lock().unwrap().model_rot[1] -= Rad(0.1),
            38 => state.lock().unwrap().model_rot[0] += Rad(0.1),
            37 => state.lock().unwrap().model_rot[1] += Rad(0.1),
            40 => state.lock().unwrap().model_rot[0] -= Rad(0.1),
            186 => state.lock().unwrap().model_rot[2] += Rad(0.1),
            81 => state.lock().unwrap().model_rot[2] -= Rad(0.1),



            87 => state.lock().unwrap().camera_rot[0] -= Rad(0.1), // left camera axis 
            90 => state.lock().unwrap().camera_rot[0] += Rad(0.1),// right camera axis
            83 => state.lock().unwrap().camera_rot[1] += Rad(0.1), // 
            86 => state.lock().unwrap().camera_rot[1] -= Rad(0.1),
            66 => state.lock().unwrap().camera_rot[2] += Rad(0.1),
            77 => state.lock().unwrap().camera_rot[2] -= Rad(0.1),

            56 => state.lock().unwrap().model_trans[0] -= 0.05, // up 8
            70 => state.lock().unwrap().model_trans[1] += 0.05, // left y
            71 => state.lock().unwrap().model_trans[0] += 0.05,// down u
            67 => state.lock().unwrap().model_trans[1] -= 0.05,// right i
            80 => state.lock().unwrap().model_trans[2] += 0.05,
            89 => state.lock().unwrap().model_trans[2] -= 0.05,

            _ => (),
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);
    et_keys
        .add_event_listener_with_callback("keydown", keypress_cb.as_ref().unchecked_ref())
        .unwrap();
    keypress_cb.forget();
}



pub struct State {

    pub model_scale: f32, // model scale into world space dimensions.
    pub model_rot: Vector4<Rad<f32>>,
    pub model_trans: Vector4<f32>,

    pub camera_rot: Vector4<cgmath::Rad<f32>>, //x, y, z axis rotations
    pub camera_trans: Vector4<f32> // x, y, z, displacements

}
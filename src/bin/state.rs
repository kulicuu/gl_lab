
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
use cgmath::Rad;

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
        log!("key code", event.key_code());
        match event.key_code() {
            39 => state.lock().unwrap().y_rot -= Rad(0.1),
            38 => state.lock().unwrap().x_rot += Rad(0.1),
            37 => state.lock().unwrap().y_rot += Rad(0.1),
            40 => state.lock().unwrap().x_rot -= Rad(0.1),
            186 => state.lock().unwrap().z_rot += Rad(0.1),
            81 => state.lock().unwrap().z_rot -= Rad(0.1),
            _ => (),
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);
    et_keys
        .add_event_listener_with_callback("keydown", keypress_cb.as_ref().unchecked_ref())
        .unwrap();
    keypress_cb.forget();

}



pub struct State {
    pub x_rot: cgmath::Rad<f32>,
    pub y_rot: cgmath::Rad<f32>,
    pub z_rot: cgmath::Rad<f32>,
}
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
use cgmath::{Rad, Point3, Vector3, Vector4, Matrix4};
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::{TryInto};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use gloo_console::log;
use std::f32::consts::PI;

use crate::utils::time_polyfill::Instant;
use crate::state;

const AMORTIZATION: f32 = 0.95;
const LOCALIZED_SCALE : f32 = 0.001;
const CORRECTION : f32 = LOCALIZED_SCALE / 2.0;
const RESOLUTION : f32 = 8.0;
const SCALE : f32 = 0.08;
const HALF : f32 = SCALE / 2.0;
const STEP : f32 = SCALE / RESOLUTION;
const NUM_PARTICLES : u32 = 9680;








pub fn prepare_draw
(
    gl: Arc<GL>,
)
-> Result<Arc<DrawStuff>, String>
{
    let mut indexed_vertices = vec![];

    // lets make a simple generative terrain.
    // random elevation gain or loss within some parameters
    // random vertex placement within some parameters.

    // First we can just make a grid of vertices in a plane,
    // deviate their x, y pos within some parameters and the
    // z value also for elevation.
    // we'll start with 100 * 100 vertices, so 10000 vertices.

    // These will cover -0.3, 0.3


    let resolution = 100;
    let terrain_size = 0.6;
    let start_point = -0.3;
    let end_point = 0.3;

    let s1 = terrain_size / (resolution as f32); 

    let s2 = s1 / 10.0;

    // start from -0.3 and go to 0.3 in x and y directions

    let mut indices: Vec<u32> = vec![];


    for idx in 0..resolution {
        for jdx in 0..resolution {
            let x_dev = js_sys::Math::random() as f32;
            let y_dev = js_sys::Math::random() as f32;
            let z_dev = js_sys::Math::random() as f32;

            let x = start_point + ((idx as f32) * s1) + (x_dev * s2);
            let y = start_point +((jdx as f32) * s1) * (y_dev * s2);
            let z = (z_dev * s2) as f32;

            indexed_vertices.extend_from_slice(&[x, y, z]);
        }
    }

    // so given this grid of resolution ^ 2, we need an index vector
    // expressing all the triangles to make the mesh.

    
    for i in 0..(resolution - 1) {
        for j in 0..(resolution - 1) {
            // These two triangles for every i, j

            indices.extend_from_slice(&[
                i + j, (i + 1) + j, i + (j * resolution),
                i + (j * resolution), (i + 1) + (j * resolution), (i + 1) + j,
            ]);
        }
    }


    let vert_code = include_str!("../shaders/terrain.vert");
    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);
    let vert_shader_log = gl.get_shader_info_log(&vert_shader);
    log!("terrain.vert shader compilation log: ", vert_shader_log);

    




    Err(std::string::String::from("err"))

}



#[derive(Clone)]
pub struct DrawStuff {
    pub shader_program: Arc<web_sys::WebGlProgram>,
    pub stuff_uniform_buffer: Arc<WebGlBuffer>,
    pub index_buffer: Arc<WebGlBuffer>,
    pub js_indices: Arc<js_sys::Int32Array>,
    pub indexed_vertex_buffer: Arc<WebGlBuffer>,
    pub indexed_js_vertices: Arc<js_sys::Float32Array>,
    pub normals_buffer: Arc<WebGlBuffer>,
    pub js_normals: Arc<js_sys::Float32Array>,
}
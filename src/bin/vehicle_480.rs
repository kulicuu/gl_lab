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
use crate::state;

const AMORTIZATION: f32 = 0.95;
const LOCALIZED_SCALE : f32 = 0.001;
const CORRECTION : f32 = LOCALIZED_SCALE / 2.0;
const RESOLUTION : f32 = 8.0;
const SCALE : f32 = 0.08;
const HALF : f32 = SCALE / 2.0;
const STEP : f32 = SCALE / RESOLUTION;
const NUM_PARTICLES : u32 = 9680;

pub fn draw
(
    gl: Arc<GL>,
    draw_stuff: Arc<DrawStuff>,
    state: Arc<Mutex<state::State>>,
)
{
    gl.use_program(Some(&draw_stuff.shader_program));
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&draw_stuff.vertex_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &draw_stuff.js_vertices, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(0 as u32, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0 as u32);




    // let x_rot = cgmath::Matrix4::from_angle_x(cgmath::Rad(0.75 * (PI / 2.0)));
    // let y_rot = cgmath::Matrix4::from_angle_y(cgmath::Rad(0.0 * (PI / 2.0)));

    let x_rot = cgmath::Matrix4::from_angle_x(state.lock().unwrap().x_rot);
    let y_rot = cgmath::Matrix4::from_angle_y(state.lock().unwrap().y_rot);
    let z_rot = cgmath::Matrix4::from_angle_z(state.lock().unwrap().z_rot);

    let scale = cgmath::Matrix4::from_scale(5.0);

    let all_rot = x_rot * y_rot * z_rot * scale;

    let view_mat = Arc::new(all_rot);

    let mut arr: [f32; 20] = [0.0; 20];

    let mut kdx: usize = 0;
    for idx in 0..4 {
        for jdx in 0..4 {
            arr[kdx] = view_mat[idx][jdx];
            // log!("view_mat: ", view_mat[idx][jdx]);
            kdx = kdx + 1;
        }
    }





    gl.bind_buffer_base(GL::UNIFORM_BUFFER, 0, Some(&draw_stuff.stuff_uniform_buffer.as_ref()));
    let arr_js = js_sys::Float32Array::from(arr.as_slice());
    gl.buffer_data_with_array_buffer_view(GL::UNIFORM_BUFFER, &arr_js, GL::STATIC_DRAW);

    gl.draw_arrays(GL::TRIANGLES, 0, 21);
    gl.bind_buffer(GL::ARRAY_BUFFER, None);

}

pub fn prepare_draw
(
    gl: Arc<GL>
)
-> Result<Arc<DrawStuff>, String>
{

    let v1 = vec![0.021, 0.0, 0.0];
    let v2 = vec![0.0, 0.021, 0.0];
    let v3 = vec![0.0, -0.021, 0.0];

    let v4 = vec![0.0, 0.0, 0.0];
    let v5 = vec![-0.031, 0.031, 0.0];
    let v6 = vec![-0.031, -0.031, 0.0];

    let v7 = vec![0.0, 0.0, 0.011];

    let mut vertices: Vec<f32> = vec![];

    vertices.extend(&v1);
    vertices.extend(&v2);
    vertices.extend(&v3);

    vertices.extend(&v4);
    vertices.extend(&v5);
    vertices.extend(&v2);

    vertices.extend(&v4);
    vertices.extend(&v3);
    vertices.extend(&v6);

    vertices.extend(&v7);
    vertices.extend(&v1);
    vertices.extend(&v2);
    
    vertices.extend(&v7);
    vertices.extend(&v1);
    vertices.extend(&v3);

    vertices.extend(&v7);
    vertices.extend(&v5);
    vertices.extend(&v2);

    vertices.extend(&v7);
    vertices.extend(&v3);
    vertices.extend(&v6);

    let vert_code = include_str!("../shaders/vehicles/vehicle_480.vert");
    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);
    let vert_shader_log = gl.get_shader_info_log(&vert_shader);
    log!("vehicle_480.vert compilation log: ", vert_shader_log);

    let frag_code = include_str!("../shaders/vehicles/vehicle_480.frag");
    let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
    gl.shader_source(&frag_shader, frag_code);
    gl.compile_shader(&frag_shader);
    let frag_shader_log = gl.get_shader_info_log(&frag_shader);
    log!("vehicle_480.frag compilation log: ", frag_shader_log);

    let shader_program = Arc::new(gl.create_program().unwrap());
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);


    let vertex_buffer = Arc::new(gl.create_buffer().unwrap());
    let js_vertices = Arc::new(js_sys::Float32Array::from(vertices.as_slice()));
    let vertices_position = Arc::new(gl.get_attrib_location(&shader_program, "a_position") as i32);


    let stuff_uniforms_loc = Arc::new(gl.get_uniform_block_index(&shader_program, "Stuff"));
    gl.uniform_block_binding(&shader_program, *stuff_uniforms_loc, 0);

    let stuff_uniform_buffer = Arc::new(gl.create_buffer().unwrap());
    
    let eye: cgmath::Point3<f32> = cgmath::Point3::new(0.0, 0.0, 1.0);
    let center: cgmath::Point3<f32> = cgmath::Point3::new(0.0, 0.0, -1.0);
    let dir: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 0.0, 0.0);
    let up: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 0.0, 1.0);
    
    let x_rot = cgmath::Matrix4::from_angle_x(cgmath::Rad(0.75 * (PI / 2.0)));
    let y_rot = cgmath::Matrix4::from_angle_y(cgmath::Rad(0.0 * (PI / 2.0)));
    let z_rot = cgmath::Matrix4::from_angle_z(cgmath::Rad(0.0 * (PI / 2.0)));

    let scale = cgmath::Matrix4::from_scale(5.0);

    let all_rot = x_rot * y_rot * z_rot * scale;

    let view_mat = Arc::new(all_rot);

    // let view_mat: Arc<cgmath::Matrix4<f32>> = Arc::new(cgmath::Matrix4::from_angle_y(cgmath::Rad(0.0 * (PI / 2.0))));

    // let mut kdx: usize = 0;
    // for idx in 0..4 {
    //     for jdx in 0..4 {
    //         // arr[kdx] = view_mat[idx][jdx];
    //         // arr[kdx] = -0.3;
    //         log!("idx:", idx, "jdx", jdx);
    //         log!("view_mat: ", view_mat[idx][jdx]);
    //     }
    // }


    let mut arr: [f32; 20] = [0.0; 20];

    let mut kdx: usize = 0;
    for idx in 0..4 {
        for jdx in 0..4 {
            arr[kdx] = view_mat[idx][jdx];
            // log!("view_mat: ", view_mat[idx][jdx]);
            kdx = kdx + 1;
        }
    }

    Ok(
        Arc::new(
            DrawStuff {
                shader_program,
                vertex_buffer,
                js_vertices,
                vertices_position,
                stuff_uniform_buffer,
                view_mat,
                arr,
            }
        )
    )
}

#[derive(Clone)]
pub struct DrawStuff {
    pub shader_program: Arc<web_sys::WebGlProgram>,
    pub vertex_buffer: Arc<WebGlBuffer>,
    pub js_vertices: Arc<js_sys::Float32Array>,
    pub vertices_position: Arc<i32>,
    pub stuff_uniform_buffer: Arc<WebGlBuffer>,
    pub view_mat: Arc<cgmath::Matrix4<f32>>,
    pub arr: [f32; 20],

}
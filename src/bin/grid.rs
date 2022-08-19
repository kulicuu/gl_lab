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

pub fn draw
(
    gl: Arc<GL>,
    draw_stuff: Arc<DrawStuff>,
    state: Arc<Mutex<state::State>>,
)
{
    gl.link_program(&draw_stuff.shader_program);
    gl.use_program(Some(&draw_stuff.shader_program));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&draw_stuff.vertex_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &draw_stuff.js_vertices, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(0 as u32, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0 as u32);

    let start_center = Vector4::new(-0.1, 0.0, 0.0, 1.0);
    let start_up = Vector4::new(0.0, 0.0, 1.0, 1.0);
    let rx = Matrix4::from_angle_x(state.lock().unwrap().camera_rot[0]);
    let ry = Matrix4::from_angle_y(state.lock().unwrap().camera_rot[1]);
    let rz = Matrix4::from_angle_z(state.lock().unwrap().camera_rot[2]);
    let rots = rx * ry * rz;

    let eye = state.lock().unwrap().camera_trans;
    let center = rots * start_center;
    let up = rots * start_up; 

    let view_matrix = Matrix4::look_at_rh(Point3::from_vec(eye.truncate()), Point3::from_vec(center.truncate()), up.truncate()); // to camera space


    let mut arr: [f32; 50] = [0.0; 50];
    let mut kdx: usize = 0;
    for idx in 0..4 {
        for jdx in 0..4 {
            arr[kdx] = view_matrix[idx][jdx];
            kdx = kdx + 1;
        }
    }

    gl.bind_buffer_base(GL::UNIFORM_BUFFER, 0, Some(&draw_stuff.stuff_uniform_buffer.as_ref()));
    let arr_js = js_sys::Float32Array::from(arr.as_slice());
    gl.buffer_data_with_array_buffer_view(GL::UNIFORM_BUFFER, &arr_js, GL::STATIC_DRAW);



    gl.draw_arrays(GL::LINES, 0, 6);
    gl.bind_buffer(GL::ARRAY_BUFFER, None);
}

pub fn prepare_draw
(
    gl: Arc<GL>,
)
-> Result<Arc<DrawStuff>, String>
{




    let vert_code = include_str!("../shaders/grid.vert");
    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);
    let vert_shader_log = gl.get_shader_info_log(&vert_shader);
    log!("grid.vert shader compilation log: ", vert_shader_log);

    let frag_code = include_str!("../shaders/grid.frag");
    let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
    gl.shader_source(&frag_shader, frag_code);
    gl.compile_shader(&frag_shader);
    let frag_shader_log = gl.get_shader_info_log(&frag_shader);
    log!("grid.frag compilation log: ", frag_shader_log);

    let shader_program = Arc::new(gl.create_program().unwrap());
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);

    gl.link_program(&shader_program);

    let vertices = vec![
        0.0, 0.0, -1.0,
        0.0, 0.0, 1.0,
        -1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
    ];

    let vertex_buffer = Arc::new(gl.create_buffer().unwrap());
    let js_vertices = Arc::new(js_sys::Float32Array::from(vertices.as_slice()));


    let stuff_uniforms_loc = Arc::new(gl.get_uniform_block_index(&shader_program, "Stuff"));
    gl.uniform_block_binding(&shader_program, *stuff_uniforms_loc, 0);
    let stuff_uniform_buffer = Arc::new(gl.create_buffer().unwrap());


    Ok(
        Arc::new(
            DrawStuff {
                shader_program,
                vertex_buffer,
                js_vertices,
                stuff_uniform_buffer,
            }
        )
    )
}

#[derive(Clone)]
pub struct DrawStuff {
    pub shader_program: Arc<web_sys::WebGlProgram>,
    pub vertex_buffer: Arc<WebGlBuffer>,
    pub js_vertices: Arc<js_sys::Float32Array>,
    pub stuff_uniform_buffer: Arc<WebGlBuffer>,
}



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

pub fn draw
(
    gl: Arc<GL>,
    draw_stuff: Arc<DrawStuff>,
    state: Arc<Mutex<state::State>>,
)
{
    gl.use_program(Some(&draw_stuff.shader_program));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&draw_stuff.indexed_vertex_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &draw_stuff.indexed_js_vertices, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(0 as u32, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0 as u32);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&draw_stuff.normals_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &draw_stuff.js_normals, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(1 as u32, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(1 as u32);

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&draw_stuff.index_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &draw_stuff.js_indices, GL::STATIC_DRAW);




    // Transforms to apply, moving model space to screen space:
    // 1. Scale down.
    // 2. Model transform to world space, rotate, then translate.
    // 3. view translation with camera look at.
    // 4. projection matrix?


    let scale_down = Matrix4::from_scale(state.lock().unwrap().model_scale);


    let x_rot = Matrix4::from_angle_x(state.lock().unwrap().model_rot[0]);
    let y_rot = Matrix4::from_angle_y(state.lock().unwrap().model_rot[1]);
    let z_rot = Matrix4::from_angle_z(state.lock().unwrap().model_rot[2]);




    let all_rot = x_rot * y_rot * z_rot * scale_down;
    

    let mtx = state.lock().unwrap().model_trans[0];
    let mty = state.lock().unwrap().model_trans[1];
    let mtz = state.lock().unwrap().model_trans[2];

    let translate = cgmath::Matrix4::from_translation(cgmath::Vector3::new(mtx, mty, mtz));
    

    let m350 = translate * all_rot;

    // This is the model matrix ^ : scaled, rotated, and translated from model to world space.


    // Treat the camera as a vehicle
    // It will be controlled by 3-axis for rotations and a thrust axis with positive and negative inputs.


    // Treat the camera as a vehicle, or as if a vehicle was 
    // there and vehicle has same attributes for location
    // in world_space model mat, 
    // one vector of rotation in three axes
    // one vector of displacement in three axes
    // on scale on the model space to get correct dimensions in world
    // space.

    // let eye = Point3::new(-0.9, 0.0, 0.0);

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

    let mut arr: [f32; 300] = [0.0; 300];
    let mut kdx: usize = 0;
    for idx in 0..4 {
        for jdx in 0..4 {
            arr[kdx] = m350[idx][jdx];
            // log!("view_mat: ", view_mat[idx][jdx]);
            kdx = kdx + 1;
        }
    }
    for idx in 0..4 {
        for jdx in 0..4 {
            arr[kdx] = view_matrix[idx][jdx];
            kdx = kdx + 1;
        }
    }



    // The next really lab thing to do is add in event handlers for other things
    // like camera position and 

    // for idx in 0..4 {
    //     for jdx in 0..4 {
    //         arr[kdx] = proj_mat[idx][jdx];
    //         kdx = kdx + 1;
    //     }
    // }

    gl.bind_buffer_base(GL::UNIFORM_BUFFER, 0, Some(&draw_stuff.stuff_uniform_buffer.as_ref()));
    let arr_js = js_sys::Float32Array::from(arr.as_slice());
    gl.buffer_data_with_array_buffer_view(GL::UNIFORM_BUFFER, &arr_js, GL::STATIC_DRAW);

    gl.draw_elements_with_i32(GL::TRIANGLES, 21, GL::UNSIGNED_INT, 0);
    gl.bind_buffer(GL::ARRAY_BUFFER, None);
}

pub fn prepare_draw
(
    gl: Arc<GL>
)
-> Result<Arc<DrawStuff>, String>
{

    let indexed_vertices = vec![
        0.021, 0.0, 0.0, // 0
        0.0, 0.021, 0.0, // 1
        0.0, -0.021, 0.0, // 2
        0.0, 0.0, 0.0, // 3
        -0.031, 0.031, 0.0, // 4
        -0.031, -0.031, 0.0, // 5
        0.0, 0.0, 0.011, // 6
    ];

    // primitives are triangles one each per row on the indices.
    // cross product of any two of those vectors see index into indexed vertices
    // will yield normalizable.

    let normal_0 = cgmath::Vector3::new(indexed_vertices[0], indexed_vertices[1], indexed_vertices[2]).cross(cgmath::Vector3::new(indexed_vertices[3], indexed_vertices[4], indexed_vertices[5])).normalize();

    // Choose 4 and 1.
    let normal_1 = cgmath::Vector3::new(indexed_vertices[12], indexed_vertices[13], indexed_vertices[14]).cross(cgmath::Vector3::new(indexed_vertices[3], indexed_vertices[4], indexed_vertices[5])).normalize();

    // Choose 2 and 5.
    let normal_2 = cgmath::Vector3::new(indexed_vertices[6], indexed_vertices[7], indexed_vertices[8]).cross(cgmath::Vector3::new(indexed_vertices[15], indexed_vertices[16], indexed_vertices[17])).normalize();

    // Choose 6 and 0
    let normal_3 = cgmath::Vector3::new(indexed_vertices[18], indexed_vertices[19], indexed_vertices[20]).cross(cgmath::Vector3::new(indexed_vertices[0], indexed_vertices[1], indexed_vertices[2])).normalize();
    // Normal_4 is the same as normal_3, notice they share 6 and 0.
    // Choose 6 and 4
    let normal_5 = cgmath::Vector3::new(indexed_vertices[18], indexed_vertices[19], indexed_vertices[20]).cross(cgmath::Vector3::new(indexed_vertices[9], indexed_vertices[10], indexed_vertices[11])).normalize();
    // Choose 6 and 2
    let normal_6 = cgmath::Vector3::new(indexed_vertices[18], indexed_vertices[19], indexed_vertices[20]).cross(cgmath::Vector3::new(indexed_vertices[6], indexed_vertices[7], indexed_vertices[8])).normalize();


    // This would work if the normals were indexed by the same index vector index.  but that would be stupid
    // because then we couldn't have distinct normal vectors for distinct gl vertices that share the same 
    // geometric vector.
    let normals_packed_list = vec![ 
        normal_0[0], normal_0[1], normal_0[2],
        normal_1[0], normal_1[1], normal_1[2],
        normal_2[0], normal_2[1], normal_2[2],
        normal_3[0], normal_3[1], normal_3[2],
        normal_3[0], normal_3[1], normal_3[2],
        normal_5[0], normal_5[1], normal_5[2],
        normal_6[0], normal_6[1], normal_6[2],
    ];

    // Hopefully the normals need to be specified per GL vertex (as opposed to geometric vertex)
    // In other words they aren't specified by the indices of the index buffer/vector,
    // but map one to one per GL vertex.  This means we specify one per GL vertex, 3 of which make a primitive
    // face, triangle here.  So we set them out like this.
    let normals = vec![
        normal_0[0], normal_0[1], normal_0[2],
        normal_0[0], normal_0[1], normal_0[2],
        normal_0[0], normal_0[1], normal_0[2],

        normal_1[0], normal_1[1], normal_1[2],
        normal_1[0], normal_1[1], normal_1[2],
        normal_1[0], normal_1[1], normal_1[2],

        normal_2[0], normal_2[1], normal_2[2],
        normal_2[0], normal_2[1], normal_2[2],
        normal_2[0], normal_2[1], normal_2[2],

        normal_3[0], normal_3[1], normal_3[2],
        normal_3[0], normal_3[1], normal_3[2],
        normal_3[0], normal_3[1], normal_3[2],

        normal_3[0], normal_3[1], normal_3[2],
        normal_3[0], normal_3[1], normal_3[2],
        normal_3[0], normal_3[1], normal_3[2],

        normal_5[0], normal_5[1], normal_5[2],
        normal_5[0], normal_5[1], normal_5[2],
        normal_5[0], normal_5[1], normal_5[2],

        normal_6[0], normal_6[1], normal_6[2],
        normal_6[0], normal_6[1], normal_6[2],
        normal_6[0], normal_6[1], normal_6[2],
    ];

    let indices = vec![
        0, 1, 2, // normal 0
        3, 4, 1, // normal 1
        3, 2, 5, // 2
        6, 0, 1, // 3
        6, 0, 2, // 4
        6, 4, 1, // 5
        6, 2, 5, // 6
    ];


    // Maybe normals are indexed the same as vertices? We have a list of geometric vertices that are indexed by the indices in 
    // the index vector.  Are the normals indexed the same way or what?  Or are they just mapped one to one into the against 
    // the index vectors own intrinsic indices?

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


    // let vertex_buffer = Arc::new(gl.create_buffer().unwrap());
    // let js_vertices = Arc::new(js_sys::Float32Array::from(vertices.as_slice()));


    let vertices_position = Arc::new(gl.get_attrib_location(&shader_program, "a_position") as i32);
    let indexed_vertex_buffer = Arc::new(gl.create_buffer().unwrap());
    let indexed_js_vertices = Arc::new(js_sys::Float32Array::from(indexed_vertices.as_slice()));
    
    let index_buffer = Arc::new(gl.create_buffer().unwrap());
    let js_indices = Arc::new(js_sys::Int32Array::from(indices.as_slice()));
    // let indices_position = Arc::new(gl.get_attrib_location(&shader_program, "a_index") as i32);

    let normals_buffer = Arc::new(gl.create_buffer().unwrap());
    let js_normals = Arc::new(js_sys::Float32Array::from(normals.as_slice()));
    let normals_position = Arc::new(gl.get_attrib_location(&shader_program, "a_normal") as i32);

    let stuff_uniforms_loc = Arc::new(gl.get_uniform_block_index(&shader_program, "Stuff"));
    gl.uniform_block_binding(&shader_program, *stuff_uniforms_loc, 0);
    let stuff_uniform_buffer = Arc::new(gl.create_buffer().unwrap());
    
    Ok(
        Arc::new(
            DrawStuff {
                shader_program,
                // vertices_position,
                stuff_uniform_buffer,
                index_buffer,
                js_indices,
                indexed_vertex_buffer,
                indexed_js_vertices,
                normals_buffer,
                // normals_position,
                js_normals,
            }
        )
    )
}

#[derive(Clone)]
pub struct DrawStuff {
    pub shader_program: Arc<web_sys::WebGlProgram>,
    // pub vertices_position: Arc<i32>,
    pub stuff_uniform_buffer: Arc<WebGlBuffer>,
    pub index_buffer: Arc<WebGlBuffer>,
    pub js_indices: Arc<js_sys::Int32Array>,
    pub indexed_vertex_buffer: Arc<WebGlBuffer>,
    pub indexed_js_vertices: Arc<js_sys::Float32Array>,
    pub normals_buffer: Arc<WebGlBuffer>,
    // pub normals_position: Arc<i32>,  
    pub js_normals: Arc<js_sys::Float32Array>,
}
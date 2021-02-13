use crate::resources::Resources;
use crate::loader::*;
use crate::shader::{Shader, Program};
use std::path::{Path};

pub fn load_assets(resources: &mut Resources) {
    let resource_path = from_relative_exe_path(Path::new("assets")).unwrap();

    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/textured_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/textured_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program("textured_program", obj_program).unwrap();
    }

    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/phong_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/phong_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program("phong_program", obj_program).unwrap();
    }


    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/g_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/g_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program("g_program", obj_program).unwrap();
    }
    
    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/lightpass_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/lightpass_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program("lightpass_program", obj_program).unwrap();
    }
    
    {
        let quad_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/quad_vert.glsl").unwrap()).unwrap();
        let quad_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/quad_frag.glsl").unwrap()).unwrap();
        let quad_program = Program::from_shaders(&quad_vert_shader, &quad_frag_shader).unwrap();
        resources.add_program("quad_program", quad_program).unwrap();
    }

    let m_mesh = load_ply(&resource_path, "meshes/face.ply").unwrap();
    resources.add_mesh("face", m_mesh).unwrap();
    let labyrinth_mesh = load_ply(&resource_path, "meshes/labyrinth.ply").unwrap();
    resources.add_mesh("lab", labyrinth_mesh).unwrap();
    let sphere_mesh = load_ply(&resource_path, "meshes/sphere.ply").unwrap();
    resources.add_mesh("sphere", sphere_mesh).unwrap();


    let mut dumb_texture = load_png(&resource_path, "textures/dumb_texture.png").unwrap();
    dumb_texture.set_parameters(vec![(gl::TEXTURE_MIN_FILTER, gl::LINEAR), (gl::TEXTURE_MAG_FILTER, gl::LINEAR), (gl::TEXTURE_WRAP_S, gl::REPEAT), (gl::TEXTURE_WRAP_T, gl::REPEAT)]);
    resources.add_texture("dumb", dumb_texture).unwrap();
    let mut m_tex = load_png(&resource_path, "textures/face.png").unwrap();
    m_tex.set_parameters(vec![(gl::TEXTURE_MIN_FILTER, gl::LINEAR), (gl::TEXTURE_MAG_FILTER, gl::LINEAR), (gl::TEXTURE_WRAP_S, gl::REPEAT), (gl::TEXTURE_WRAP_T, gl::REPEAT)]);
    resources.add_texture("face", m_tex).unwrap();

}


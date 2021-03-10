use crate::resources::Resources;
use crate::loader::*;
use crate::shader::{Shader, Program};
use std::path::{Path};

pub fn load_assets(resources: &mut Resources) {
    let resource_path = from_relative_exe_path(Path::new("assets")).unwrap();
    {
        let vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/flat_vert.glsl").unwrap()).unwrap();
        let frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/flat_frag.glsl").unwrap()).unwrap();
        let program = Program::from_shaders(&vert_shader, &frag_shader).unwrap();
        resources.add_program_by_name("flat_program", program).unwrap();
    }
    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/textured_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/textured_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program_by_name("textured_program", obj_program).unwrap();
    }

    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/phong_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/phong_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program_by_name("phong_program", obj_program).unwrap();
    }


    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/g_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/g_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program_by_name("g_program", obj_program).unwrap();
    }
    {
        let vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/g_terrain_vert.glsl").unwrap()).unwrap();
        let frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/g_terrain_frag.glsl").unwrap()).unwrap();
        let quad_program = Program::from_shaders(&vert_shader, &frag_shader).unwrap();
        resources.add_program_by_name("g_terrain_program", quad_program).unwrap();
    }
    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/g_tex_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/g_tex_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program_by_name("g_tex_program", obj_program).unwrap();
    }

    {
        let obj_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/lightpass_vert.glsl").unwrap()).unwrap();
        let obj_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/lightpass_frag.glsl").unwrap()).unwrap();
        let obj_program = Program::from_shaders(&obj_vert_shader, &obj_frag_shader).unwrap();
        resources.add_program_by_name("lightpass_program", obj_program).unwrap();
    }
    
    {
        let quad_vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/quad_vert.glsl").unwrap()).unwrap();
        let quad_frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/quad_frag.glsl").unwrap()).unwrap();
        let quad_program = Program::from_shaders(&quad_vert_shader, &quad_frag_shader).unwrap();
        resources.add_program_by_name("quad_program", quad_program).unwrap();
    }
    {
        let vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/tone_map_vert.glsl").unwrap()).unwrap();
        let frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/tone_map_frag.glsl").unwrap()).unwrap();
        let program = Program::from_shaders(&vert_shader, &frag_shader).unwrap();
        resources.add_program_by_name("tone_map_program", program).unwrap();
    }
    {
        let vert_shader = Shader::from_vert_source(&load_cstring(&resource_path, "shaders/hud_vert.glsl").unwrap()).unwrap();
        let frag_shader = Shader::from_frag_source(&load_cstring(&resource_path, "shaders/hud_frag.glsl").unwrap()).unwrap();
        let program = Program::from_shaders(&vert_shader, &frag_shader).unwrap();
        resources.add_program_by_name("hud_program", program).unwrap();
    }
    {
        let labyrinth_mesh = load_ply(&resource_path, "meshes/labyrinth.ply").unwrap();
        resources.add_mesh_by_name("lab", labyrinth_mesh).unwrap();
        let sphere_mesh = load_ply(&resource_path, "meshes/sphere.ply").unwrap();
        resources.add_mesh_by_name("sphere", sphere_mesh).unwrap();
        let cube_mesh = load_ply(&resource_path, "meshes/cube.ply").unwrap();
        resources.add_mesh_by_name("cube", cube_mesh).unwrap();
        let gun_mesh = load_ply(&resource_path, "meshes/gun.ply").unwrap();
        resources.add_mesh_by_name("gun", gun_mesh).unwrap();
    }

    {
        let mut dumb_texture = load_png(&resource_path, "textures/dumb_texture.png").unwrap();
        dumb_texture.set_parameters(vec![(gl::TEXTURE_MIN_FILTER, gl::LINEAR), (gl::TEXTURE_MAG_FILTER, gl::LINEAR), (gl::TEXTURE_WRAP_S, gl::REPEAT), (gl::TEXTURE_WRAP_T, gl::REPEAT)]);
        resources.add_texture_by_name("dumb", dumb_texture).unwrap();
    }


}



/*
holds resources
*/

use std::collections::HashMap;

use crate::mesh::Mesh;
use crate::texture::Texture;
use crate::shader::{Program};

#[derive(Debug)]
pub enum Error {
    ResourceDoesNotExist,
    ResourceNameDoesNotExist(String),
    ResourceNameAlreadyExists(String),
}

pub type ResourceKey = usize;

struct ResourceEntries<T> {
    entries: Vec<T>,
    name_map: HashMap<&'static str, ResourceKey>,
}
impl<T> ResourceEntries<T> {
    pub fn new() -> Self {
        ResourceEntries {
            entries: Vec::new(),
            name_map: HashMap::new(),
        }
    }
    pub fn add_resource(&mut self, r: T) -> ResourceKey {
        self.entries.push(r);
        self.entries.len() - 1
    }
    pub fn add_resource_by_name(&mut self, name: &'static str, res: T) -> Result<ResourceKey, Error> {
        if self.name_map.contains_key(name) {
            return Err(Error::ResourceNameAlreadyExists(name.to_string()));
        }
        let id = self.add_resource(res);
        self.name_map.insert(name, id);
        Ok(id)
    }
    pub fn get_resource(&self, k: ResourceKey) -> Result<&T, Error> {
        self.entries.get(k).ok_or(Error::ResourceDoesNotExist)
    }
    pub fn get_resource_by_name(&self, name: &str) -> Result<&T, Error> {
        if !self.name_map.contains_key(name) {
            return Err(Error::ResourceNameDoesNotExist(name.to_string()));
        }
        let id = *self.name_map.get(name).unwrap();
        self.entries.get(id).ok_or(Error::ResourceDoesNotExist)
    }
    pub fn get_resource_id_by_name(&self, name: &str) -> Result<ResourceKey, Error> {
        if !self.name_map.contains_key(name) {
            return Err(Error::ResourceNameDoesNotExist(name.to_string()));
        }
        Ok(*self.name_map.get(name).unwrap())
    }
}

pub struct Resources {
    meshes: ResourceEntries<Mesh>,
    textures: ResourceEntries<Texture>,
    programs: ResourceEntries<Program>,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            meshes: ResourceEntries::new(),
            textures: ResourceEntries::new(),
            programs: ResourceEntries::new(),
        }
    }
    pub fn add_mesh(&mut self, mesh: Mesh) -> ResourceKey {
        self.meshes.add_resource(mesh)
    }
    pub fn add_mesh_by_name(&mut self, name: &'static str, mesh: Mesh) -> Result<ResourceKey, Error> {
        self.meshes.add_resource_by_name(name, mesh)
    }   
    pub fn get_mesh(&self, id: ResourceKey) -> Result<&Mesh, Error> {
        self.meshes.get_resource(id)
    }
    pub fn get_mesh_by_name(&self, name: &str) -> Result<&Mesh, Error> {
        self.meshes.get_resource_by_name(name)
    }
    pub fn get_mesh_id_by_name(&self, name: &str) -> Result<ResourceKey, Error> {
        self.meshes.get_resource_id_by_name(name)
    }
    pub fn add_texture(&mut self, texture: Texture) -> ResourceKey {
        self.textures.add_resource(texture)
    }
    pub fn add_texture_by_name(&mut self, name: &'static str, texture: Texture) -> Result<ResourceKey, Error> {
        self.textures.add_resource_by_name(name, texture)
    }   
    pub fn get_texture(&self, id: ResourceKey) -> Result<&Texture, Error> {
        self.textures.get_resource(id)
    }
    pub fn get_texture_by_name(&self, name: &str) -> Result<&Texture, Error> {
        self.textures.get_resource_by_name(name)
    }
    pub fn get_texture_id_by_name(&self, name: &str) -> Result<ResourceKey, Error> {
        self.textures.get_resource_id_by_name(name)
    }
    pub fn add_program(&mut self, program: Program) -> ResourceKey {
        self.programs.add_resource(program)
    }
    pub fn add_program_by_name(&mut self, name: &'static str, program: Program) -> Result<ResourceKey, Error> {
        self.programs.add_resource_by_name(name, program)
    }   
    pub fn get_program(&self, id: ResourceKey) -> Result<&Program, Error> {
        self.programs.get_resource(id)
    }
    pub fn get_program_by_name(&self, name: &str) -> Result<&Program, Error> {
        self.programs.get_resource_by_name(name)
    }
    pub fn get_program_id_by_name(&self, name: &str) -> Result<ResourceKey, Error> {
        self.programs.get_resource_id_by_name(name)
    }
}
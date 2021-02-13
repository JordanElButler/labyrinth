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
    ResourceIDAlreadyExists,
}

enum ResourceType {
    Mesh,
    Texture,
    Program,
}

struct ResourceEntry<T> {
    name: &'static str,
    item: T,
}

pub struct Resources {
    mesh_map: HashMap<&'static str, Mesh>,
    texture_map: HashMap<&'static str, Texture>,
    program_map: HashMap<&'static str, Program>,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            mesh_map: HashMap::new(),
            texture_map: HashMap::new(),
            program_map: HashMap::new(),
        }
    }
    pub fn add_mesh(&mut self, id: &'static str, mesh: Mesh) -> Result<(), Error> {
        if self.mesh_map.contains_key(id) {
            return Err(Error::ResourceIDAlreadyExists);
        }
        self.mesh_map.insert(id, mesh);

        Ok(())
    }

    pub fn get_mesh(&mut self, id: &str) -> Result<&mut Mesh, Error> {
        let durr = self.mesh_map.get_mut(id);
        match durr {
            Some(p) => Ok(p),
            None => Err(Error::ResourceDoesNotExist)
        }
    }
    pub fn add_texture(&mut self, id: &'static str, texture: Texture) -> Result<(), Error> {
        if self.program_map.contains_key(id) {
            return Err(Error::ResourceIDAlreadyExists);
        }
        self.texture_map.insert(id, texture);

        Ok(())
    }

    pub fn get_texture(&mut self, id: &str) -> Result<&mut Texture, Error> {
        let durr = self.texture_map.get_mut(id);
        match durr {
            Some(p) => Ok(p),
            None => Err(Error::ResourceDoesNotExist)
        }
    }
    pub fn add_program(&mut self, id: &'static str, program: Program) -> Result<(), Error> {
        if self.program_map.contains_key(id) {
            return Err(Error::ResourceIDAlreadyExists);
        }
        self.program_map.insert(id, program);

        Ok(())
    }

    pub fn get_program(&mut self, id: &str) -> Result<&mut Program, Error> {
        let durr = self.program_map.get_mut(id);
        match durr {
            Some(p) => Ok(p),
            None => Err(Error::ResourceDoesNotExist)
        }
    }
}
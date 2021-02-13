/*
Partially derived from
https://github.com/Nercury/rust-and-opengl-lessons
*/

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;

#[derive(Debug)]
pub enum Error {
    BadFilePath,
    FileContainsNil,
    Io(io::Error),
    BadPlyFile,
    BadPngFile,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct FilePath {
    root_path: PathBuf,
}

pub fn from_relative_exe_path(rel_path: &Path) -> Result<FilePath, Error> {
    let exe_file_name = ::std::env::current_exe()
        .map_err(|_| Error::BadFilePath)?;

    let exe_path = exe_file_name.parent()
                    .ok_or(Error::BadFilePath)?;
    
    Ok(FilePath {
        root_path: exe_path.join(rel_path),
    })
}

pub fn from_exe_path() -> Result<FilePath, Error> {
    from_relative_exe_path(Path::new(""))
}


fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }
    path
}

pub fn load_cstring(file_path: &FilePath, name: &str) -> Result<ffi::CString, Error> {
    let mut file = fs::File::open(
        resource_name_to_path(&file_path.root_path, name)
    )?;
    let mut buffer: Vec<u8> = Vec::with_capacity(
        file.metadata()?.len() as usize + 1
    );

    file.read_to_end(&mut buffer)?;
    
    if buffer.iter().find(|i| **i == 0).is_some() {
        return Err(Error::FileContainsNil);
    }

    Ok(unsafe {ffi::CString::from_vec_unchecked(buffer) })
}

use crate::mesh::{Mesh};
use crate::vertex::{Vertex, VertexLayout, AttributeType};

pub fn load_ply(root_path: &FilePath, name: &str) -> Result<Mesh, Error> {
    let mut file = fs::File::open(
        resource_name_to_path(&root_path.root_path, name )
    )?;

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut vertex_start_line = 0;
    let mut vertex_count = 0;
    let mut face_count = 0;

    for (i, line) in lines.iter().enumerate() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "element" && words[1] == "vertex" {
            vertex_count = words[2].parse().unwrap();
        } else if words[0] == "element" && words[1] == "face" {
            face_count = words[2].parse().unwrap();
        } else if words[0] == "end_header" {
            vertex_start_line = i+1;
            break
        }
    }
    let vertex_layout = VertexLayout::new(vec![AttributeType::Position3D, AttributeType::Normal3D, AttributeType::ST]);
    let mut vertices: Vec<f32> = Vec::with_capacity(vertex_count * vertex_layout.get_width());
    let mut indices: Vec<i32> = Vec::with_capacity(3 * face_count);
    // collect vertices
    for vertex_line in 0..vertex_count {
        let line = lines[vertex_line + vertex_start_line];
        let words: Vec<&str> = line.split_whitespace().collect();
        
        vertices.push(words[0].parse().unwrap());
        vertices.push(words[1].parse().unwrap());
        vertices.push(words[2].parse().unwrap());
        vertices.push(words[3].parse().unwrap());
        vertices.push(words[4].parse().unwrap());
        vertices.push(words[5].parse().unwrap());
        vertices.push(words[6].parse().unwrap());
        vertices.push(words[7].parse().unwrap());
    }

    for index in 0..face_count {
        let line = lines[index + vertex_start_line + vertex_count];
        let words: Vec<&str> = line.split_whitespace().collect();

        indices.push(words[1].parse().unwrap());
        indices.push(words[2].parse().unwrap());
        indices.push(words[3].parse().unwrap());
    }

    Ok(Mesh::new(Vertex::new(vertices, indices, vertex_layout)))
}

use image::codecs::png::{PngDecoder};
use image::ImageDecoder;
use image::ColorType;
use crate::texture::Texture;

pub fn load_png(root_path: &FilePath, name: &str) -> Result<Texture, Error> {
    let file = fs::File::open(
        resource_name_to_path(&root_path.root_path, name)
    )?;

    let decoder = PngDecoder::new(file).map_err(|_| Error::BadPngFile)?;

    let dimensions = decoder.dimensions();
    let color_type_format = decoder.color_type();
    
    let format = match color_type_format {
        ColorType::Rgb16 => (gl::RGB, gl::UNSIGNED_SHORT),
        ColorType::Rgba8 => (gl::RGBA, gl::UNSIGNED_BYTE),
        _ => panic!("No matching opengl type for {:?}", color_type_format),
    };

    let mut bytes: Vec<u8> = vec![0; (dimensions.0 * dimensions.1) as usize * color_type_format.bytes_per_pixel() as usize];

    decoder.read_image(&mut bytes[..]).map_err(|_| Error::BadPngFile)?;

    Ok(Texture::create_with_data(gl::RGBA, format.0, format.1, dimensions, bytes))
}
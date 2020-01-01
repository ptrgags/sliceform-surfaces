use std::fs::File;
use std::io::Write;

use crate::geom::{Vec3, Triangle};

pub struct Face {
    v1: usize,
    v2: usize,
    v3: usize,
    normal: usize,
}

impl Face {
    pub fn new(v1: usize, v2: usize, v3: usize, normal: usize) -> Self {
        Self {
            v1,
            v2,
            v3,
            normal
        }
    }

    pub fn write_line(&self, file: &mut File) {
        writeln!(
            file, 
            "f {0}//{3} {1}//{3} {2}//{3}", 
            self.v1, 
            self.v2, 
            self.v3, 
            self.normal).expect("Failed to write face");
    }
}

pub struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    faces: Vec<Face>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vec3) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);

        index
    }

    pub fn add_normal(&mut self, normal: Vec3) -> usize {
        let index = self.normals.len();
        self.normals.push(normal);

        index
    }

    pub fn add_face(&mut self, face: Face) -> usize {
        let index = self.faces.len();
        self.faces.push(face);

        index
    }
    
    pub fn save_obj_file(&self, fname: &str) {
        let mut file = File::create(fname).expect("Failed to open file");
        
        for Vec3(x, y, z) in self.vertices.iter() {
            writeln!(&mut file, "v {} {} {}", x, y, z)
                .expect("Failed to write vertex");
        }

        for Vec3(x, y, z) in self.normals.iter() {
            writeln!(&mut file, "vn {} {} {}", x, y, z)
                .expect("Failed to write normal");
        }

        for face in self.faces.iter() {
            face.write_line(&mut file);
        }
    }
}

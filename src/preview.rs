use crate::mesh::{Mesh, Face};
use crate::heights::Height2D;
use crate::geom::Vec3;

type VertexSlot = Option<usize>;

pub struct SurfacePreview {
    // Actual container for the geometry
    mesh: Mesh,

    // Number of vertices in each direction (x, y, z)
    // (N, M, P) = (width, length, height)
    width: usize,
    length: usize,
    height: usize,

    // 2D arrays of a vertex index, or None if not yet created
    top: Vec<VertexSlot>, 
    bottom: Vec<VertexSlot>,
    front: Vec<VertexSlot>,
    right: Vec<VertexSlot>,
    back: Vec<VertexSlot>,
    left: Vec<VertexSlot>,

    // The math surface to model
    surf: Box<dyn Height2D>,
}

impl SurfacePreview {
    /// Allocate an empty width x height vector initialized to None
    /// but with a type to hold vertex indices.
    fn allocate_slots(width: usize, height: usize) -> Vec<VertexSlot> {
        (0..(width * height)).map(|_| None).collect()
    }

    pub fn new(m: usize, n: usize, p: usize, surf: Box<dyn Height2D>) -> Self {
        Self {
            mesh: Mesh::new(),
            width: m,
            length: n,
            height: p,
            top: Self::allocate_slots(m, n),
            bottom: Self::allocate_slots(m, n),
            front: Self::allocate_slots(m, p),
            right: Self::allocate_slots(n, p),
            back: Self::allocate_slots(m, p),
            left: Self::allocate_slots(n, p),
            surf,
        }
    }

    pub fn generate_mesh(&mut self) {
        self.populate_top();
        self.populate_bottom();
        self.populate_front();
        self.populate_right();
        self.populate_back();
        self.populate_left();

        self.create_top_faces();
        self.create_bottom_faces();
        self.create_front_faces();
    }

    fn to_coordinate(index: usize, max: usize) -> f64 {
        let numerator = index as f64;
        let denominator = (max as f64) - 1.0;
        numerator / denominator
    }

    fn to_index_1d(i: usize, j: usize, width: usize) -> usize {
        j * width + i
    }

    fn populate_top(&mut self) {
        for i in 0..self.width {
            let x = Self::to_coordinate(i, self.width);
            for j in 0..self.length {
                let y = Self::to_coordinate(j, self.length);
                let z = self.surf.compute_clamped(x, y);

                let vertex = Vec3(x, y, z);
                let vertex_index = self.mesh.add_vertex(vertex);
                let idx = Self::to_index_1d(i, j, self.width);
                self.top[idx] = Some(vertex_index);
            }
        }

        // Update the front face
        for i in 0..self.width { 
            let src_idx = Self::to_index_1d(i, 0, self.width);
            let vert_idx = self.top[src_idx].unwrap();
            let dst_idx = Self::to_index_1d(i, self.height - 1, self.width);
            
            self.front[dst_idx] = Some(vert_idx);
        }

        // Update the back face
        for i in 0..self.width { 
            let src_idx = Self::to_index_1d(self.width - 1 - i, 0, self.width);
            let vert_idx = self.top[src_idx].unwrap();
            let dst_idx = Self::to_index_1d(i, self.height - 1, self.width);
            
            self.back[dst_idx] = Some(vert_idx);
        }
    }
    
    fn populate_bottom(&mut self) {
        let z = 0.0;
        for i in 0..self.width {
            let x = Self::to_coordinate(i, self.width);
            for j in 0..self.length {
                let y = Self::to_coordinate(self.length - 1 - j, self.length);

                let vertex = Vec3(x, y, z);
                let vertex_index = self.mesh.add_vertex(vertex);
                let idx = Self::to_index_1d(i, j, self.width);
                self.bottom[idx] = Some(vertex_index);
            }
        }

        // Update the front face
        for i in 0..self.width { 
            let src_idx = Self::to_index_1d(i, self.length - 1, self.width);
            let vert_idx = self.bottom[src_idx].unwrap();
            let dst_idx = Self::to_index_1d(i, 0, self.width);
            
            self.front[dst_idx] = Some(vert_idx);
        }
    }

    fn populate_front(&mut self) {
        let y = 0.0;
        for i in 0..(self.width - 1) {
            let x = Self::to_coordinate(i, self.width);
            let h = self.surf.compute_clamped(x, y);
            for j in 1..(self.height - 1) {
                let factor = Self::to_coordinate(j, self.height);
                let z = factor * h;

                let vertex = Vec3(x, y, z);
                let vertex_index = self.mesh.add_vertex(vertex);
                let idx = Self::to_index_1d(i, j, self.width);
                self.front[idx] = Some(vertex_index);
            }
        }
    }

    fn populate_right(&mut self) {
        let x = 1.0;
        for i in 0..(self.length - 1) {
            let y = Self::to_coordinate(i, self.width);
            let h = self.surf.compute_clamped(x, y);
            for j in 1..(self.height - 1) {
                let factor = Self::to_coordinate(j, self.height);
                let z = factor * h;

                let vertex = Vec3(x, y, z);
                let vertex_index = self.mesh.add_vertex(vertex);
                let idx = Self::to_index_1d(i, j, self.width);
                self.right[idx] = Some(vertex_index);
            }
        }

        // Populate last row of front face
        for j in 1..(self.height - 1) {
            let src_idx = Self::to_index_1d(0, j, self.length);
            let vert_idx = self.right[src_idx].unwrap();
            let dst_idx = Self::to_index_1d(self.width - 1, j, self.width);
            
            self.front[dst_idx] = Some(vert_idx);
        }
    }
 
    fn populate_back(&mut self) {
        let y = 1.0;
        for i in 0..(self.width - 1) {
            let x = Self::to_coordinate(self.width - 1 - i, self.width);
            let h = self.surf.compute_clamped(x, y);
            for j in 1..(self.height - 1) {
                let factor = Self::to_coordinate(j, self.height);
                let z = factor * h;

                let vertex = Vec3(x, y, z);
                let vertex_index = self.mesh.add_vertex(vertex);
                let idx = Self::to_index_1d(i, j, self.width);
                self.back[idx] = Some(vertex_index);
            }
        }
    }

    fn populate_left(&mut self) {
        let x = 0.0;
        for i in 0..(self.length - 1) {
            let y = Self::to_coordinate(self.length - 1 - i, self.width);
            let h = self.surf.compute_clamped(x, y);
            for j in 1..(self.height - 1) {
                let factor = Self::to_coordinate(j, self.height);
                let z = factor * h;

                let vertex = Vec3(x, y, z);
                let vertex_index = self.mesh.add_vertex(vertex);
                let idx = Self::to_index_1d(i, j, self.width);
                self.left[idx] = Some(vertex_index);
            }
        }
    }

    fn create_top_faces(&mut self) {
        for i in 0..(self.width - 1) {
            for j in 0..(self.length - 1) {
                let idx1 = Self::to_index_1d(i, j, self.width);
                let idx2 = Self::to_index_1d(i + 1, j, self.width);
                let idx3 = Self::to_index_1d(i + 1, j + 1, self.width);
                let idx4 = Self::to_index_1d(i, j + 1, self.width);

                let v1 = self.top[idx1].expect("Missing vertex");
                let v2 = self.top[idx2].expect("Missing vertex");
                let v3 = self.top[idx3].expect("Missing vertex");
                let v4 = self.top[idx4].expect("Missing vertex");

                self.mesh.add_triangle(v1, v2, v3);
                self.mesh.add_triangle(v1, v3, v4);
            }
        }
    }

    fn create_bottom_faces(&mut self) {
        let normal = Vec3(0.0, 0.0, -1.0);
        let normal_idx = self.mesh.add_normal(normal);

        for i in 0..(self.width - 1) {
            for j in 0..(self.length - 1) {
                let idx1 = Self::to_index_1d(i, j, self.width);
                let idx2 = Self::to_index_1d(i + 1, j, self.width);
                let idx3 = Self::to_index_1d(i + 1, j + 1, self.width);
                let idx4 = Self::to_index_1d(i, j + 1, self.width);

                let v1 = self.bottom[idx1].expect("Missing vertex");
                let v2 = self.bottom[idx2].expect("Missing vertex");
                let v3 = self.bottom[idx3].expect("Missing vertex");
                let v4 = self.bottom[idx4].expect("Missing vertex");

                let tri1 = Face::new(v1, v2, v3, normal_idx);
                let tri2 = Face::new(v1, v3, v4, normal_idx); 

                self.mesh.add_face(tri1);
                self.mesh.add_face(tri2);
            }
        }
    }

    fn create_front_faces(&mut self) {
        let normal = Vec3(0.0, -1.0, 0.0);
        let normal_idx = self.mesh.add_normal(normal);

        for i in 0..(self.width - 1) {
            for j in 0..(self.height - 1) {
                let idx1 = Self::to_index_1d(i, j, self.width);
                let idx2 = Self::to_index_1d(i + 1, j, self.width);
                let idx3 = Self::to_index_1d(i + 1, j + 1, self.width);
                let idx4 = Self::to_index_1d(i, j + 1, self.width);

                let v1 = self.front[idx1].expect("Missing vertex");
                let v2 = self.front[idx2].expect("Missing vertex");
                let v3 = self.front[idx3].expect("Missing vertex");
                let v4 = self.front[idx4].expect("Missing vertex");

                let tri1 = Face::new(v1, v2, v3, normal_idx);
                let tri2 = Face::new(v1, v3, v4, normal_idx); 

                self.mesh.add_face(tri1);
                self.mesh.add_face(tri2);
            }
        }
    }

    pub fn save_obj_file(&self, fname: &str) {
        self.mesh.save_obj_file(fname);
    }
}

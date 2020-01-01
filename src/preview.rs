use crate::mesh::Mesh;
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

    pub fn new(M: usize, N: usize, P: usize, surf: Box<dyn Height2D>) -> Self {
        Self {
            mesh: Mesh::new(),
            width: M,
            length: N,
            height: P,
            top: Self::allocate_slots(M, N),
            bottom: Self::allocate_slots(M, N),
            front: Self::allocate_slots(M, P),
            right: Self::allocate_slots(N, P),
            back: Self::allocate_slots(M, P),
            left: Self::allocate_slots(N, P),
            surf,
        }
    }

    pub fn generate_mesh(&mut self) {
        self.populate_top();
        /*
        self.populate_bottom();
        self.populate_front();
        self.populate_right();
        self.populate_back();
        self.populate_left();

        self.create_top_faces();
        self.create_faces(&mut self.bottom, Vec3(0.0, 0.0, -1.0));
        self.create_faces(&mut self.front, Vec3(0.0, -1.0, 0.0));
        self.create_faces(&mut self.right, Vec3(1.0, 0.0, 0.0));
        self.create_faces(&mut self.back, Vec3(0.0, 1.0, 0.0));
        self.create_faces(&mut self.left, Vec3(-1.0, 0.0, 0.0));
        */
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
    }
    
    fn populate_bottom(&mut self) {
    }

    fn populate_front(&mut self) {
    }

    fn populate_right(&mut self) {
    }
 
    fn populate_back(&mut self) {
    }

    fn populate_left(&mut self) {
    }

    fn create_top_faces(&mut self) {
    }

    fn create_faces(&mut self, side: &mut Vec<VertexSlot>, normal: Vec3) {
    }

    pub fn save_obj_file(&self, fname: &str) {
        self.mesh.save_obj_file(fname);
    }
}

use rgx::{
    color::Rgba8,
    core::{IndexBuffer, Pass, VertexBuffer},
    math::Vector3,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Rgba8,
}

pub struct Shape {
    pub index_count: u32,
    pub vertices: VertexBuffer,
    pub indices: IndexBuffer,
}

impl Shape {
    pub fn draw(&self, pass: &mut Pass) {
        pass.set_vertex_buffer(&self.vertices);
        pass.set_index_buffer(&self.indices);
        pass.draw_indexed(0..self.index_count, 0..1)
    }
}

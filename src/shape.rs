use rgx::{
    color::Rgba8,
    core::{IndexBuffer, Pass, VertexBuffer},
    math::Vector3,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    pub position: Vector3<f32>,
    pub color: Rgba8,
}

/// Shape is a loaded, prepared ShapeBuilder that is ready to be drawn
pub struct Shape {
    /// Number of indices contained in `indices`
    pub index_count: u32,
    /// The vertices stored in a vertex buffer
    pub vertices: VertexBuffer,
    /// An index buffer representing a TriangleList of indices within `vertices`
    pub indices: IndexBuffer,
}

impl Shape {
    /// Draws the shape to the Pass.
    ///
    /// You should use `Pass::set_pipeline` before calling this method.
    ///
    /// # Arguments
    ///
    /// * `pass`- The render pass to draw to.
    pub fn draw(&self, pass: &mut Pass) {
        pass.set_vertex_buffer(&self.vertices);
        pass.set_index_buffer(&self.indices);
        pass.draw_indexed(0..self.index_count, 0..1)
    }
}

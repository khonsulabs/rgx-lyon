use crate::shape::{Shape, Vertex};
use rgx::{color::Rgba, core::Renderer, kit::ZDepth, math::Vector3};

mod lyon_builders;

#[derive(Default, Debug)]
/// Builds a shape using lyon for tesselation
pub struct ShapeBuilder {
    zdepth: ZDepth,
    vertices: Vec<Vertex>,
    indicies: Vec<u16>,

    /// This RGBA color is used when tesselating a path with no color data (Attributes in lyon terminology)
    pub default_color: [f32; 4],
}

impl ShapeBuilder {
    /// Create a new ShapeBuilder with a given ZDepth
    ///
    /// # Arguments
    ///
    /// * `zdepth` - The z depth for shapes in this builder to have
    pub fn new(zdepth: ZDepth, default_color: [f32; 4]) -> Self {
        Self {
            zdepth,
            default_color,
            ..Default::default()
        }
    }

    /// Prepare and load this builder into the renderer.
    ///
    /// This does not consume the builder, because wgpu copies the buffer rather than taking ownerhip.
    pub fn prepare(&self, renderer: &Renderer) -> Shape {
        let verticies = renderer.device.create_buffer(&self.vertices);
        let indicies = renderer.device.create_index(&self.indicies);

        Shape {
            index_count: self.indicies.len() as u32,
            vertices: verticies,
            indices: indicies,
        }
    }

    /// Fill an arbitrary path from `lyon::path`
    pub fn fill(
        &mut self,
        path: &lyon::path::Path,
        options: &lyon::tessellation::FillOptions,
    ) -> Result<(), lyon::tessellation::TessellationError> {
        let mut tesselator = lyon::tessellation::FillTessellator::new();
        let _ = tesselator.tessellate_with_ids(path.id_iter(), path, Some(path), options, self)?;
        Ok(())
    }

    /// Stroke an arbitrary path from `lyon::path`
    pub fn stroke(
        &mut self,
        path: &lyon::path::Path,
        options: &lyon::tessellation::StrokeOptions,
    ) -> Result<(), lyon::tessellation::TessellationError> {
        let mut tesselator = lyon::tessellation::StrokeTessellator::new();
        let _ = tesselator.tessellate_with_ids(path.id_iter(), path, Some(path), options, self)?;
        Ok(())
    }

    /// Fill a circle using `lyon::tesselation::basic_shapes`
    pub fn fill_circle(
        &mut self,
        center: lyon::math::Point,
        radius: f32,
        options: &lyon::tessellation::FillOptions,
    ) -> Result<(), lyon::tessellation::TessellationError> {
        lyon::tessellation::basic_shapes::fill_circle(center, radius, options, self)?;
        Ok(())
    }

    /// Fill a circle using `lyon::tesselation::basic_shapes`
    pub fn stroke_circle(
        &mut self,
        center: lyon::math::Point,
        radius: f32,
        options: &lyon::tessellation::StrokeOptions,
    ) -> Result<(), lyon::tessellation::TessellationError> {
        lyon::tessellation::basic_shapes::stroke_circle(center, radius, options, self)?;
        Ok(())
    }

    fn new_vertex(&mut self, point: lyon::math::Point, attributes: &[f32]) -> Vertex {
        let attributes = if attributes.is_empty() {
            &self.default_color
        } else {
            attributes
        };

        assert!(attributes.len() == 4, "Attributes should be RGBA");

        Vertex {
            color: Rgba {
                r: attributes[0],
                g: attributes[1],
                b: attributes[2],
                a: attributes[3],
            }
            .into(),
            position: Vector3::new(point.x, point.y, self.zdepth.0),
        }
    }

    fn add_vertex(
        &mut self,
        point: lyon::math::Point,
        attributes: &[f32],
    ) -> Result<lyon::lyon_tessellation::VertexId, lyon::lyon_tessellation::GeometryBuilderError>
    {
        let vertex = self.new_vertex(point, attributes);
        let new_id = lyon::tessellation::VertexId(self.vertices.len() as u32);
        self.vertices.push(vertex);
        if self.vertices.len() > u16::MAX as usize {
            return Err(lyon::tessellation::GeometryBuilderError::TooManyVertices);
        }

        Ok(new_id)
    }
}

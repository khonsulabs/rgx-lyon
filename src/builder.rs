use crate::{Shape, Vertex};
use rgx::{color::Rgba, core::Renderer, math::Vector3};

#[derive(Default, Debug)]
pub struct ShapeBuilder {
    vertices: Vec<Vertex>,
    indicies: Vec<u16>,
    pub(crate) basic_color: Option<[f32; 4]>,
}

impl ShapeBuilder {
    pub fn prepare(&self, renderer: &Renderer) -> Shape {
        let verticies = renderer.device.create_buffer(&self.vertices);
        let indicies = renderer.device.create_index(&self.indicies);

        Shape {
            index_count: self.indicies.len() as u32,
            vertices: verticies,
            indices: indicies,
        }
    }

    pub fn fill(
        &mut self,
        path: impl IntoIterator<Item = lyon::path::PathEvent>,
        options: &lyon::tessellation::FillOptions,
    ) -> Result<(), lyon::tessellation::TessellationError> {
        let mut tesselator = lyon::tessellation::FillTessellator::new();
        let _ = tesselator.tessellate(path, options, self)?;
        Ok(())
    }

    pub fn fill_circle(
        &mut self,
        center: lyon::math::Point,
        radius: f32,
        color: &[f32; 4],
        fill_options: &lyon::tessellation::FillOptions,
    ) -> Result<(), lyon::tessellation::TessellationError> {
        self.basic_color = Some(*color);
        lyon::tessellation::basic_shapes::fill_circle(center, radius, fill_options, self)?;
        Ok(())
    }

    fn new_vertex(&mut self, point: lyon::math::Point, attributes: &[f32]) -> Vertex {
        Vertex {
            color: Rgba {
                r: attributes[0],
                g: attributes[1],
                b: attributes[2],
                a: attributes[3],
            }
            .into(),
            position: Vector3::new(point.x, point.y, 0.0),
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

impl lyon::tessellation::FillVertexConstructor<Vertex> for ShapeBuilder {
    fn new_vertex(
        &mut self,
        point: lyon::math::Point,
        mut attributes: lyon::lyon_tessellation::FillAttributes,
    ) -> Vertex {
        let attributes = attributes.interpolated_attributes();
        self.new_vertex(point, attributes)
    }
}

impl lyon::tessellation::StrokeVertexConstructor<Vertex> for ShapeBuilder {
    fn new_vertex(
        &mut self,
        point: lyon::math::Point,
        mut attributes: lyon::lyon_tessellation::StrokeAttributes,
    ) -> Vertex {
        let attributes = attributes.interpolated_attributes();
        self.new_vertex(point, attributes)
    }
}

impl lyon::tessellation::FillGeometryBuilder for ShapeBuilder {
    fn add_fill_vertex(
        &mut self,
        position: lyon::math::Point,
        mut attributes: lyon::lyon_tessellation::FillAttributes,
    ) -> Result<lyon::lyon_tessellation::VertexId, lyon::lyon_tessellation::GeometryBuilderError>
    {
        let attributes = attributes.interpolated_attributes();
        self.add_vertex(position, attributes)
    }
}

impl lyon::tessellation::GeometryBuilder for ShapeBuilder {
    fn begin_geometry(&mut self) {}

    fn end_geometry(&mut self) -> lyon::lyon_tessellation::Count {
        lyon::lyon_tessellation::Count {
            vertices: self.vertices.len() as u32,
            indices: self.indicies.len() as u32,
        }
    }

    fn add_triangle(
        &mut self,
        a: lyon::lyon_tessellation::VertexId,
        b: lyon::lyon_tessellation::VertexId,
        c: lyon::lyon_tessellation::VertexId,
    ) {
        self.indicies.push(a.0 as u16);
        self.indicies.push(b.0 as u16);
        self.indicies.push(c.0 as u16);
    }

    fn abort_geometry(&mut self) {
        self.vertices.clear();
        self.indicies.clear();
    }
}

impl lyon::tessellation::BasicGeometryBuilder for ShapeBuilder {
    fn add_vertex(
        &mut self,
        position: lyon::math::Point,
    ) -> Result<lyon::lyon_tessellation::VertexId, lyon::lyon_tessellation::GeometryBuilderError>
    {
        let basic_colors = self.basic_color.unwrap();
        self.add_vertex(position, &basic_colors)
    }
}

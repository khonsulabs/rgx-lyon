use crate::{builder::ShapeBuilder, shape::Vertex};
use lyon::tessellation::{
    BasicGeometryBuilder, FillGeometryBuilder, FillVertexConstructor, GeometryBuilder,
    GeometryBuilderError, StrokeAttributes, StrokeGeometryBuilder, StrokeVertexConstructor,
    VertexId,
};

impl FillVertexConstructor<Vertex> for ShapeBuilder {
    fn new_vertex(
        &mut self,
        point: lyon::math::Point,
        mut attributes: lyon::lyon_tessellation::FillAttributes,
    ) -> Vertex {
        let attributes = attributes.interpolated_attributes();
        self.new_vertex(point, attributes)
    }
}

impl StrokeVertexConstructor<Vertex> for ShapeBuilder {
    fn new_vertex(
        &mut self,
        point: lyon::math::Point,
        mut attributes: lyon::lyon_tessellation::StrokeAttributes,
    ) -> Vertex {
        let attributes = attributes.interpolated_attributes();
        self.new_vertex(point, attributes)
    }
}

impl FillGeometryBuilder for ShapeBuilder {
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

impl StrokeGeometryBuilder for ShapeBuilder {
    fn add_stroke_vertex(
        &mut self,
        position: lyon::math::Point,
        mut attributes: StrokeAttributes,
    ) -> Result<VertexId, GeometryBuilderError> {
        let attributes = attributes.interpolated_attributes();
        self.add_vertex(position, attributes)
    }
}

impl GeometryBuilder for ShapeBuilder {
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

impl BasicGeometryBuilder for ShapeBuilder {
    fn add_vertex(
        &mut self,
        position: lyon::math::Point,
    ) -> Result<lyon::lyon_tessellation::VertexId, lyon::lyon_tessellation::GeometryBuilderError>
    {
        let color = self.default_color;
        self.add_vertex(position, &color)
    }
}

use lyon::{
    math::point,
    path::Path,
    tessellation::{FillOptions, StrokeOptions},
};
use rgx::core::{Blending, Pass, Renderer};
use rgx_lyon::{LyonPipeline, Shape, ShapeBuilder};

mod sandbox;
use sandbox::Sandbox;

fn main() -> Result<(), std::io::Error> {
    PathExample::run()
}

struct PathExample {
    pipeline: LyonPipeline,
    shape: Shape,
}

impl Sandbox for PathExample {
    fn create(renderer: &Renderer) -> Self {
        let pipeline = renderer.pipeline(Blending::default());

        let mut builder = ShapeBuilder::default();

        // RGBA colors specified for each vertex
        let mut path_builder = Path::builder_with_attributes(4);
        path_builder.move_to(point(50., 50.), &[1., 0., 0., 1.]);
        path_builder.line_to(point(100., 150.), &[0., 1., 0., 1.]);
        path_builder.line_to(point(150., 50.), &[0., 0., 1., 1.]);
        path_builder.close();
        let path = path_builder.build();
        builder
            .fill(&path, &FillOptions::default())
            .expect("Error tesselating path");

        // White outline
        builder.default_color = [1., 1., 1., 1.];
        let mut path_builder = Path::builder();
        path_builder.move_to(point(50., 50.));
        path_builder.line_to(point(100., 150.));
        path_builder.line_to(point(150., 50.));
        path_builder.close();
        let path = path_builder.build();
        builder
            .stroke(&path, &StrokeOptions::default())
            .expect("Error tesselating path");

        let shape = builder.prepare(&renderer);

        Self { pipeline, shape }
    }

    fn pipeline(&self) -> &'_ LyonPipeline {
        &self.pipeline
    }

    fn render(&self, pass: &mut Pass) {
        pass.set_pipeline(&self.pipeline);
        self.shape.draw(pass);
    }
}

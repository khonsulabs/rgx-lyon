use lyon::tessellation::{basic_shapes::fill_circle, FillOptions};
use rgx::core::{Blending, Pass, Renderer};
use rgx_lyon::{LyonPipeline, Shape, ShapeBuilder};

mod sandbox;
use sandbox::Sandbox;

fn main() -> Result<(), std::io::Error> {
    CircleExample::run()
}

struct CircleExample {
    pipeline: LyonPipeline,
    shape: Shape,
}

impl Sandbox for CircleExample {
    fn create(renderer: &Renderer) -> Self {
        let pipeline = renderer.pipeline(Blending::default());

        let mut builder = ShapeBuilder::default();
        builder.default_color = [1., 0., 0., 1.];

        fill_circle(
            lyon::math::Point::new(50., 50.),
            25.,
            &FillOptions::default(),
            &mut builder,
        )
        .expect("Error tesselating circle");
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

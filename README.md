# rgx-lyon

This crate provides extremely feature-rich vector shape rendering to [rgx](https://github.com/cloudhead/rgx) via [lyon](https://github.com/nical/lyon).

## A basic example

```rust
let mut builder = ShapeBuilder::default();
lyon::tessellation::basic_shapes::fill_circle(
    Point::new(50., 50.),
    25.,
    &FillOptions::default(),
    &mut builder,
)
.expect("Error tesselating circle");
let shape = builder.prepare(&renderer);

// { ... }

pass.set_pipeline(&self.pipeline);
self.shape.draw(pass);
```

For a more in-depth look, check out the [examples/](./examples/) directory. You can run individual examples using `cargo run --example <name>`, e.g., `cargo run --example circle`

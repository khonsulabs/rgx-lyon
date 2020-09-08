# rgx-lyon

This crate provides extremely feature-rich vector shape rendering to [rgx](https://github.com/cloudhead/rgx) via [lyon](https://github.com/nical/lyon).

## A basic example

```rust
let mut builder = ShapeBuilder::default();
builder
    .fill_circle(
        lyon::math::Point::new(50., 50.),
        25.,
        &[1., 0., 0., 1.],
        &FillOptions::default(),
    )
    .expect("Error tesselating circle");
let shape = builder.prepare(&renderer);

// { ... }

pass.set_pipeline(&self.pipeline);
self.shape.draw(pass);
```

For a more in-depth look, check out the [examples/](./examples/) directory. You can run individual examples using `cargo run --example <name>`, e.g., `cargo run --example circle`

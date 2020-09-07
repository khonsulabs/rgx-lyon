use rgx::{core, math::Matrix4};

pub struct LyonPipeline {
    pipeline: core::Pipeline,
    bindings: core::BindingGroup,
    buf: core::UniformBuffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms {
    pub ortho: Matrix4<f32>,
    pub transform: Matrix4<f32>,
}

impl<'a> core::AbstractPipeline<'a> for LyonPipeline {
    type PrepareContext = Matrix4<f32>;
    type Uniforms = self::Uniforms;

    fn description() -> core::PipelineDescription<'a> {
        core::PipelineDescription {
            vertex_layout: &[
                core::VertexFormat::Float3,
                core::VertexFormat::Float2,
                core::VertexFormat::UByte4,
                core::VertexFormat::Float,
            ],
            pipeline_layout: &[
                core::Set(&[core::Binding {
                    binding: core::BindingType::UniformBuffer,
                    stage: core::ShaderStage::Vertex,
                }]),
                core::Set(&[
                    core::Binding {
                        binding: core::BindingType::SampledTexture,
                        stage: core::ShaderStage::Fragment,
                    },
                    core::Binding {
                        binding: core::BindingType::Sampler,
                        stage: core::ShaderStage::Fragment,
                    },
                ]),
            ],
            vertex_shader: include_bytes!("shaders/shape.vert.spv"),
            fragment_shader: include_bytes!("shaders/shape.frag.spv"),
        }
    }

    fn setup(pipeline: core::Pipeline, dev: &core::Device) -> Self {
        let transform = Matrix4::identity();
        let ortho = Matrix4::identity();
        let buf = dev.create_uniform_buffer(&[self::Uniforms { ortho, transform }]);
        let bindings = dev.create_binding_group(&pipeline.layout.sets[0], &[&buf]);

        Self {
            pipeline,
            buf,
            bindings,
        }
    }

    fn apply(&self, pass: &mut core::Pass) {
        pass.set_pipeline(&self.pipeline);
        pass.set_binding(&self.bindings, &[]);
    }

    fn prepare(
        &'a self,
        ortho: Matrix4<f32>,
    ) -> Option<(&'a core::UniformBuffer, Vec<self::Uniforms>)> {
        let transform = Matrix4::identity();
        Some((&self.buf, vec![self::Uniforms { transform, ortho }]))
    }
}

// impl core::Renderable for super::Batch {
//     fn buffer(&self, r: &core::Renderer) -> core::VertexBuffer {
//         let buf = self.vertices();
//         r.device.create_buffer(buf.as_slice())
//     }
// }

use super::*;
// - DescriptorContext にどんどんconsしていって、適用する。
//   - Stackになるので、動作がわかりやすいはず
//   - RenderPass・Pipelineも持つ
// TODO: ShaderBind時にデフォルトテクスチャを貼る

pub struct Descriptor {
  vao: Option<Arc<dyn VaoTrait>>,
  u_buffers: Vec<Arc<dyn UniformBufferTrait>>,
  u_mappings: Vec<Arc<dyn TextureMappingTrait>>,
}
impl Descriptor {
  pub fn new() -> Descriptor {
    Self {
      vao: None,
      u_buffers: Vec::new(),
      u_mappings: Vec::new(),
    }
  }
  pub fn set_vao(&mut self, vao: &Arc<dyn VaoTrait>) {
    self.vao = Some(Arc::clone(vao));
  }
  pub fn add_uniform_buffer(&mut self, buffer: &Arc<dyn UniformBufferTrait>) {
    self.u_buffers.push(Arc::clone(buffer));
  }
  pub fn add_texture_mapping(&mut self, mapping: &Arc<dyn TextureMappingTrait>) {
    self.u_mappings.push(Arc::clone(mapping));
  }
}
pub enum DescriptorContext<'a, 'b> {
  Cons {
    prior: &'a Descriptor,
    others: &'b DescriptorContext<'a, 'b>,
  },
  Nil,
}

impl<'a, 'b> DescriptorContext<'a, 'b> {
  pub fn cons(&'a self, prior: &'b Descriptor) -> DescriptorContext<'a, 'b> {
    Self::Cons {
      prior,
      others: self,
    }
  }
  pub fn bind(&self, cmd: &mut Command) {
    if let Self::Cons { prior, others } = self {
      others.bind(cmd);
      for u_buffer in &prior.u_buffers {
        u_buffer.bind(cmd);
      }
      for u_mapping in &prior.u_mappings {
        u_mapping.bind(cmd);
      }
      if let Some(vao) = &prior.vao {
        vao.bind(cmd);
      }
    }
  }
}

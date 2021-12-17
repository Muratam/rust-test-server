use super::*;

// 一度バインド情報を覚えたら二回目以降は問い合わせないためのオブジェクト
struct DescriptorPerShaderProgram {
  raw_vao: RawVao, // for vertex, index
}

// xN のバッファを持つ？
pub struct IndexBuffer {
  gl: Rc<GlContext>,
  raw_buffer: RawGpuBuffer,
}
pub struct VertexBuffer {
  raw_buffer: RawGpuBuffer,
}
pub struct UniformBuffer {
  raw_buffer: RawGpuBuffer,
}
pub struct Vao {
  shader_id_to_raw_vao: std::collections::HashMap<u64, RawVao>,
}
impl IndexBuffer {
  // pub fn new(gl: Rc<GlContext>, data: &Vec<IndexBufferType>) -> Self {
  //   let raw_buffer = RawGpuBuffer::new(&gl, data.as_slice(), BufferUsage::Index);
  //   Self {
  //     gl: Rc::clone(&gl),
  //     raw_buffer,
  //   }
  // }
}
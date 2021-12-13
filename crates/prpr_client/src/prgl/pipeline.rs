use super::*;

enum DrawCommand {
  Draw { first: i32, count: i32 },
  // DrawInstanced {
  //   first: i32,
  //   count: i32,
  //   instance_count: i32,
  // },
  DrawIndexed { first: i32, count: i32 },
  // DrawIndexedInstanced {
  //   first: i32,
  //   count: i32,
  //   instance_count: i32,
  // },
  // Buffers([buf])
  // RangeElements { u32, u32, u32, u32 }  // start, end, count, (type), offset
}

#[derive(Clone, Copy)]
pub enum PrimitiveToporogy {
  Points = gl::POINTS as isize,
  LineStrip = gl::LINE_STRIP as isize,
  LineLoop = gl::LINE_LOOP as isize,
  Lines = gl::LINES as isize,
  TriangleStrip = gl::TRIANGLE_STRIP as isize,
  TriangleFan = gl::TRIANGLE_FAN as isize,
  Triangles = gl::TRIANGLES as isize,
}

pub struct Pipeline {
  gl: Rc<GlContext>,
  draw_command: Option<DrawCommand>,
  primitive_topology: PrimitiveToporogy,
  raw_shader_program: Option<RawShaderProgram>,
  raw_vao: Option<RawVao>,
  // BlendState
  // ColorState
  // CullState
  // DepthState
  // CoverageState
  // PolygonOffsetState
  // StencilState
  // Scissor
  // Viewport
}
impl Pipeline {
  pub fn new(gl: Rc<GlContext>) -> Self {
    Self {
      gl: Rc::clone(&gl),
      draw_command: None,
      primitive_topology: PrimitiveToporogy::Triangles,
      raw_shader_program: None,
      raw_vao: None,
    }
  }
  pub fn setup_sample(&mut self) {
    let gl = self.gl.as_ref();
    #[repr(C)]
    struct VertexType {
      position: Vec3Attr,
      color: Vec4Attr,
    }
    let v_attrs = vec![
      RawVertexAttribute {
        name: String::from("position"),
        location: 0, // from shader ?
        primitive_type: Vec3Attr::primitive_type(),
        count: Vec3Attr::count(),
      },
      RawVertexAttribute {
        name: String::from("color"),
        location: 1, // from shader ?
        primitive_type: Vec4Attr::primitive_type(),
        count: Vec4Attr::count(),
      },
    ];
    let common_header = "#version 300 es\nprecision highp float;";
    let layouts = v_attrs
      .iter()
      .map(|x| x.to_layout_location_str())
      .collect::<String>();
    let vs_code = "
      out vec4 in_color;
      void main() {
        in_color = color;
        gl_Position = vec4(position, 1.0);
      }
    ";
    let fs_code = "
      in vec4 in_color;
      out vec4 out_color;
      layout (std140) uniform Global { vec4 add_color; };
      void main() { out_color = in_color + add_color; }
    ";
    let vs_code = format!("{}\n{}\n{}", common_header, layouts, vs_code);
    let fs_code = format!("{}\n{}", common_header, fs_code);
    // shader
    let vertex_shader = RawShader::new(gl, &vs_code, ShaderType::VertexShader);
    let fragment_shader = RawShader::new(gl, &fs_code, ShaderType::FragmentShader);
    self.raw_shader_program = RawShaderProgram::new(
      gl,
      &RawShaderProgramContents {
        vertex_shader,
        fragment_shader,
      },
    );
    // buffer
    let v_data = vec![
      VertexType {
        position: Vec3Attr::new(Vec3::Y),
        color: Vec4Attr::new(Vec4::X + Vec4::W),
      },
      VertexType {
        position: Vec3Attr::new(Vec3::X),
        color: Vec4Attr::new(Vec4::Y + Vec4::W),
      },
      VertexType {
        position: Vec3Attr::new(-Vec3::X),
        color: Vec4Attr::new(Vec4::Z + Vec4::W),
      },
      VertexType {
        position: Vec3Attr::new(-Vec3::Y),
        color: Vec4Attr::new(Vec4::ONE),
      },
    ];
    let v_buffer = RawGpuBuffer::new(gl, v_data.as_slice(), BufferUsage::Vertex);
    let i_data: Vec<IndexBufferType> = vec![0, 1, 2, 2, 3, 1];
    let i_buffer = RawGpuBuffer::new(gl, i_data.as_slice(), BufferUsage::Index);
    self.raw_vao = Some(RawVao::new(gl, &v_attrs, &v_buffer, Some(&i_buffer)));
    if let Some(program) = &self.raw_shader_program {
      let u_data = vec![Vec4::new(0.5, 0.5, 0.5, 0.5)];
      let u_buffer = RawGpuBuffer::new(gl, u_data.as_slice(), BufferUsage::Uniform);
      let u_name = "Global";
      // let shader_id = &program.raw_program_id();
      let u_index = gl.get_uniform_block_index(&program.raw_program(), u_name);
      if u_index == gl::INVALID_INDEX {
        log::error(format!("invalid uniform buffer name: {}", u_name));
      }
      gl.bind_buffer_base(gl::UNIFORM_BUFFER, u_index, Some(&u_buffer.raw_buffer()));
    }
    self.set_draw_indexed(0, i_data.len() as i32);
  }
  pub fn draw(&self) {
    let gl = &self.gl;
    if let Some(program) = &self.raw_shader_program {
      gl.use_program(Some(program.raw_program()));
    } else {
      log::error("No Shader Program");
      return;
    }
    if let Some(vao) = &self.raw_vao {
      gl.bind_vertex_array(Some(vao.get_raw_vao()));
    } else {
      log::error("No Vertex Array Object");
      return;
    }
    let topology = self.primitive_topology as u32;
    if let Some(command) = &self.draw_command {
      match &command {
        DrawCommand::Draw { first, count } => {
          gl.draw_arrays(topology, *first, *count);
        }
        DrawCommand::DrawIndexed { first, count } => {
          assert_type_eq!(u32, IndexBufferType);
          gl.draw_elements_with_i32(topology, *count, gl::UNSIGNED_INT, *first);
        }
      }
    } else {
      log::error("No Draw Command");
      return;
    }
  }
  pub fn set_draw(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::Draw { first, count });
  }
  pub fn set_draw_indexed(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::DrawIndexed { first, count });
  }
  pub fn set_draw_mode(&mut self, primitive_topology: PrimitiveToporogy) {
    self.primitive_topology = primitive_topology;
  }
}

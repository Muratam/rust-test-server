use super::*;

pub struct TextureRecipe {}

impl TextureRecipe {
  pub fn new_fullscreen(format: PixelFormat) -> ArcOwner<Texture> {
    let max_viewport = system::WholeScreen::max_viewport();
    ArcOwner::new(Texture::new_uninitialized(&Texture2dDescriptor {
      width: max_viewport.width as usize,
      height: max_viewport.height as usize,
      format,
      mipmap: true,
    }))
  }
  pub fn new_fullscreen_depth() -> ArcOwner<Texture> {
    let max_viewport = system::WholeScreen::max_viewport();
    ArcOwner::new(Texture::new_uninitialized(&Texture2dDescriptor {
      width: max_viewport.width as usize,
      height: max_viewport.height as usize,
      format: PixelFormat::Depth24,
      mipmap: false,
    }))
  }
  pub fn new_dummy() -> ArcOwner<Texture> {
    ArcOwner::new(Texture::new_uninitialized(&Texture2dDescriptor {
      width: 1,
      height: 1,
      format: PixelFormat::R8G8B8A8,
      mipmap: true,
    }))
  }
}

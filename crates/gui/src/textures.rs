use std::{borrow::Cow, rc::Rc, sync::Mutex};

use glium::{
	backend::Facade,
	texture::{ClientFormat, RawImage2d},
	Texture2d,
};
use imgui::TextureId;
use imgui_glium_renderer::Renderer;

// pub struct TextureId(usize);

#[derive(Clone)]
pub struct DrawTexture {
	size: (usize, usize),
	inner: Rc<Mutex<DrawTextureInner>>,
	pub texture_id: TextureId,
}

struct DrawTextureInner {
	data: Vec<u8>,
	dirty: bool,
}

impl DrawTexture {
	pub(crate) fn update<F: Facade>(&mut self, renderer: &mut Renderer, gl_ctx: &F) {
		// Uploaded updated screen texture data
		if let Some(text) = renderer.textures().get_mut(self.texture_id) {
			let mut lock = self.inner.lock().unwrap();
			let raw = RawImage2d {
				data: Cow::Borrowed(&lock.data),
				width: self.size.0 as u32,
				height: self.size.1 as u32,
				format: ClientFormat::U8U8U8,
			};
			let texture = Texture2d::new(gl_ctx, raw).unwrap();
			text.texture = Rc::new(texture);
			lock.dirty = false;
		}
	}

	// pub fn id(&self) -> TextureId {
	// 	self.texture_id
	// }

	// pub fn get_size(&self, scale: f32) -> [f32; 2] {
	// 	[(self.size.0 as f32) * scale, (self.size.1 as f32) * scale]
	// }

	pub(crate) fn dirty(&self) -> bool {
		self.inner.lock().unwrap().dirty
	}

	pub fn refresh<F>(&mut self, refresh_fn: F)
	where
		F: Fn(usize, usize) -> [u8; 3],
	{
		let mut lock = self.inner.lock().unwrap();

		lock.dirty = true;
		for x in 0..self.size.0 {
			for y in 0..self.size.1 {
				let x0 = x * 3;
				let y0 = y * 3;
				let pos = y0 * self.size.0;
				lock.data[pos + x0..pos + x0 + 3].copy_from_slice(&refresh_fn(x, y));
			}
		}
	}

	pub(crate) fn new(size: (usize, usize), data: Vec<u8>, texture_id: TextureId) -> DrawTexture {
		let inner = Rc::new(Mutex::new(DrawTextureInner { data, dirty: false }));
		Self {
			size,
			inner,
			texture_id,
		}
	}
}

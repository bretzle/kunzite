use std::{borrow::Cow, collections::HashMap, rc::Rc};

use crate::{prelude::DrawTexture, Options};
use glium::{
	backend::Facade,
	glutin,
	glutin::{event_loop::EventLoop, window::WindowBuilder},
	texture::{ClientFormat, RawImage2d},
	uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerBehavior},
	Display, Texture2d,
};
use imgui::{Context, FontConfig, FontSource, TextureId};
use imgui_glium_renderer::{Renderer, Texture};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub struct System {
	pub(crate) event_loop: EventLoop<()>,
	pub(crate) display: glium::Display,
	pub(crate) imgui: Context,
	pub(crate) platform: WinitPlatform,
	pub(crate) renderer: Renderer,
	pub font_size: f32,
	pub(crate) draw_textures: HashMap<TextureId, DrawTexture>,
}

impl System {
	pub fn create_texture(&mut self, width: usize, height: usize) -> DrawTexture {
		let gl_ctx = self.display.get_context();

		let mut data = Vec::with_capacity(width * height);
		let mut color = true;
		for _ in 0..height {
			for _ in 0..width {
				// Insert RGB values
				let val = if color { 255 } else { 0 };
				color = !color;
				data.push(val);
				data.push(val);
				data.push(val);
			}
			color = !color;
		}

		let raw = RawImage2d {
			data: Cow::Borrowed(&data),
			width: width as u32,
			height: height as u32,
			format: ClientFormat::U8U8U8,
		};

		let gl_texture = Texture2d::new(gl_ctx, raw).unwrap();

		let default_texture = Texture {
			texture: Rc::new(gl_texture),
			sampler: SamplerBehavior {
				magnify_filter: MagnifySamplerFilter::Nearest,
				minify_filter: MinifySamplerFilter::Linear,
				..Default::default()
			},
		};

		let texture_id = self.renderer.textures().insert(default_texture);

		let text = DrawTexture::new((width, height), data, texture_id);
		self.draw_textures.insert(texture_id, text.clone());
		text
	}
}

pub fn init(options: &Options) -> System {
	let event_loop = EventLoop::new();
	let context = glutin::ContextBuilder::new().with_vsync(true);
	let builder = WindowBuilder::new()
		.with_title(options.title.to_owned())
		.with_inner_size(glutin::dpi::LogicalSize::new(
			options.size[0],
			options.size[1],
		));
	let display =
		Display::new(builder, context, &event_loop).expect("Failed to initialize display");

	let mut imgui = Context::create();
	imgui.style_mut().window_rounding = 5.0;
	imgui.set_ini_filename(None);

	let mut platform = WinitPlatform::init(&mut imgui);
	{
		let gl_window = display.gl_window();
		let window = gl_window.window();
		platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
	}

	let hidpi_factor = platform.hidpi_factor();
	let font_size = (13.0 * hidpi_factor) as f32;
	imgui.fonts().add_font(&[FontSource::DefaultFontData {
		config: Some(FontConfig {
			size_pixels: font_size,
			..FontConfig::default()
		}),
	}]);

	imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

	let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

	System {
		event_loop,
		display,
		imgui,
		platform,
		renderer,
		font_size,
		draw_textures: HashMap::new(),
	}
}

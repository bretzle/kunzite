use crate::Options;
use glium::{
	glutin,
	glutin::{event_loop::EventLoop, window::WindowBuilder},
	Display,
};
use imgui::{Context, FontConfig, FontSource};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub struct System {
	pub event_loop: EventLoop<()>,
	pub display: glium::Display,
	pub imgui: Context,
	pub platform: WinitPlatform,
	pub renderer: Renderer,
	pub font_size: f32,
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
	}
}

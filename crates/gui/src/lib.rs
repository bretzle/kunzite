use imgui::Ui;
use imgui_opengl_renderer::Renderer;
use imgui_sdl2::ImguiSdl2;
use sdl2::event::Event;
use std::time::{Duration, Instant};

pub mod prelude {
	pub use imgui::*;
	pub use sdl2::event::Event;
}

pub trait Application {
	type Error;

	fn setup() -> Self;
	fn handle_event(&mut self, event: Event, running: &mut bool) -> Result<(), Self::Error>;
	fn update(&mut self, frame_time: &Duration, running: &mut bool) -> Result<(), Self::Error>;
	fn draw(&mut self, ui: &Ui);
}

pub struct Options {
	title: String,
	size: [u32; 2],
}

impl Options {
	pub fn new<T: Into<String>>(title: T, width: u32, height: u32) -> Self {
		Self {
			title: title.into(),
			size: [width, height],
		}
	}
}

pub fn run<App: Application>(options: Options) -> Result<(), App::Error> {
	let mut app = App::setup();

	let sdl = sdl2::init().unwrap();
	let video = sdl.video().unwrap();

	{
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(3, 0);
	}

	let window = video
		.window(options.title.as_str(), options.size[0], options.size[1])
		.position_centered()
		.opengl()
		.allow_highdpi()
		.build()
		.unwrap();

	let _gl_context = window
		.gl_create_context()
		.expect("Couldn't create GL context");
	gl::load_with(|s| video.gl_get_proc_address(s) as _);

	let mut imgui = imgui::Context::create();

	imgui.style_mut().window_rounding = 5.0;
	imgui.set_ini_filename(None);

	let mut imgui_sdl = ImguiSdl2::new(&mut imgui, &window);
	let renderer = Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

	let mut event_pump = sdl.event_pump().unwrap();

	let mut last_frame = Instant::now();
	let mut running = true;
	let mut last_frame_time = Duration::from_secs(1);

	while running {
		// handle events
		for event in event_pump.poll_iter() {
			imgui_sdl.handle_event(&mut imgui, &event);
			if imgui_sdl.ignore_event(&event) {
				continue;
			}

			app.handle_event(event, &mut running)?;
		}

		// update
		app.update(&last_frame_time, &mut running)?;

		// rendering
		imgui_sdl.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

		let now = Instant::now();
		last_frame_time = now - last_frame;
		let delta_s = last_frame_time.as_secs() as f32
			+ last_frame_time.subsec_nanos() as f32 / 1_000_000_000.0;
		last_frame = now;
		imgui.io_mut().delta_time = delta_s;

		let ui = imgui.frame();

		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		app.draw(&ui);

		imgui_sdl.prepare_render(&ui, &window);
		renderer.render(ui);

		window.gl_swap_window();
	}

	Ok(())
}

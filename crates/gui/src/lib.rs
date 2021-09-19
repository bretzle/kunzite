mod event;
mod system;

use glium::{
	glutin::{event::Event, event_loop::ControlFlow},
	Surface,
};
use imgui::Ui;
use std::time::{Duration, Instant};
use system::{init, System};

pub mod prelude {
	pub use crate::event::Event;
	pub use glium::glutin::event::{ModifiersState, VirtualKeyCode};
	pub use imgui::*;
}

pub trait Application {
	type Error: std::fmt::Debug;

	fn setup() -> Self;
	fn handle_event(
		&mut self,
		event: prelude::Event,
		running: &mut bool,
	) -> Result<(), Self::Error>;
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

pub fn run<App: 'static + Application>(options: Options) -> Result<(), App::Error> {
	let System {
		event_loop,
		display,
		mut imgui,
		mut platform,
		mut renderer,
		..
	} = init(&options);

	let mut app = App::setup();

	let mut last_frame = Instant::now();
	let mut running = true;
	let mut last_frame_time = Duration::from_secs(1);

	event_loop.run(move |event, _, control_flow| {
		app.update(&last_frame_time, &mut running).unwrap();

		match event {
			Event::NewEvents(_) => {
				let now = Instant::now();
				last_frame_time = now - last_frame;
				imgui.io_mut().update_delta_time(last_frame_time);
				last_frame = now;
			}
			Event::MainEventsCleared => {
				let gl_window = display.gl_window();
				platform
					.prepare_frame(imgui.io_mut(), gl_window.window())
					.expect("Failed to prepare frame");
				gl_window.window().request_redraw();
			}
			Event::RedrawRequested(_) => {
				let ui = imgui.frame();

				app.draw(&ui);

				let gl_window = display.gl_window();
				let mut target = display.draw();
				target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
				platform.prepare_render(&ui, gl_window.window());
				let draw_data = ui.render();
				renderer
					.render(&mut target, draw_data)
					.expect("Rendering failed");
				target.finish().expect("Failed to swap buffers");
			}
			other => {
				let gl_window = display.gl_window();
				platform.handle_event(imgui.io_mut(), gl_window.window(), &other);

				if let Event::WindowEvent { event, .. } = other {
					if let Some(event) = event::event(event) {
						app.handle_event(event, &mut running).unwrap();
					}
				}
			}
		}

		if !running {
			*control_flow = ControlFlow::Exit;
		}
	});
}

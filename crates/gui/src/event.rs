use glium::glutin::{
	self,
	event::{ModifiersState, VirtualKeyCode, WindowEvent},
};
use std::path::PathBuf;

pub enum Event {
	Quit,
	DroppedFile(PathBuf),
	Keypress {
		keycode: Option<VirtualKeyCode>,
		keymod: ModifiersState,
		repeat: bool,
	},
}

#[allow(deprecated)]
pub fn event(ev: WindowEvent) -> Option<Event> {
	let ret = match ev {
		WindowEvent::CloseRequested => Event::Quit,
		WindowEvent::DroppedFile(p) => Event::DroppedFile(p),
		WindowEvent::KeyboardInput { input, .. } => Event::Keypress {
			keycode: input.virtual_keycode,
			keymod: input.modifiers,
			repeat: match input.state {
				glutin::event::ElementState::Pressed => true,
				glutin::event::ElementState::Released => false,
			},
		},
		_ => return None,
	};

	Some(ret)
}

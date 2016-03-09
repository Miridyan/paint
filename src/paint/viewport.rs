use gtk;

use glib::Object;
use glib::object::{ Downcast, IsA };
use glib::translate::*;

pub struct Viewport {
	notebook: gtk::Notebook,
	scroll_area: gtk::ScrolledWindow,
	view_area: gtk::Viewport,
	image: gtk::DrawingArea,
}

pub enum State<T: Image> {
	Edit(T),
	New,
}

impl State {
	pub fn is_new(&self) -> bool {
		match self {
			State::New           => true,
			State::Edit(_) | _   => false,
		}
	}
	// pub fn access<T>(&self) -> T {
	// 	where T: Image {
	// 		match self {
	// 			State::Edit(T)   => T,
	// 			_                => unimplemented!(),
	// 		}
	// 	}
	// }
}

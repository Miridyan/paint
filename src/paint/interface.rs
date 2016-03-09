use gtk;
use gtk::prelude::*;

pub struct Window {
	builder: gtk::Builder,
	window: gtk::Window,
}

pub struct MainMenu<'a> {
	popover: gtk::Popover,
	builder: Option<gtk::Builder>,
	parent: &'a gtk::MenuButton,
}

impl Window {
	pub fn new(path: &str, source: &str) -> Window {
		if gtk::init().is_err() {
			panic!("Failed to initialize GTK!");
		}
		let _builder = gtk::Builder::new_from_file(path);
		let _window: gtk::Window = _builder.get_object(source).unwrap();
		Window {
			builder: _builder,
			window: _window,
		}
	}
	pub fn builder(&self) -> &gtk::Builder {
		&self.builder
	}
	pub fn window(&self) -> &gtk::Window {
		&self.window
	}
	pub fn run(&self) {
		self.window.show_all();
		gtk::main();
	}
}

impl<'a> MainMenu<'a> {
	pub fn new(parent: &'a gtk::MenuButton) -> MainMenu {
		let popover = gtk::Popover::new(Some(parent));

		parent.set_direction(gtk::ArrowType::Down);
		parent.set_popover(Some(&popover));
		parent.set_can_focus(true);

		MainMenu {
			popover: popover,
			builder: None,
			parent: parent,
		}
	}
	pub fn reparent(&mut self, new_parent: &'a gtk::MenuButton) {
		self.parent = new_parent;
	}
	// pub fn add_widget<T: gtk::WidgetExt>(widget: T) {}
	// pub fn remove_widget<T: gtk::WidgetExt>(widget: T) -> Option<T> {}
	// pub fn content_from_builder(&mut self, builder_path: &str) {}
	pub fn run(&self) {
		self.popover.show();
	}
}

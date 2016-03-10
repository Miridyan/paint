use gtk;
use gtk::prelude::*;

use term_painter::ToStyle;
use term_painter::Color::*;

pub struct Window {
	builder: gtk::Builder,
	window: gtk::Window,
}

pub struct MainMenu<'a> {
	popover: gtk::Popover,
	parent: &'a gtk::MenuButton,
	builder: Option<gtk::Builder>,
	content: Option<gtk::Container>,
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
		popover.set_transitions_enabled(true);

		parent.set_direction(gtk::ArrowType::Down);
		parent.set_popover(Some(&popover));
		parent.set_can_focus(true);

		MainMenu {
			popover: popover,
			parent: parent,
			builder: None,
			content: None,
		}
	}
	pub fn reparent(&mut self, new_parent: &'a gtk::MenuButton) {
		self.parent = new_parent;
	}
	// pub fn add_widget<T: gtk::WidgetExt>(&mut self, widget: T) {}
	pub fn remove_widget<T: gtk::WidgetExt>(&mut self, widget: T) {
		if !self.content.is_some() {
			println!("{} Cannot remove widget {}. Was not found",
				Red.bold().paint("[ERR]"), widget.get_name().unwrap());
			return;
		}
	}
	pub fn add_content_from_builder(&mut self, builder_path: &str, content_name: &str) {
		let new_builder = gtk::Builder::new_from_file(builder_path);
		let content: gtk::Container = new_builder.get_object(content_name).unwrap();
		self.popover.add(&content);
	}
	pub fn clear_content(&mut self) {
		self.content = None;
	}
	pub fn has_content(&self) -> bool {
		self.content.is_some()
	}
	// pub fn run(&self) {
	// 	self.popover.show();
	// }
}

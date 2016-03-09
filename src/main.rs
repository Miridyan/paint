extern crate gtk;
extern crate paint;

use paint::interface::*;
use gtk::prelude::*;

fn main() {
	let window = Window::new("assets/main_window.glade", "main_window");
	let builder = window.builder();
	let menu_button: gtk::MenuButton = builder.get_object("main_menu_button").unwrap();
	let menu = MainMenu::new(&menu_button);

	window.window().connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	window.run();
}

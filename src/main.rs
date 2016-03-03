extern crate gtk;
extern crate gdk;
extern crate term_painter;

use gtk::prelude::*;
use term_painter::ToStyle;
use term_painter::Color::*;
// use term_painter::Attr::*;

struct MainWindow {
	window: gtk::Window,
	header_bar: gtk::HeaderBar,
	tool_bar: gtk::Toolbar,
	status_bar: gtk::Statusbar,
}
struct ImageViewport {
	notebook: gtk::Notebook,
	scroll_area: gtk::ScrolledWindow,
	viewport: gtk::Viewport,
	image: gtk::DrawingArea,
}

fn about_dialog_action() {
	let about_dialog = gtk::AboutDialog::new();
	let authors = ["Michael Ditto"];

	about_dialog.set_program_name("OGPaint");
	about_dialog.set_version(Some("0.0.1"));
	about_dialog.set_authors(&authors);
	about_dialog.set_artists(&authors);

	about_dialog.run();
	about_dialog.destroy();
}
fn main() {
	let err_label = Red.bold().paint("[ERR]");
	let info_label = Green.bold().paint("[INFO]");
	let warn_label = Yellow.bold().paint("[WARN]");

	if gtk::init().is_err() {
		println!("{} Failed to initialize gtk.", err_label);
		return;
	}
	println!("{} Using gtk version {}.{}",
		info_label,
		gtk::get_major_version(),
		gtk::get_minor_version());

	let builder = gtk::Builder::new_from_file("assets/main_window.glade");

	let window: gtk::Window = builder.get_object("main_window").unwrap();
	let menu_button: gtk::MenuButton = builder.get_object("main_menu_button").unwrap();
	let menu = gtk::Popover::new(Some(&menu_button));
	let font = gtk::FontChooserWidget::new();

	menu_button.set_direction(gtk::ArrowType::Down);
	menu_button.set_popover(Some(&menu));
	menu_button.set_can_focus(true);

	menu.add(&font);

	font.show();

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	window.show_all();
	gtk::main();
}
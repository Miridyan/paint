extern crate gtk;
extern crate gdk;
extern crate term_painter;

use gtk::prelude::*;
use term_painter::ToStyle;
use term_painter::Color::*;

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
	let font_button: gtk::ToolButton = builder.get_object("font_button").unwrap();
	let color_button: gtk::ToolButton = builder.get_object("toolbar_color_button").unwrap();

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
	{
		let _window = window.clone();
		font_button.connect_clicked(move |_| {
			let lambda_builder = gtk::Builder::new_from_file("assets/font_dialog.glade");

			let font_dialog: gtk::Window = lambda_builder.get_object("font_chooser_popup").unwrap();
			let select_button: gtk::Button = lambda_builder.get_object("font_chooser_select").unwrap();
			let cancel_button: gtk::Button = lambda_builder.get_object("font_chooser_cancel").unwrap();

			font_dialog.set_transient_for(Some(&_window));

			select_button.connect_clicked(|ref_self| {
				ref_self.get_toplevel().unwrap().destroy();
			});
			cancel_button.connect_clicked(|ref_self| {
				ref_self.get_toplevel().unwrap().destroy();
			});
			font_dialog.show_all();
		});
	}
	{
		let _window = window.clone();
		color_button.connect_clicked(move |_| {
			let lambda_builder = gtk::Builder::new_from_file("assets/color_dialog.glade");

			let color_dialog: gtk::Window = lambda_builder.get_object("color_chooser_popup").unwrap();
			let select_button: gtk::Button = lambda_builder.get_object("color_chooser_select").unwrap();
			let cancel_button: gtk::Button = lambda_builder.get_object("color_chooser_cancel").unwrap();

			color_dialog.set_transient_for(Some(&_window));

			select_button.connect_clicked(|ref_self| {
				ref_self.get_toplevel().unwrap().destroy();
			});
			cancel_button.connect_clicked(|ref_self| {
				ref_self.get_toplevel().unwrap().destroy();
			});
			color_dialog.show_all();
		});
	}

	window.show_all();
	gtk::main();
}

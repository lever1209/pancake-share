use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

use crate::interactive_cli;
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

pub fn init_gtk_gui() {
	// Create a new application
	let app = Application::builder().application_id(APP_ID).build();

	// Connect to "activate" signal of `app`
	app.connect_activate(build_ui);

	// Run the application
	app.run_with_args(&[""]);
}

fn build_ui(app: &Application) {
	// Create a button with label and margins
	let button = Button::builder()
		.label("button")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	// Connect to "clicked" signal of `button`
	button.connect_clicked(move |button| {
		// Set the label to "Hello World!" after the button has been clicked on
		match interactive_cli::run_command("test", Some(vec!["nuke".to_owned()])) {
			Ok(o) => button.set_label(&("Ok: ".to_owned() + &o.unwrap_or("None".to_owned()))),
			Err(e) => button.set_label(&("Err: ".to_owned() + &e)),
		}
	});

	// Create a window
	let window = ApplicationWindow::builder()
		.application(app)
		.title("UwU")
		.child(&button)
		.build();

	window.set_width_request(128);

	// Present window
	window.present();
}

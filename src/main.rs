// #![windows_subsystem = "windows"] // cant use or cli fails under windows; immediately calling hide_console_window is the current workaround
#![feature(fn_traits)]

mod interactive_cli;


#[cfg(all(feature = "linux-qt", feature = "linux-gtk"))]
use clap::builder::{self, ArgPredicate};

use clap::{self, *};

#[cfg(feature = "linux-gtk")]
mod gtk_gui;
#[cfg(feature = "windows-native")]
mod w_native_gui;

#[cfg(all(feature = "windows-native", feature = "windows-uwp"))]
compile_error!(
	"Windows native and Windows UWP are incompatible.
Please disable either of the two features."
);

#[cfg(all(feature = "windows-os", feature = "linux-os"))]
compile_error!(
	"Windows and Linux are incompatible.
Please disable either all windows or all linux features."
);

fn main() {
	let mut args = Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(
			arg!(-c --config <VALUE> "Path to an alternate config file.")
				.id("config_path")
				.action(clap::ArgAction::Set),
		);

	#[cfg(any( // TODO assumes cli is default, refactor all instances to include case where cli is absent
		feature = "linux-qt",
		feature = "linux-gtk",
		feature = "windows-uwp",
		feature = "windows-native"
	))]
	{
		args = args.arg(arg!(--nogui).id("nogui").action(clap::ArgAction::SetTrue));
	}

	#[cfg(all(feature = "linux-qt", feature = "linux-gtk"))]
	{
		args = args
			.arg(
				arg!(--gtk "Uses GTK for gui.")
					.conflicts_with_all(["use_qt", "nogui"])
					.id("use_gtk")
					.action(clap::ArgAction::SetTrue)
					.default_value("true")
					.default_value_if("use_qt", ArgPredicate::IsPresent, "false"),
			)
			.arg(
				arg!(--qt "Uses Qt for gui.")
					.conflicts_with_all(["use_gtk", "nogui"])
					.id("use_qt")
					.action(clap::ArgAction::SetTrue),
			);
	}

	#[cfg(feature = "interactive-cli")]
	{
		args = args.arg(
			arg!(-s --script <VALUE> "Allows you to script multiple commands, useful for automation").action(ArgAction::Append),
		);
	}

	let matches = args.get_matches();

	unwrap_config(matches.get_one::<String>("config_path"));

	#[cfg(any(
		feature = "linux-gtk",
		feature = "linux-qt",
		feature = "windows-uwp",
		feature = "windows-native"
	))]
	{
		let handler = std::thread::spawn(move || {
			#[cfg(all(feature = "linux-qt", feature = "linux-gtk"))]
			{
				if matches.get_flag("use_gtk") && !matches.get_flag("nogui") {
					println!("Using GTK");
					gtk_gui::init_gtk_gui();
				}
				if matches.get_flag("use_qt") && !matches.get_flag("nogui") {
					println!("Using Qt");
					// qt_gui::init_qt_gui();
				}
			}
			#[cfg(all(feature = "linux-gtk", not(feature = "linux-qt")))]
			if !matches.get_flag("nogui") {
				println!("Using GTK");
				gtk_gui::init_gtk_gui();
			}
			#[cfg(all(feature = "linux-qt", not(feature = "linux-gtk")))]
			if !matches.get_flag("nogui") {
				println!("Using Qt");
				// qt_gui::init_qt_gui();
			}
			#[cfg(feature = "windows-uwp")]
			if !matches.get_flag("nogui") {
				println!("Using Windows UWP");
				hide_console_window();
				// w_uwp_gui::init_w_uwp_gui();
			}
			#[cfg(feature = "windows-native")]
			if !matches.get_flag("nogui") {
				println!("Using Windows Native");
				hide_console_window();
				w_native_gui::init();
			}
			#[cfg(feature = "interactive-cli")]
			{
				if matches.get_flag("nogui") {
					interactive_cli::init_loop();
				} else if matches.get_one::<String>("script").is_some() {
					match interactive_cli::run_commands(matches.get_one("script").unwrap()) {
						Ok(_) => (),
						Err(x) => println!("Error: {}", x),
					}
				}
			}
		});

		handler.join().unwrap();
	}
	#[cfg(all(
		feature = "interactive-cli",
		not(feature = "linux-gtk"),
		not(feature = "linux-qt"),
		not(feature = "windows-uwp"),
		not(feature = "windows-native")
	))]
	if matches.get_one::<String>("script").is_some() {
		match interactive_cli::run_commands(matches.get_one("script").unwrap()) {
			Ok(_) => (),
			Err(x) => println!("Error: {}", x),
		}
	} else {
		interactive_cli::init_loop();
	}
}

#[cfg(feature = "windows-os")]
fn hide_console_window() {
	unsafe {
		winapi::um::wincon::FreeConsole();
	}
}

fn unwrap_config(path: Option<&String>) {
	
	// TODO
	
}

// struct ConfigStruct<'a> {
// 	key_path: &'a str,
// }

// const CONFIG_DATA: ConfigStruct = ConfigStruct { key_path: "" };

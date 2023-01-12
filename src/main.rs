#![windows_subsystem = "windows"]

use clap::{self, builder::ArgPredicate, *};

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

	dbg!(env!("CARGO_PKG_AUTHORS"));

	#[cfg(any(
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
			arg!(-s --script <VALUE> "Allows you to script multiple commands, useful for automation"),
		);
	}

	let matches = args.get_matches();

	#[cfg(any(
		feature = "linux-gtk",
		feature = "linux-qt",
		feature = "windows-uwp",
		feature = "windows-native",
		feature = "interactive-cli"
	))]
	{
		let handler = std::thread::spawn(move || {
			#[cfg(feature = "linux-gtk")]
			if matches.get_flag("use_gtk") {
				println!("Using GTK");
				gtk_gui::init_gtk_gui();
			}
			#[cfg(feature = "linux-qt")]
			if matches.get_flag("use_qt") {
				println!("Using Qt");
				// qt_gui::init_qt_gui();
			}
			#[cfg(feature = "windows-uwp")]
			if matches.get_flag("use_w_uwp") {
				println!("Using Windows UWP");
				// w_uwp_gui::init_w_uwp_gui();
			}
			#[cfg(feature = "windows-native")]
			if matches.get_flag("use_w_native") {
				println!("Using Windows Native");
				// w_native_gui::init_w_native_gui();
			}
			#[cfg(feature = "interactive-cli")]
			{
				println!("Using interactive CLI"); // unimplemented at the moment
			}
		});

		handler.join().unwrap();
	}
}

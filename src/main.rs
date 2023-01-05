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
	use clap::{arg, Command};

	let mut args = Command::new("media-railway")
		.version("0.0.0")
		.author("Nora S. <leverplays@gmail.com>")
		.about("a simple p2p file/media sharing program");

	#[cfg(all(feature = "linux-qt", feature = "linux-gtk"))]
	{
		args = args
			.arg(
				arg!(--gtk)
					.conflicts_with_all(["use_qt", "nogui"])
					.id("use_gtk")
					.action(clap::ArgAction::SetTrue),
			)
			.arg(
				arg!(--qt)
					.conflicts_with_all(["use_gtk", "nogui"])
					.id("use_qt")
					.action(clap::ArgAction::SetTrue),
			)
			.arg(
				arg!(--nogui)
					.conflicts_with_all(["use_gtk", "use_qt"])
					.id("nogui")
					.action(clap::ArgAction::SetTrue),
			);
	}
	#[cfg(all(feature = "linux-gtk", not(feature = "linux-qt")))]
	{
		args = args
			.arg(
				arg!(--gtk)
					.conflicts_with_all(["nogui"])
					.id("use_gtk")
					.action(clap::ArgAction::SetTrue),
			)
			.arg(
				arg!(--nogui)
					.conflicts_with_all(["use_gtk"])
					.id("nogui")
					.action(clap::ArgAction::SetTrue),
			);
	}
	#[cfg(all(feature = "linux-qt", not(feature = "linux-gtk")))]
	{
		args = args
			.arg(
				arg!(--qt)
					.conflicts_with_all(["nogui"])
					.id("use_qt")
					.action(clap::ArgAction::SetTrue),
			)
			.arg(
				arg!(--nogui)
					.conflicts_with_all(["use_qt"])
					.id("nogui")
					.action(clap::ArgAction::SetTrue),
			);
	}
	#[cfg(all(not(feature = "windows-native"), feature = "windows-uwp"))]
	{
		args = args
			.arg(
				arg!(--gui)
					.conflicts_with_all(["nogui"])
					.id("use_w_uwp")
					.action(clap::ArgAction::SetTrue),
			)
			.arg(
				arg!(--nogui)
					.conflicts_with_all(["use_w_uwp"])
					.id("nogui")
					.action(clap::ArgAction::SetTrue),
			);
	}
	#[cfg(all(not(feature = "windows-uwp"), feature = "windows-native"))]
	{
		args = args
			.arg(
				arg!(--gui)
					.conflicts_with_all(["nogui"])
					.id("use_w_native")
					.action(clap::ArgAction::SetTrue).default_value("true"),
			)
			.arg(
				arg!(--nogui)
					.conflicts_with_all(["use_w_native"])
					.id("nogui")
					.action(clap::ArgAction::SetTrue),
			);
	}

	let matches = args.get_matches();
	// dbg!(&matches);

	#[cfg(feature = "linux-gtk")]
	if matches.get_flag("use_gtk") {
		println!("Using GTK"); // TODO compare println to dbg with --release
		gtk_gui::init_gtk_gui();
	}
	#[cfg(feature = "linux-qt")]
	if matches.get_flag("use_qt") {
		println!("Using Qt");
		qt_gui::init_qt_gui();
	}
	#[cfg(feature = "windows-uwp")]
	if matches.get_flag("use_w_uwp") {
		println!("Using Windows UWP");
		w_uwp_gui::init_w_uwp_gui();
	}
	#[cfg(feature = "windows-native")]
	if matches.get_flag("use_w_native") {
		println!("Using Windows Native");
		w_native_gui::init_w_native_gui();
	}
}

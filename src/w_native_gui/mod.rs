use nwd::NwgUi;
use nwg::NativeUi;

pub fn init() {#[cfg(feature = "debug-logging")]
	println!("Init windows gui");
	nwg::init().expect("Failed to init Native Windows GUI");
	nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
	let _app = WindowsNativeGUI::build_ui(Default::default()).expect("Failed to build UI");
	nwg::dispatch_thread_events();
}

#[derive(Default, NwgUi)]
pub struct WindowsNativeGUI {
	#[nwg_control(size: (300, 115), center: true, title: "Media Railway", flags: "WINDOW|VISIBLE")]
	#[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
	window: nwg::Window,

	#[nwg_layout(parent: window, spacing: 1)]
	grid: nwg::GridLayout,

	#[nwg_control(text: "Text", focus: true)]
	#[nwg_layout_item(layout: grid, row: 0, col: 0)]
	name_edit: nwg::TextInput,

	#[nwg_control(text: "Create Popup")]
	#[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
	#[nwg_events( OnButtonClick: [WindowsNativeGUI::say_hello] )]
	hello_button: nwg::Button,
}

impl WindowsNativeGUI {
	fn say_hello(&self) {
		// nwg::modal_info_message(&self.window, "Popup", "");// &format!("{}", self.name_edit.text()));
		
		let params = nwg::MessageParams {
			title: "Popup",
			content: &format!("{}", self.name_edit.text()),
			buttons: nwg::MessageButtons::Ok,
			icons: nwg::MessageIcons::Info
		};
	
		nwg::modal_message(&self.window, &params);
		
	}
}

use seed::{self, prelude::*, *};

use web_sys::{ self, FileList, FormData };

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	FilesChanged(Option<FileList>),
	SendUpload(FormData),
}

pub fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::FilesChanged(files_opt) => {
			if let Some(files) = files_opt {
				for i in 0..files.length()
				{
					if let Some(file) = files.get(i) {
						if let Ok(form_data) = FormData::new() {
							if form_data.append_with_blob("file", &file).is_ok() {
								log!(form_data)
							}
						}
						
					}
				}
			}
		},
		Msg::SendUpload(form_data) => {

		}
	}

}

pub fn view() -> Node<Msg> {
	div![
		C!("field"),
		div![
			C!("control"),
			div![
				C!["file", "is-centered", "is-medium", "is-success", "is-boxed"],
				label![
					C!("file-label"),
					input![
						C!("file-input"),
						attrs! { 
							At::Type => "file", 
							At::Name => "pictures", 
							At::Accept => "image/*",
							At::Multiple => "multiple"
						},
						ev(Ev::Change, |event| {
							let files = event
								.target()
								.and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
								.and_then(|file_input| file_input.files());
		
							Msg::FilesChanged(files)
						})
					],
					span![
						C!("file-cta"),
						span![C!("file-icon"), i![C!["ion-upload"]]],
						span![C!("file-label"), "Add pictures"]
					]
				]
			]
		]
	]
}
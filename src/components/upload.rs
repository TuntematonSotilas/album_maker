use seed::{self, prelude::*, *};
use uuid::Uuid;
use web_sys::{ self, FileList, FormData };
use load_dotenv::load_dotenv;

use crate::models::{vars::UPLOAD_URI, picture::Picture};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
	group_id: Uuid,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	FilesChanged(Option<FileList>, Uuid),
	SendUpload(FormData),
	Success(Picture, Uuid),
	Error,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::FilesChanged(files_opt, group_id) => {
			model.group_id = group_id;
			if let Some(files) = files_opt {
				for i in 0..files.length()
				{
					if let Some(file) = files.get(i) {
						if let Ok(form_data) = FormData::new() {
							load_dotenv!();
							let upload_preset = env!("UPLOAD_PRESET");
							if form_data.append_with_blob("file", &file).is_ok() &&
								form_data.append_with_str("upload_preset", upload_preset).is_ok() {
								orders.send_msg(Msg::SendUpload(form_data));
							}
						}
						
					}
				}
			}
		}
		Msg::SendUpload(form_data) => {
			orders.skip(); // No need to rerender
			let group_id = model.group_id.clone();
			let uri = UPLOAD_URI.to_string();
			let request = Request::new(uri)
                .method(Method::Post)
                .body(JsValue::from(form_data));
			
			orders.perform_cmd(async {
				//let gr = group_id.to_owned();
				let response = fetch(request).await.expect("HTTP request failed");
				if response.status().is_ok() {
					let res_pic = response.json::<Picture>().await;
					if let Ok(picture) = res_pic {
						Msg::Success(picture, Uuid::from(group_id))
					} else {
						Msg::Error
					}
				} else {
					Msg::Error
				}
			});
			
		}
		Msg::Success(_, _) => (),
		Msg::Error => {
			error!("Error upload")
		}
	}

}

pub fn view(group_id: Uuid) -> Node<Msg> {
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
						ev(Ev::Change, move |event| {
							let files = event
								.target()
								.and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
								.and_then(|file_input| file_input.files());
		
							Msg::FilesChanged(files, group_id.clone())
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
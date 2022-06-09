use load_dotenv::load_dotenv;
use seed::{self, prelude::*, *};
use uuid::Uuid;
use web_sys::{self, FileList, FormData};

use crate::models::{picture::Picture, vars::UPLOAD_URI};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    FilesChanged(Option<FileList>, Uuid),
    RenderFakePictures(u32, Uuid),
    SendUpload(FormData, Uuid),
    Success(Picture, Uuid),
    Error,
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FilesChanged(files_opt, group_id) => {
			load_dotenv!();
            if let Some(files) = files_opt {
                let count = files.length();
                orders.send_msg(Msg::RenderFakePictures(count, group_id));
                for i in 0..count {
                    if let Some(file) = files.get(i) {
                        if let Ok(form_data) = FormData::new() {
                            let upload_preset = env!("UPLOAD_PRESET");
                            _ = form_data.append_with_blob("file", &file);
							_ = form_data.append_with_str("upload_preset", upload_preset);
							let folder = "amaker/".to_string() + group_id.to_string().as_str();
							_ = form_data.append_with_str("folder", folder.as_str());
                            orders.send_msg(Msg::SendUpload(form_data, group_id));
                        }
                    }
                }
            }
        }
        Msg::SendUpload(form_data, group_id) => {
            let uri = UPLOAD_URI.to_string();
            let request = Request::new(uri)
                .method(Method::Post)
                .body(JsValue::from(form_data));

            orders.perform_cmd(async move {
                let response = fetch(request).await.expect("HTTP request failed");
                if response.status().is_ok() {
                    let res_pic = response.json::<Picture>().await;
                    if let Ok(picture) = res_pic {
                        Msg::Success(picture, group_id)
                    } else {
                        Msg::Error
                    }
                } else {
                    Msg::Error
                }
            });
        }
        Msg::RenderFakePictures(_, _) | Msg::Success(_, _) => (),
        Msg::Error => {
            error!("Error when uploading");
        }
    }
}

pub fn view(group_id: Uuid) -> Node<Msg> {
    div![
        C!("field mt-2"),
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
                                .and_then(|target| {
                                    target.dyn_into::<web_sys::HtmlInputElement>().ok()
                                })
                                .and_then(|file_input| file_input.files());

                            Msg::FilesChanged(files, group_id)
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

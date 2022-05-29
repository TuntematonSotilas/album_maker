use load_dotenv::load_dotenv;
use seed::{self, prelude::*, *};
use uuid::Uuid;
use web_sys::{self, FileList, FormData};

use crate::models::{group::Group, picture::Picture, vars::UPLOAD_URI};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    group: Group,
}

impl Model {
    pub fn new() -> Self {
        Self {
            group: Group::new(),
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    FilesChanged(Option<FileList>, Group),
    RenderFakePictures(u32, Group),
    SendUpload(FormData),
    Success(Picture, Uuid),
    Error,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FilesChanged(files_opt, group) => {
            model.group = group;
			let gr = model.group.clone();
            if let Some(files) = files_opt {
                let count = files.length();
                orders.send_msg(Msg::RenderFakePictures(count, gr));
                for i in 0..count {
                    if let Some(file) = files.get(i) {
                        if let Ok(form_data) = FormData::new() {
                            load_dotenv!();
                            let upload_preset = env!("UPLOAD_PRESET");
                            if form_data.append_with_blob("file", &file).is_ok()
                                && form_data
                                    .append_with_str("upload_preset", upload_preset)
                                    .is_ok()
                            {
                                orders.send_msg(Msg::SendUpload(form_data));
                            }
                        }
                    }
                }
            }
        }
        Msg::SendUpload(form_data) => {
            let group_id = model.group.id;
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

pub fn view(group: Group) -> Node<Msg> {
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
                                .and_then(|target| {
                                    target.dyn_into::<web_sys::HtmlInputElement>().ok()
                                })
                                .and_then(|file_input| file_input.files());

                            Msg::FilesChanged(files, group)
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

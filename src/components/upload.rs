use std::path::Path;

use load_dotenv::load_dotenv;
use seed::{self, prelude::*, *};
use uuid::Uuid;
use web_sys::{self, FileList, FormData};

use crate::{
    api::albumapi,
    models::{
        notif::{Notif, TypeNotifs},
        picture::Picture,
    },
};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    FilesChanged(Option<FileList>, String, Uuid),
    RenderFakePictures(u32, Uuid),
    SendUpload(FormData, String, Uuid),
    Success(Picture, Uuid),
    Error,
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FilesChanged(files_opt, album_id, group_id) => {
            load_dotenv!();
            if let Some(files) = files_opt {
                let count = files.length();
                orders.send_msg(Msg::RenderFakePictures(count, group_id));
                for i in 0..count {
                    if let Some(file) = files.get(i) {
                        if let Ok(form_data) = FormData::new() {
                            let upload_preset = env!("CLD_UPLOAD_PRESET");
                            let folder = format!("amaker/{album_id}");
                            let file_res = form_data.append_with_blob("file", &file);
                            let preset_res_ =
                                form_data.append_with_str("upload_preset", upload_preset);
                            let folder_res = form_data.append_with_str("folder", folder.as_str());
                            if file_res.is_ok() && preset_res_.is_ok() && folder_res.is_ok() {
                                orders.send_msg(Msg::SendUpload(form_data, file.name(), group_id));
                            }
                        }
                    }
                }
            }
        }
        Msg::SendUpload(form_data, name, group_id) => {
            orders.skip(); // No need to rerender
            orders.perform_cmd(async move {
                let pic_opt = albumapi::upload_picture(form_data).await;
                match pic_opt {
                    Some(mut pic) => {
                        let name = Path::new(&name).file_stem().unwrap_or_default();
                        let name = name.to_str().unwrap_or_default().to_string();
                        pic.caption = Some(name);
                        Msg::Success(pic, group_id)
                    }
                    None => Msg::Error,
                }
            });
        }
        Msg::RenderFakePictures(_, _) | Msg::Success(_, _) => (),
        Msg::Error => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error uploading picture".to_string(),
            });
        }
    }
}

pub fn view(album_id: String, group_id: Uuid) -> Node<Msg> {
    div![
        C!("field mt-2"),
        div![
            C!("control"),
            div![
                C!["file", "is-centered", "is-small", "is-success", "is-boxed"],
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

                            Msg::FilesChanged(files, album_id, group_id)
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

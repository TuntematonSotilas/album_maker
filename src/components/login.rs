use crate::models::{
    notif::{Notif, TypeNotifs},
    page::TITLE_LOGIN,
    vars::{AUTH_HEAD, BASE_URI},
};
use gloo_net::http::{Method, Request};
use seed::{self, prelude::*, *};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    username: String,
    password: String,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    Submit,
    UsernameChanged(String),
    PwdChanged(String),
    SetAuth(String),
    NotifyError,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Submit => {
            orders.skip(); // No need to rerender
            let uri = BASE_URI.to_string() + "login";
            let b64 = base64::encode(format!("{}:{}", model.username, model.password));
            let auth = format!("Basic {b64}");

            orders.perform_cmd(async move {
                let response = Request::new(&uri)
                    .method(Method::POST)
                    .header(AUTH_HEAD, &auth)
                    .send()
                    .await
                    .expect("HTTP request failed");

                if response.status() == 200 {
                    Msg::SetAuth(auth.clone())
                } else {
                    Msg::NotifyError
                }
            });
        }
        Msg::NotifyError => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Login error".to_string(),
            });
        }
        Msg::UsernameChanged(username) => model.username = username,
        Msg::PwdChanged(password) => model.password = password,
        Msg::SetAuth(_) => (),
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["columns", "is-centered", "mt-5"],
        div![
            C!["column", "is-half"],
            div![
                C!("box"),
                p![C!["title", "is-5", "has-text-link"], TITLE_LOGIN],
                div![
                    C!("field"),
                    div![
                        C!("control"),
                        input![
                            C!("input"),
                            attrs! {
                                At::Type => "text",
                                At::Name => "username",
                                At::Placeholder => "Username",
                                At::Value => model.username,
                            },
                            input_ev(Ev::Input, Msg::UsernameChanged),
                        ]
                    ],
                ],
                div![
                    C!("field"),
                    div![
                        C!("control"),
                        input![
                            C!("input"),
                            attrs! {
                                At::Type => "password",
                                At::Name => "password",
                                At::Value => model.password,
                            },
                            input_ev(Ev::Input, Msg::PwdChanged),
                        ]
                    ]
                ],
                div![
                    C!("field"),
                    div![
                        C!("control"),
                        button![
                            C!["button", "is-primary"],
                            "LOGIN",
                            attrs! {At::Type => "Submit"},
                            ev(Ev::Click, |_| Msg::Submit),
                        ]
                    ]
                ],
            ],
        ],
    ]
}

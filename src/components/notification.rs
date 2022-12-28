use crate::models::notif::{Notif, TypeNotifs};
use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    pub is_visible: bool,
    pub notif: Notif,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            is_visible: false,
            notif: Notif::new(),
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    Show(Notif),
    Hide,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Show(notif) => {
            model.is_visible = true;
            model.notif = notif;
            let time = match model.notif.notif_type {
                TypeNotifs::Share => 20000,
                _ => 3000
            };
            orders.perform_cmd(cmds::timeout(time, || Msg::Hide));
        }
        Msg::Hide => model.is_visible = false,
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let c_visible = match &model.is_visible {
        true => "notif-anim",
        _ => "",
    };
    let c_type = match &model.notif.notif_type {
        TypeNotifs::Success | TypeNotifs::Share => "is-success",
        TypeNotifs::Error => "is-danger",
    };
    div![
        C!["notification", "notif", c_type, c_visible],
        &model.notif.message
    ]
}

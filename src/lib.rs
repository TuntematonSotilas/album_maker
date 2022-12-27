// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![allow(clippy::unused_unit)]

use crate::components::*;
use models::{
    notif::Notif,
    page::{Page, LK_LOGIN},
};
use seed::{prelude::*, *};

mod api;
mod components;
mod models;

// ------ ------
//     Init
// ------ ------
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::ShowNotif);
    orders.subscribe(Msg::UrlChanged);

    orders.notify(subs::UrlRequested::new(url));

    let login_page = models::page::Page::Login;

    Model {
        is_logged: false,
        header: header::Model::new(login_page.clone()),
        notification: notification::Model::new(),
        my_albums: my_albums::Model::default(),
        edit_album: edit_album::Model::new(),
        view_album: view_album::Model::new(),
        slideshow: slideshow::Model::new(),
        page: login_page,
        login: login::Model::default(),
        my_sharings: my_sharings::Model::default(),
    }
}

// ------ ------
//     Model
// ------ ------
struct Model {
    is_logged: bool,
    header: header::Model,
    page: models::page::Page,
    my_albums: my_albums::Model,
    edit_album: edit_album::Model,
    view_album: view_album::Model,
    slideshow: slideshow::Model,
    notification: notification::Model,
    login: login::Model,
    my_sharings: my_sharings::Model,
}

// ------ ------
//    Update
// ------ ------
enum Msg {
    Header(header::Msg),
    MyAlbums(my_albums::Msg),
    EditAlbum(edit_album::Msg),
    ViewAlbum(view_album::Msg),
    Slideshow(slideshow::Msg),
    Login(login::Msg),
    UrlChanged(subs::UrlChanged),
    InitComp(Option<String>),
    SetAuth(String),
    Notification(notification::Msg),
    SetIsLogged,
    ShowNotif(Notif),
    MySharings(my_sharings::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Header(msg) => {
            if let header::Msg::LogInOrOut = msg {
                if model.is_logged {
                    orders.send_msg(Msg::SetAuth(String::new()));
                    model.is_logged = false;
                }
                let url = Url::new().add_path_part(LK_LOGIN);
                orders.notify(subs::UrlRequested::new(url));
            }
            header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
        }
        Msg::Notification(msg) => {
            notification::update(
                msg,
                &mut model.notification,
                &mut orders.proxy(Msg::Notification),
            );
        }
        Msg::ShowNotif(notif) => {
            orders.send_msg(Msg::Notification(notification::Msg::Show(notif)));
        }
        Msg::MyAlbums(msg) => {
            my_albums::update(msg, &mut model.my_albums, &mut orders.proxy(Msg::MyAlbums));
        }
        Msg::EditAlbum(msg) => {
            edit_album::update(
                msg,
                &mut model.edit_album,
                &mut orders.proxy(Msg::EditAlbum),
            );
        }
        Msg::ViewAlbum(msg) => {
            view_album::update(
                msg,
                &mut model.view_album,
                &mut orders.proxy(Msg::ViewAlbum),
            );
        }
        Msg::Slideshow(msg) => {
            slideshow::update(msg, &mut model.slideshow, &mut orders.proxy(Msg::Slideshow));
        }
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            let page = match url.next_path_part() {
                Some(models::page::LK_NEW_ALBUM) => models::page::Page::NewAlbum,
                Some(models::page::LK_VIEW_ALBUM) => models::page::Page::ViewAlbum,
                Some(models::page::LK_EDIT_ALBUM) => models::page::Page::EditAlbum,
                Some(models::page::LK_SLIDESHOW) => models::page::Page::Slideshow,
                Some(models::page::LK_LOGIN) => models::page::Page::Login,
                Some(models::page::LK_MY_SHARINGS) => models::page::Page::MySharings,
                Some(models::page::LK_SHARE) => models::page::Page::Share,
				Some(models::page::LK_SHARESLIDE) => models::page::Page::ShareSlide,
                _ => models::page::Page::MyAlbums,
            };

			log!(page);

            model.page = page.clone();

            orders.send_msg(Msg::Header(header::Msg::SetPage(page)));

            let opt_id: Option<String> = url.next_path_part().map(str::to_string);
            orders.send_msg(Msg::InitComp(opt_id));
        }
        Msg::InitComp(opt_id) => {
			log!("InitComp");
            if model.is_logged || model.page == Page::Share || model.page == Page::ShareSlide {
                init_comp(&model.page, opt_id, orders);
            }
        }
        Msg::Login(msg) => {
            if let login::Msg::SetAuth(ref auth) = msg {
                orders.send_msg(Msg::SetIsLogged);
                orders.send_msg(Msg::SetAuth(auth.clone()));
                orders.notify(subs::UrlRequested::new(Url::new()));
            }
            login::update(msg, &mut model.login, &mut orders.proxy(Msg::Login));
        }
        Msg::SetIsLogged => {
            model.is_logged = true;
            orders.send_msg(Msg::Header(header::Msg::SetIsLogged));
        }
        Msg::SetAuth(auth) => {
            orders.send_msg(Msg::EditAlbum(edit_album::Msg::SetAuth(auth.clone())));
            orders.send_msg(Msg::MyAlbums(my_albums::Msg::SetAuth(auth.clone())));
            orders.send_msg(Msg::ViewAlbum(view_album::Msg::SetAuth(auth.clone())));
            orders.send_msg(Msg::Slideshow(slideshow::Msg::SetAuth(auth.clone())));
            orders.send_msg(Msg::MySharings(my_sharings::Msg::SetAuth(auth)));
        }
        Msg::MySharings(msg) => {
            my_sharings::update(msg, &mut model.my_sharings, &mut orders.proxy(Msg::MySharings));
        }
    }
}

fn init_comp(page: &Page, opt_id: Option<String>, orders: &mut impl Orders<Msg>) {
    match page {
        models::page::Page::MyAlbums => {
            orders.send_msg(Msg::MyAlbums(my_albums::Msg::InitComp));
        }
        models::page::Page::NewAlbum => {
            orders.send_msg(Msg::EditAlbum(edit_album::Msg::InitComp(None)));
        }
        models::page::Page::EditAlbum => {
            orders.send_msg(Msg::EditAlbum(edit_album::Msg::InitComp(opt_id)));
        }
        models::page::Page::ViewAlbum => {
            orders.send_msg(Msg::ViewAlbum(view_album::Msg::InitComp(opt_id, None)));
        }
        models::page::Page::Slideshow => {
            orders.send_msg(Msg::Slideshow(slideshow::Msg::InitComp(opt_id, None)));
        }
        models::page::Page::MySharings => {
            orders.send_msg(Msg::MySharings(my_sharings::Msg::InitComp));
        }
        models::page::Page::Share => {
            orders.send_msg(Msg::ViewAlbum(view_album::Msg::InitComp(None, opt_id)));
        }
		models::page::Page::ShareSlide => {
           orders.send_msg(Msg::Slideshow(slideshow::Msg::InitComp(None, opt_id)));
        }
        models::page::Page::Login => (),
    }
}

// ------ ------
//     View
// ------ ------
fn view(model: &Model) -> Node<Msg> {
	let mut c_cont = "";
	if model.page != Page::Slideshow && model.page != Page::ShareSlide {
		c_cont = "columns is-centered m-1";
	}
    div![
        notification::view(&model.notification).map_msg(Msg::Notification),
        header::view(&model.header).map_msg(Msg::Header),
        div![
            match &model.page {
                models::page::Page::Login => login::view(&model.login).map_msg(Msg::Login),
                models::page::Page::Share =>  div![
                    C!("columns is-centered m-1"),
                    view_album::view(&model.view_album).map_msg(Msg::ViewAlbum) 
                ],
				models::page::Page::ShareSlide =>  div![
					slideshow::view(&model.slideshow).map_msg(Msg::Slideshow)
                ],
                _ => match &model.is_logged {
                    true => {
                        div![
							C!(c_cont),
                            match &model.page {
                                models::page::Page::NewAlbum | models::page::Page::EditAlbum =>
                                    edit_album::view(&model.edit_album).map_msg(Msg::EditAlbum),
                                models::page::Page::MyAlbums =>
                                    my_albums::view(&model.my_albums).map_msg(Msg::MyAlbums),
                                models::page::Page::ViewAlbum =>
                                    view_album::view(&model.view_album).map_msg(Msg::ViewAlbum),
                                models::page::Page::MySharings =>
                                    my_sharings::view(&model.my_sharings).map_msg(Msg::MySharings),
								models::page::Page::Slideshow =>
                                    slideshow::view(&model.slideshow).map_msg(Msg::Slideshow),
                                _ => empty!(),
                            }
                        ]
                    }
                    false => error::view(
                        "Please log in to continue".to_string(),
                        "ion-log-in".to_string()
                    ),
                },
            }
        ]
    ]
}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

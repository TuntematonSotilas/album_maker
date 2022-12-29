use seed::{prelude::*, *};

use crate::models::page::LK_LOGIN;

pub fn view<Ms>(msg: String, icon: String) -> Node<Ms> {
    div![
        C!["block", "is-flex", "is-align-items-center", "is-flex-direction-column", "mt-6"],
        div![
            C!["icon", "is-size-1", "has-text-info"], 
            i![C!(icon)]
        ],
        div![C!("m-2"),
            h1![C!["title", "has-text-centered"], msg]
        ],
        a![
            C!["button", "is-light"],
            attrs!{ At::Href => format!("/{LK_LOGIN}")},
            "Sign In"
        ]
    ]
}

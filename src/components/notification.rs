use seed::{prelude::*, *};

pub enum NotifType {
	Success,
}

pub fn view<Ms>(notif_type: NotifType, msg: String) -> Node<Ms> {
	let c_type = match notif_type {
		Sucess => "is-success",
		_ => "is-info",
	};
	div![C!["notification", c_type],
        button![C!["delete"],
			msg
        ],
    ]
}
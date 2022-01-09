use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
	div![C!["hero", "is-large", "columns", "is-flex", "is-vcentered"],
        div![C!["hero-body"],
			section![
				span![C!["icon", "is-size-1", "has-text-info"],
					i![C!("ion-log-in")]
				],
			],
			section![
				h1![C!["title"],
					"Please log in to continue",
				],
			],
        ],
    ]
}
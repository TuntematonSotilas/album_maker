use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
	div![C!["hero", "is-large"],
        div![C!["hero-body"],
			div![C!["is-flex", "is-justify-content-center", "m-3"],
				div![C!["icon", "is-size-1", "has-text-info"],
					i![C!("ion-log-in")]
				],
			],
			div![C!["is-flex", "is-justify-content-center"],
				h1![C!["title"],
					"Please log in to continue",
				],
			],
        ],
    ]
}
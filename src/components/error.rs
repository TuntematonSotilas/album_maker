use seed::{prelude::*, *};

pub fn view<Ms>(msg: String, icon: String) -> Node<Ms> {
	div![C!["hero", "is-large"],
        div![C!("hero-body"),
			div![C!["is-flex", "is-justify-content-center", "block"],
				div![C!["icon", "is-size-1", "has-text-info"],
					i![C!(icon)]
				],
			],
			div![C!["is-flex", "is-justify-content-center"],
				h1![C!("title"),
					msg,
				],
			],
        ],
    ]
}
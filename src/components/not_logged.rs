use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
	div![C!["hero", "is-medium"],
        div![C!["hero-body"],
            h1![C!["title"],
                "Please log in to continue",
            ],
        ]
    ]
}
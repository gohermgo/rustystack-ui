#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn button(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            ul {
                li { "an item" }
                li { "the next item" }
            }
            button {
                "button text"
            }
        }
    })
}

fn header(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            class: "ui secondary menu",
            a {
                href: "http://yoink.ltd",
                class: "header item",
                "yoink"
            }
            a {
                href: "https://www.youtube.com",
                class: "item",
                "youtube"
            }
            a {
                href: "https://www.google.com",
                class: "item",
                "google"
            }
            a {
                class: "item",
                "link"
            }
            div {
                class: "right menu",
                div {
                    class: "item",
                    div {
                        class: "ui action left icon input",
                        i { class: "search icon" },
                        input { "type": "text", placeholder: "search" },
                    button { class: "inverted blue ui button", "submit" }
                    }
                }
                a {
                    class: "item",
                    "link"
                }
            }
        }
    })
}

struct Card {
    title: String,
    description: String,
    git: String,
    imsrc: String,
}

impl Card {
    fn image(cx: Scope<()>, imsrc: String) -> Element {
        cx.render(rsx! {
            div {
                class: "image dimmable",
                div {
                    class: "ui blurring inverted dimmer transition hidden",
                    div {
                        class: "content",
                        div {
                            class: "center",
                            div {
                                class: "ui teal button",
                                "add friend"
                            }
                        }
                    }
                }
                img {
                    alt: "card image broken",
                    src: "{imsrc}"
                }
            }
        })
    }
    fn content(cx: Scope<()>, title: String, description: String) -> Element {
        cx.render(rsx! {
            div {
                class: "content",
                div {
                    class: "header",
                    "{title}"
                }
                div {
                    class: "meta",
                    a {
                        class: "group",
                        "meta"
                    }
                }
                div {
                    class: "description",
                    "{description}"
                }
            }
        })
    }
    pub fn add(cx: Scope <()>, card: Card) -> Element {
        cx.render(rsx! {
            div {
                class: "ui card",
                Self::image(cx, card.imsrc.clone()),
                Self::content(cx, card.title.clone(), card.description.clone()),
                div {
                    class: "extra content",
                    a {
                        href: "{card.git.clone()}",
                        class: "right floated created",
                        "git"
                    }
                    a {
                        class: "friends",
                        "arbitrary"
                    }
                }
            }
        })
    }
}

fn cards(cx: Scope<()>) -> Element {
    let placeholder_source = String::from("https://placehold.co/400");
    cx.render(rsx! {
        h1 {
            margin_left: "0.5em",
            "earlier projects"
        }
        div {
            // An outer div to keep the inner cards from touching the "edges"
            padding_left: "1em",
            padding_right: "1em",
            div {
                class: "ui four cards",
                Card::add(cx, Card {
                    title: String::from("rustystack"),
                    description: String::from("A project consisting of a website, whose stack is entirely in rust (this was hardcoded into that stack)"),
                    git: String::from("https://github.com/gohermgo/rustystack"),
                    imsrc: placeholder_source.clone()
                })
                Card::add(cx, Card {
                    title: String::from("project"),
                    description: String::from("description"),
                    git: String::from("localhost:8080"),
                    imsrc: placeholder_source.clone()
                })
                Card::add(cx, Card {
                    title: String::from("project"),
                    description: String::from("description"),
                    git: String::from("localhost:8080"),
                    imsrc: placeholder_source.clone()
                })
                Card::add(cx, Card {
                    title: String::from("project"),
                    description: String::from("description"),
                    git: String::from("localhost:8080"),
                    imsrc: placeholder_source.clone()
                })
            }
        }
    })
}

fn list_item(cx: Scope<()>, item: (String, String)) -> Element {
    cx.render(rsx! {
        div {
            class: "item",
            div {
                class: "content",
                div {
                    class: "header",
                    "{item.0}"
                }
                "{item.1}"
            }
        }
    })
}


fn inverted_list(cx: Scope<()>, items: Vec<(String, String)>) -> Element {
    cx.render(rsx! {
        div {
            class: "ui inverted segment",
            div {
                class: "ui inverted relaxed divided list",
                items.iter().map(|item| list_item(cx, item.clone()))
            }
        }
    })
}

fn app(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            font: "14px 'Helvetica Neue', Helvetica, Arial, sans-serif",
            width: "100%",
            header(cx),
            div {
                margin: "0.5em",
                class: "ui grid",
                div {
                    class: "eleven wide column",
                    img {
                        class: "ui image rounded",
                        src: "https://placehold.co/1920x1080"
                    }
                }
                div {
                    class: "three wide column",
                    align_content: "left",
                    h1 { class: "title", "eiffel tower" }
                    p { font_size: "16px", "paragraph text lorem ipsum suck my nuts, woow its the fucking eiffel tower, oh my goooood i cant believe it, have you ever seen such an eiffel tower? its like tall and shit" }
                    // "paragraph text lorem ipsum suck my nuts, woow its the fucking eiffel tower, oh my goooood i cant believe it, have you ever seen such an eiffel tower? its like tall and shit"                
                }
                // div {
                //     class: "seven wide column",
                //     img {
                //         class: "ui image rounded",
                //         src: "https://placehold.co/800x500"
                //     }
                // }
                div {
                    class: "two wide column",
                    div {
                        // class: "ui inverted segment",
                        // div {
                        //     class: "item",
                        //     div {
                        //         class: "content",
                        //         div {
                        //             class: "header",
                        //             "item header"
                        //         }
                        //         "item info"
                        //     }
                            
                        // }
                        inverted_list(cx, (0..11).map(|x| (format!("title {}", x + 1), format!("description describing thing mentioned in title {}", x + 1).to_string())).collect())
                    }
                }
            }
            cards(cx),
        }
    })
}

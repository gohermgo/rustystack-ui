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
            // a {
            //     href: "http://yoink.ltd",
            //     class: "header item",
            //     "yoink"
            // }
            div {
                class: "item",
                a {
                    href: "http://www.yoink.ltd",
                    class: "ui jiggle transition header",
                    margin_left: "0.25em",
                    font_size: "64px",
                    "yoink",
                }
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
    pub fn add(cx: Scope<()>, card: Card) -> Element {
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
        div {
            // An outer div to keep the inner cards from touching the "edges"
            // padding_left: "1em",
            // padding_right: "1em",
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

fn earlier_work(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            h1 {
                class: "ui dividing header",
                margin_left: "0.5em",
                "earlier projects"
            }
            div {
                class: "ui grid",
                div {
                    class: "two wide column",
                    div {
                        class: "ui grid",
                        margin_left: "0.5em",
                        margin_right: "0.5em",
                        div {
                            class: "row",
                            h3 {
                                class: "ui header",
                                "categories"
                            }
                        }
                        div {
                            class: "row",
                            div {
                                class: "ui link list",
                                div {
                                    class: "active item",
                                    "all"
                                }
                                a {
                                    class: "item",
                                    "some category"
                                }
                                a {
                                    class: "item",
                                    "other category"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "fourteen wide column",
                    cards(cx)
                }
            }
        }
    })
}

#[derive(Clone)]
struct ListItem {
    title: String,
    description: String,
}

impl ListItem {
    pub fn add(cx: Scope<()>, list_item: ListItem) -> Element {
        cx.render(rsx! {
            div {
                class: "item",
                div {
                    class: "content",
                    div {
                        class: "header",
                        "{list_item.title}"
                    }
                    "{list_item.description}"
                }
            }
        })
    }
}

fn inverted_list(cx: Scope<()>, items: Vec<ListItem>) -> Element {
    cx.render(rsx! {
        div {
            class: "ui inverted segment",
            div {
                class: "ui inverted relaxed divided list",
                items.iter().map(|item| ListItem::add(cx, item.clone()))
            }
        }
    })
}

fn introduction(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            // padding_left: "1em",
            // padding_right: "1em",
            class: "ui grid",
            div {
                class: "twelve wide column",
                img {
                    class: "ui image rounded",
                    src: "https://placehold.co/1920x1080"
                }
            }
            div {
                class: "four wide column",
                align_content: "left",
                h1 { class: "title", "eiffel tower" }
                p { font_size: "16px", "paragraph text lorem ipsum suck my nuts, woow its the fucking eiffel tower, oh my goooood i cant believe it, have you ever seen such an eiffel tower? its like tall and shit" }
                // "paragraph text lorem ipsum suck my nuts, woow its the fucking eiffel tower, oh my goooood i cant believe it, have you ever seen such an eiffel tower? its like tall and shit"                
            }
        }
    })
}

fn app_main_body(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            class: "ui grid",
            div {
                class: "row",
                introduction(cx),
            }
            div {
                class: "row",
                earlier_work(cx)
            }
        }
    })
}

fn app(cx: Scope<()>) -> Element {
    let placeholder_list: Vec<ListItem> = (0..20)
        .map(|x| ListItem {
            title: format!("title {}", x + 1),
            description: format!("description of element {}", x + 1),
        })
        .collect();
    cx.render(rsx! {
        div {
            font: "14px 'Helvetica Neue', Helvetica, Arial, sans-serif",
            width: "100%",
            header(cx),
            div {
                // margin: "0.5em",
                class: "ui grid",
                div {
                    padding: "0px",
                    class: "fourteen wide column",
                    div {
                        margin: "1em",
                        padding_left: "2em",
                        padding_right: "2em",
                        div {
                            app_main_body(cx)
                        }
                    }
                }
                div {
                    class: "two wide column",
                    padding: "0px",
                    div {
                        inverted_list(cx, placeholder_list)
                    }
                }
            }
        }
    })
}

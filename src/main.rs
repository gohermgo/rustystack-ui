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

fn card_image(cx: Scope<()>, image_source: String) -> Element {
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
                alt: "image broken",
                src: "{image_source}"
            }
        }
    })
}

fn card_content(cx: Scope<()>, card_title: String, card_description: String) -> Element {
    cx.render(rsx! {
        div {
            class: "content",
            div {
                class: "header",
                "{card_title}"
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
                "{card_description}"
            }
        }
    })
}

fn card(
    cx: Scope<()>,
    image_source: String,
    card_git: String,
    card_title: String,
    card_description: String,
) -> Element {
    cx.render(rsx! {
        div {
            class: "ui card",
            card_image(cx, image_source),
            card_content(cx, card_title, card_description),
            div {
                class: "extra content",
                a {
                    href: "{card_git}",
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
                card(cx, placeholder_source.clone(), String::from("https://github.com/gohermgo/rustystack"), String::from("rustystack"), String::from("A project consisting of a website, whose stack is entirely in rust (this was hardcoded into that stack)")),
                card(cx, placeholder_source.clone(), String::from("https://github.com/gohermgo"), String::from("project 1"), String::from("empty description")),
                card(cx, placeholder_source.clone(), String::from("https://github.com/gohermgo"), String::from("project 2"), String::from("empty description")),
                card(cx, placeholder_source.clone(), String::from("https://github.com/gohermgo"), String::from("project 3"), String::from("empty description"))
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
                    class: "nine wide column",
                    img {
                        class: "ui image rounded",
                        src: "https://images.pexels.com/photos/2260800/pexels-photo-2260800.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1"
                    }
                }
                div {
                    class: "seven wide column",
                    h1 { "Paragraph header" }
                    p { "paragraph text lorem ipsum suck my nuts" }
                }
            }
            cards(cx),
        }
    })
}

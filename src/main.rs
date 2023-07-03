#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn button(cx: Scope<()>) -> Element {
    render!(rsx! {
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
    render!(rsx! {
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
                    font_size: "34px",
                    color: "white",
                    "yoink",
                }
            }
            a {
                href: "https://www.youtube.com",
                class: "item",
                font_size: "20px",
                color: "white",
                "youtube"
            }
            a {
                href: "https://www.google.com",
                class: "item",
                font_size: "20px",
                color: "white",
                "google"
            }
            a {
                class: "item",
                font_size: "20px",
                color: "white",
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
                    color: "white",
                    "link"
                }
            }
        }
    })
}

fn inverted_header(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            class: "ui inverted segment",
            header(cx)
        }
    })
}

#[derive(Clone, PartialEq)]
struct GitLink {
    user: String,
    repo: String,
}

impl Default for GitLink {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl GitLink {
    fn url(&self) -> String {
        format!("https://www.github.com/{}/{}.git", self.user, self.repo)
    }
}

impl From<GitLink> for String {
    fn from(value: GitLink) -> Self {
        format!("https://www.github.com/{}/{}.git", value.user, value.repo)
    }
}

#[derive(Clone, PartialEq)]
enum CardCategory {
    Freetime,
    Professional,
    Academic,
}

#[derive(Clone, PartialEq, Props)]
struct ProjectProps {
    title: String,
    category: Option<CardCategory>,
    description: String,
    git: Option<GitLink>,
    imsrc: String,
}

// impl Clone for Card {
//     fn clone(&self) -> Self {
//         Self {
//             title: self.title.clone(),
//             category: self.category.clone(),
//             description: self.description.clone(),
//             git: self.git.clone(),
//             imsrc: self.imsrc.clone(),
//         }
//     }
// }

impl Default for ProjectProps {
    fn default() -> Self {
        Self {
            title: String::from("empty project"),
            category: None,
            description: String::from("empty description"),
            git: None,
            imsrc: String::from("https://www.placehold.co/400"),
        }
    }
}

fn project_add_button(cx: Scope<ProjectProps>) -> Element {
    render!(rsx! {
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
    })
}
fn project_image(cx: Scope<ProjectProps>) -> Element {
    render!(rsx! {
        div {
            class: "image dimmable",
            project_add_button(cx),
            img {
                alt: "card image broken",
                src: "{cx.props.imsrc}"
            }
        }
    })
}
fn project_content(cx: Scope<ProjectProps>) -> Element {
    render!(rsx! {
        div {
            class: "content",
            div {
                class: "header",
                "{cx.props.title}"
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
                "{cx.props.description}"
            }
        }
    })
}
fn project_extra_content(cx: Scope<ProjectProps>) -> Element {
    let content = match cx.props.git {
        Some(gitlink) => render!(rsx! {
            a {
                href: "{gitlink.url()}",
                class: "right floated created",
                "git"
            }
            a {
                class: "friends",
                "arbitrary"
            }
        }),
        None => render!(rsx! {
            a {
                class: "friends",
                "arbitrary"
            }
        }),
    };
    render!(rsx! {
        div {
            class: "extra content",
            content
        }
    })
}

fn render_project_card(cx: Scope<ProjectProps>) -> Element {
    render!(rsx! {
        div {
            class: "ui card",
            project_image(cx),
            project_content(cx),
            project_extra_content(cx)
        }
    })
}

impl ProjectProps {
    // fn new(
    //     title: String,
    //     category: Option<CardCategory>,
    //     description: String,
    //     git: Option<GitLink>,
    //     imsrc: Option<String>,
    // ) -> Self {
    //     match imsrc {
    //         Some(img_url) => Self {
    //             title,
    //             category,
    //             description,
    //             git,
    //             imsrc: img_url,
    //         },
    //         None => Self {
    //             title,
    //             category,
    //             description,
    //             git,
    //             ..Default::default()
    //         },
    //     }
    // }
    // fn set_image_source(&mut self, imsrc: String) -> &Self {
    //     self.imsrc = imsrc;
    //     self
    // }
    pub fn render(&self, cx: Scope<()>) -> Element {
        render!(rsx! {
            div {
                class: "ui card",
                self.image(cx),
                self.content(cx),
                self.extra_content(cx)
            }
        })
    }
}

struct CardList {
    active_category: Option<CardCategory>,
    pub data: Vec<ProjectProps>,
}

impl Clone for CardList {
    fn clone(&self) -> Self {
        let mut clone_data = Vec::with_capacity(self.data.len());
        clone_data.clone_from_slice(&self.data);
        Self {
            active_category: self.active_category,
            data: clone_data,
        }
    }
}

impl Default for CardList {
    fn default() -> Self {
        Self {
            active_category: None,
            data: vec![],
        }
    }
}

impl CardList {
    fn get_active(&mut self, cx: Scope<()>) -> Vec<ProjectProps> {
        match self.active_category {
            Some(active_category) => self
                .data
                .into_iter()
                .filter(|card| {
                    if let Some(category) = card.category {
                        category == active_category
                    } else {
                        false
                    }
                })
                .collect(),
            None => self.data,
        }
    }
    fn set_active_category(&mut self, cx: Scope<()>, category: CardCategory) {
        self.active_category = Some(category)
    }
    fn clear_active_category(&mut self, cx: Scope<()>) {
        self.active_category = None
    }
    fn push(&mut self, cx: Scope<()>, card: ProjectProps) {
        self.data.push(card)
    }
    pub fn render(&self, cx: Scope<()>) -> Element {
        render!(rsx! {
            div {
                div {
                    class: "ui four cards",
                    for card in self.get_active(cx) {
                        card.render(cx)
                    }
                }
            }
        })
    }
    pub fn placeholder(cx: Scope<()>) -> Self {
        let mut list = Self::default();
        list.push(
            cx,
            ProjectProps::new(
                "rustystack".to_string(),
                Some(CardCategory::Freetime),
                "A full stack consisting entirely of rust (and some html and css)".to_string(),
                Some(GitLink {
                    user: "gohermgo".to_string(),
                    repo: "rustystack".to_string(),
                }),
                None,
            ),
        );
        while list.data.len() < 4 {
            list.push(cx, ProjectProps::default());
        }
        list
    }
}

// fn cards(cx: Scope<()>) -> Element {
//     let placeholder_source = String::from("https://placehold.co/400");
//     cx.render(rsx! {
//         div {
//             // An outer div to keep the inner cards from touching the "edges"
//             // padding_left: "1em",
//             // padding_right: "1em",
//             div {
//                 class: "ui four cards",
//                 Card::add(cx, Card {
//                     title: String::from("rustystack"),
//                     description: String::from("A project consisting of a website, whose stack is entirely in rust (this was hardcoded into that stack)"),
//                     git: String::from("https://github.com/gohermgo/rustystack"),
//                     imsrc: placeholder_source.clone()
//                 })
//                 Card::add(cx, Card {
//                     title: String::from("project"),
//                     description: String::from("description"),
//                     git: String::from("localhost:8080"),
//                     imsrc: placeholder_source.clone()
//                 })
//                 Card::add(cx, Card {
//                     title: String::from("project"),
//                     description: String::from("description"),
//                     git: String::from("localhost:8080"),
//                     imsrc: placeholder_source.clone()
//                 })
//                 Card::add(cx, Card {
//                     title: String::from("project"),
//                     description: String::from("description"),
//                     git: String::from("localhost:8080"),
//                     imsrc: placeholder_source.clone()
//                 })
//             }
//         }
//     })
// }

#[derive(Clone)]
struct CardSection {
    pub card_list: CardList,
}

// impl Clone for CardSection {
//     fn clone(&self) -> Self {
//         Self {}
//     }
// }

impl Default for CardSection {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl CardSection {
    fn placeholder(cx: Scope<()>) -> Self {
        Self {
            card_list: CardList::placeholder(cx),
        }
    }
    fn filter_buttons(&self, cx: Scope<()>) -> Element {
        match self.card_list.active_category {
            None => render!(rsx! {
                div {
                    class: "ui link list",
                    div {
                        class: "active item",
                        "all"
                    }
                    a {
                        class: "item",
                        "freetime"
                    }
                    a {
                        class: "item",
                        "professional"
                    }
                    a {
                        class: "item",
                        "academic"
                    }
                }
            }),
            Some(CardCategory::Freetime) => render!(rsx! {
                div {
                    class: "ui link list",
                    a {
                        class: "item",
                        "all"
                    }
                    div {
                        class: "active item",
                        "freetime"
                    }
                    a {
                        class: "item",
                        "professional"
                    }
                    a {
                        class: "item",
                        "academic"
                    }
                }
            }),
            Some(CardCategory::Professional) => render!(rsx! {
                div {
                    class: "ui link list",
                    a {
                        class: "item",
                        "all"
                    }
                    a {
                        class: "item",
                        "freetime"
                    }
                    div {
                        class: "active item",
                        "professional"
                    }
                    a {
                        class: "item",
                        "academic"
                    }
                }
            }),
            Some(CardCategory::Academic) => render!(rsx! {
                div {
                    class: "ui link list",
                    a {
                        class: "item",
                        "all"
                    }
                    a {
                        class: "item",
                        "freetime"
                    }
                    a {
                        class: "item",
                        "professional"
                    }
                    div {
                        class: "active item",
                        "academic"
                    }
                }
            }),
        }
    }
    pub fn render(&self, cx: Scope<()>) -> Element {
        render!(rsx! {
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
                                self.filter_buttons(cx)
                            }
                        }
                    }
                    div {
                        class: "fourteen wide column",
                        self.card_list.clone().render(cx)
                    }
                }
            }
        })
    }
    pub fn render_placeholder(cx: Scope<()>) -> Element {
        let card_section = CardSection::placeholder(cx);
        card_section.clone().render(cx)
    }
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
    let card_section = CardSection::placeholder(cx);
    cx.render(rsx! {
        div {
            class: "ui grid",
            div {
                class: "row",
                introduction(cx),
            }
            div {
                class: "row",
                card_section.render(cx)
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

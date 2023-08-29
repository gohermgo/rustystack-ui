#![allow(non_snake_case)]
// use core::num::fmt::Formatted;

use std::fmt::{Display, Write};

use dioxus::prelude::*;
use rustystack_common::*;

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

trait Url {
    fn get_url(&self) -> String;
}

trait RenderBlock<P> {
    fn render(cx: Scope<P>) -> Element;
}

#[derive(Clone, Default, PartialEq, Props)]
struct GitLink {
    user: String,
    repo: String,
}

impl std::string::ToString for GitLink {
    fn to_string(&self) -> String {
        format!("https://www.github.com/{}/{}.git", self.user, self.repo)
    }
}

// impl Copy for GitLink {}

// impl Clone for GitLink {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

// impl PartialEq for GitLink {
//     fn eq(&self, other: &Self) -> bool {
//         self.user.eq(&other.user) && self.repo.eq(&other.repo)
//     }
//     fn ne(&self, other: &Self) -> bool {
//         self.user.ne(&other.user) || self.repo.ne(&other.repo)
//     }
// }

// impl Default for GitLink {
//     fn default() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }
// }

// impl std::string::ToString for GitLink {
//     fn to_string(&self) -> String {
//         String::from(self)
//     }
// }

// impl std::convert::From<GitLink> for String {
//     fn from(value: GitLink) -> Self {
//         format!("https://www.github.com/{}/{}.git", value.user, value.repo)
//     }
// }

// impl Url for GitLink {
//     fn get_url(&self) -> String {
//         String::from(*self)
//     }
// }

// impl std::convert::From<&GitLink> for String {
//     fn from(value: &GitLink) -> Self {
//         format!("https://www.github.com/{}/{}.git", *value.user, *value.repo)
//     }
// }

// impl std::fmt::Write for GitLink {
//     fn write_str(mut self: &mut Self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
//         String::from(self).write_str(args)
//     }
// }

// impl std::fmt::Display for GitLink {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         String::from(self).write_str(f)
//         // self.write_str(f)
//     }
// }

// impl RenderBlock<GitLink> for GitLink {
//     fn render(cx: Scope<) -> Element {
//         render!(rsx ! {
//             a {
//                 href: "{self.get_url()}",
//                 class: "right floated created",
//                 "git"
//             }
//         })
//     }
// }

fn git_element(cx: Scope<GitLink>) -> Element {
    render!(rsx! {
        a {
            href: "{cx.props.to_string()}",
            class: "right floated created",
            "git"
        }
    })
}

#[derive(Clone, PartialEq)]
enum CardCategory {
    Freetime,
    Professional,
    Academic,
}

impl std::convert::From<CardCategory> for String {
    fn from(value: CardCategory) -> Self {
        match value {
            CardCategory::Freetime => String::from("freetime"),
            CardCategory::Professional => String::from("professional"),
            CardCategory::Academic => String::from("academic"),
        }
    }
}

// impl std::fmt::Write for CardCategory {
//     fn write_str(&mut self, s: &str) -> std::fmt::Result {
//         String::from(*self).write_str()
//     }
// }

// impl std::fmt::Display for CardCategory {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.write_str()
//     }
// }

#[derive(Clone, PartialEq, Props)]
struct Image {
    #[props(default = String::from("https://www.placehold.co/400"))]
    source_url: String,
}

fn image_interaction_button(cx: Scope) -> Element {
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

#[inline_props]
fn project_image(cx: Scope, image_url: String) -> Element {
    render!(rsx! {
        div {
            class: "image dimmable",
            image_interaction_button {},
            img {
                alt: "card image broken",
                src: "{image_url}"
            }
        }
    })
}

// #[derive(Clone, PartialEq, Props)]
// struct ContentProps {
//     #[props(default = String::from("blank title"), into)]
//     title: String,
//     #[props(default = String::from("blank description"), into)]
//     content: String
// }

#[inline_props]
fn project_content(cx: Scope, title: String, description: String) -> Element {
    render!(rsx! {
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

#[derive(Clone, PartialEq, Props)]
struct ProjectProps {
    #[props(default = String::from("blank title"))]
    title: String,
    #[props(default = String::from("blank description"))]
    description: String,
    #[props(default = None)]
    category: Option<CardCategory>,
    #[props(default = None)]
    git: Option<GitLink>,
    #[props(default = String::from("https://www.placehold.co/400"))]
    image_url: String,
}
fn project_extra_content(cx: Scope<ProjectProps>) -> Element {
    match cx.props.git {
        Some(git) => render!(rsx! {
            div {
                class: "extra content",
                git_element {
                    user: git.user,
                    repo: git.repo,
                },
            }
        }),
        None => render!(rsx! {
            div {
                class: "extra content",
                a {
                    class: "friends",
                    "arbitrary"
                }
            }
        }),
    }
}

fn render_project_card(cx: Scope<ProjectProps>) -> Element {
    render!(rsx! {
        div {
            class: "ui card",
            project_image {
                image_url: cx.props.image_url,
            },
            project_content {
                title: cx.props.title,
                description: cx.props.description
            },
            project_extra_content {}
        }
    })
}

// impl PartialEq for Scope<ProjectProps> {
//     fn eq(&self, other: &Self) -> bool {
//         self.props == other.props
//     }
// }

// impl Clone for Vec<ProjectProps> {
//     fn clone(&self) -> Self {
//         self.into_iter().map(|x| x.clone()).collect()
//     }
// }

#[derive(Clone, PartialEq, Props)]
struct CardList {
    #[props(default = None)]
    pub active_category: Option<CardCategory>,
    #[props(default = vec![ProjectProps::builder().build()])]
    pub project_props: Vec<ProjectProps>,
}

#[derive(PartialEq, Props)]
struct FilterState {
    #[props(default = None)]
    pub active_category: Option<CardCategory>
}

fn _render_active_cards(cx: Scope<(FilterState, Vec<ProjectProps>)>) -> Element {
    cx.props.1.into_iter().filter(|project| {
        project.category == cx.props.0.active_category
    }).map(|active_project| {
        render!( rsx ! {
            render_project_card {               
                title: active_project.title,
                description: active_project.description,
                image_url: active_project.image_url
            }        
        })
    })
}

// impl PartialEq for CardList {
//     fn eq(&self, other: &Self) -> bool {
//         match self.project_props.len() == other.project_props.len() {
//             true => self
//                 .project_props
//                 .iter()
//                 .enumerate()
//                 .all(|(idx, val)| other.project_props[idx].props.eq(val.props)),
//             false => false,
//         }
//     }
//     fn ne(&self, other: &Self) -> bool {
//         match self.project_props.len() == other.project_props.len() {
//             true => self
//                 .project_props
//                 .iter()
//                 .enumerate()
//                 .any(|(idx, val)| other.project_props[idx].props.ne(val.props)),
//             false => true,
//         }
//     }
// }

// impl Clone for CardList {
//     fn clone(&self) -> Self {
//         let mut clone_data = Vec::with_capacity(self.project_props.len());
//         clone_data.clone_from_slice(&self.project_props);
//         Self {
//             active_category: self.active_category,
//             project_props: clone_data,
//         }
//     }
// }

// trait Renderable {
//     fn render_function(&self) -> F
//     where
//         F: std::convert::Into<Element>;
//     fn render(&self, cx: Scope<()>) -> Element;
// }

// impl RenderBlock for T {
//     fn render(&self) -> F where F: std::convert::Into<Element>,
//     {
//         |
//     }
// |}
// impl RenderBlock for CardList {
//     fn render_function(&self) -> F
//     where
//         F: std::convert::Into<Element>,
//     {
//     }
//     fn render(&self, cx: Scope<()>) -> Element {
//         render!(rsx! {
//             self.project_props
//         })
//     }
// }

#[derive(PartialEq, Props)]
struct ProjectFilterProps {}

fn get_active_cards(cx: Scope<(CardList, CardCategory)>) -> Vec<CardList> {
    cx
        .props
        .0
        .project_props
        .into_iter()
        .filter_map(|props| match props.category {
            Some(c) => todo!(),
            _ => None,
        })
        .collect();
}

fn render_active_cards(cx: Scope<CardList>) -> Element {
    let active_cards = 
    render!(rsx! {
        let active_cards = get_active_cards

        match cx.props.active_category {
            Some(category) => cx.props.project_props.data.for_each(|prop| {
                println!("hello")
            }),
            None => cx.props.project_props.iter().map(|x| render_project_card(dioxus::prelude::Scope::from(x)))
        }
    })
}
impl CardList {
    fn get_active(&mut self, cx: Scope<()>) -> Vec<ProjectProps> {
        match self.active_category {
            Some(active_category) => self
                .project_props
                .into_iter()
                .filter(|card| {
                    if let Some(category) = card.category {
                        category == active_category
                    } else {
                        false
                    }
                })
                .collect(),
            None => self.project_props,
        }
    }
    fn set_active_category(&mut self, cx: Scope<()>, category: CardCategory) {
        self.active_category = Some(category)
    }
    fn clear_active_category(&mut self, cx: Scope<()>) {
        self.active_category = None
    }
    fn push(&mut self, cx: Scope<()>, card: ProjectProps) {
        self.project_props.push(card)
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
        while list.project_props.len() < 4 {
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

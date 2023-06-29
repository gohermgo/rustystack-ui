use std::ops::Div;
use stylist::css;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;

mod contexts;

use contexts::{use_theme, ThemeKind, ThemeProvider};

pub const CSS_BUTTON_WRAPPER: &str = r#"
    color: white;
    height: 50px;
    width: 30px;
    font-size: 20px;
    background-color: rgb(88, 164, 255);
    border-radius: 5px;
    border: none;
"#;

#[styled_component]
pub fn Inside() -> Html {
    let theme = use_theme();
    let theme_str = match theme.kind() {
        ThemeKind::Dark => "Dark Theme",
        ThemeKind::Light => "Light Theme",
    };
    let other_theme = match theme.kind() {
        ThemeKind::Dark => ThemeKind::Light,
        ThemeKind::Light => ThemeKind::Dark,
    };
    let switch_theme = Callback::from(move |_| theme.set(other_theme.clone()));
    html! {
        <div>
            <button class={css!(CSS_BUTTON_WRAPPER)} onclick={switch_theme} id="yew-sample-button">
                {"Switch to "}{theme_str}
            </button>
        </div>
    }
}

// #[derive(Properties, PartialEq)]
// pub struct HeaderButtonProps {
//     pub content: Html,
// }
// pub const BUTTON_CSS_STRING: &str = r#"
//     display: inline-block;
//     border-style: solid;
//     border-width: 2px;
//     border-color: black;
//     border-radius: 0.5rem;
//     padding-top: 5px;
//     padding-bottom: 5px;
//     padding-right: 5px;
//     padding-left: 5px;
//     color: white;
//     font-family: Helvetica;
//     text-align: center;
//     letter-spacing: -0.025em;
//     background-color: black;
//     transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
//     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
//     transition-duration: 200ms;
//     &:hover {
//         border-style: double;
//         box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
//         transform: scaleX(1.1)
//     }
// "#;
// #[styled_component]
// pub fn HeaderButton(props: &HeaderButtonProps) -> Html {
//     html! {
//         <a class={css!(BUTTON_CSS_STR)} id="header-button">
//             {props.content}
//         </a>
//     }
// }

// #[styled_component]
// pub fn List() -> Html {
//     html! {
//         <ul class={css!(r#"
//             flex: 1 1 auto;
//             align-items: center;
//             margin-right: 8px;
//         "#)} id="header-list">
//             <li class={css!(BUTTON_CSS_STR)} id="header-list-item">
//                 <a href="youtube.com"> {"youtube"} </a>
//             </li>
//             <li class={css!(BUTTON_CSS_STR)} id="header-list-item">
//                 <a href="google.com"> {"google"} </a>
//             </li>
//         </ul>
//     }
// }

#[styled_component]
pub fn LinkComponent() -> Html {
    let button_component: Html = html! {<p1> {"Some button"} </p1/>};
    html! {
        <div class={css!(r#"
            flex: 1 1 auto;
            flex-wrap: wrap;
            align-items: center;   
        "#)}>
            <div class={css!(r#"
                display: inline-block;
                max-width: 28rem;
            "#)}>
                <div class={css!(r#"
                    width: auto;
                "#)}>
                    <List/>
                </div>
                <div class={css!(r#"
                    width: auto;
                "#)}>
                    <HeaderButton content={button_component} />
                </div>
            </div>
        </div>
    }
}

#[styled_component]
pub fn Header() -> Html {
    let logo_component: Html = html! {<h1> {"yoink"} </h1>};
    html! {

        <section class="bg-palbg">
            <div class="container px-4 mx-auto">
                <div class="flex items-center justify-between py-10">
                    {logo_component}
                    <div class={css!(r#"
                        width: auto;
                    "#)}> <LinkComponent /> </div>
                </div>
            </div>
        </section>
    }
}

pub struct HeaderStruct {}

impl HeaderStruct {
    pub fn button_css() -> &str { 
        r#"
            display: inline-block;
            border-style: solid;
            border-width: 2px;
            border-color: black;
            border-radius: 0.5rem;
            padding-top: 5px;
            padding-bottom: 5px;
            padding-right: 5px;
            padding-left: 5px;
            color: white;
            font-family: Helvetica;
            text-align: center;
            letter-spacing: -0.025em;
            background-color: black;
            transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 200ms;
            &:hover {
                border-style: double;
                box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
                transform: scaleX(1.1)
            }
        "#
    }
    pub fn button_component(content: Html) -> Html {
        let css = Self::button_css();
        html! {
            <a id="header-button">
                {content}
            </a>
        }
    }
    pub fn list_css() -> &str {
        r#"
            flex: 1 1 auto;
            align-items: center;
            margin-right: 8px;
        "#
    }
    pub fn list_component() -> Html {
        let link_css = Self::button_css();
        let list_css = Self::list_css();
        let div_css: &str = r#"
            width: auto;
        "#;
        html! {
            <div>
                <ul id="header-list">
                    <li id="header-list-item">
                        <a href="youtube.com"> {"youtube"} </a>
                    </li>
                    <li id="header-list-item">
                        <a href="google.com"> {"google"} </a>
                    </li>
                </ul>
            </div>
        }
    }
    pub fn logo() -> Html {
        // let style: &str = r#"
            
        // "#;
        let content: Html = html! {<h1> {"yoink"} </h1>};
        // let div_css: &str = r#"
        //     width: auto;
        // "#;
        html! {
            <div>
                {Self::button_component(content)}
            </div>
        }
    }
    pub fn button() -> Html {
        let content: Html = html! {<p1> {"Some button"} </p1/>};
        let div_css: &str = r#"
            width: auto;
        "#;
        html! {
            <div>
                {Self::button_component(content)}
            </div>
        }
    }
}

// Create main app that will load all other Components
pub struct App {}

impl App {
    fn view_header(&self, _ctx: &Context<Self>) -> Html {
        // fn header_button(button_content: Html) -> Html {
        //     html! {
        //         <div class="inline-block max-w-md">
        //             <a class="inline-block border-solid border-2 border-palfg px-5 py-3 text-palwhite font-semibold text-center tracking-tight bg-palacclight rounded-lg focus:ring-4 focus:ring-pal2 transition duration-200 ease-in-out hover:bg-palaccdark hover:border-pal2 hover:border-double hover:shadow-lg dark:hover:shadow-black/30 hover:scale-110 hover:underline">
        //                 {button_content}
        //             </a>
        //         </div>
        //     }
        // }
        // fn small_header_button(button_content: Html) -> Html {
        //     html! {
        //         <li class="px-3 py-1 border-solid border-2 border-palfg mr-10 font-medium text-md text-palwhite tracking-tight bg-palbg rounded-lg focus:ring-4 focus:ring-pal2 transition duration-200 ease-in-out hover:underline hover:bg-palaccdark hover:border-pal2 hover:border-double hover:shadow-lg dark:hover:shadow-black/30 hover:scale-105">
        //             {button_content}
        //         </li>
        //     }
        // }
        // let title_component: Html = header_button(html! {
        //     <h1 class="text-4xl font-arvo px-5"> {"yoink"} </h1>
        // });
        // let list_component: Html = html! {
        //     <ul class="xl:flex items-center mr-8">
        //         {
        //             small_header_button(html! {
        //                 <a href="https://www.youtube.com"> {"youtube"} </a>
        //             })
        //         }
        //         {
        //             small_header_button(html! {
        //                 <a href="https://www.google.com"> {"google"} </a>
        //             })
        //         }
        //     </ul>
        // };
        // // let button_component: Html = html! {<p1> {"Some button"} </p1/>};
        // let link_component: Html = html! {
        //     <div class="flex flex-wrap items-center">
        //         <div class="w-auto hidden lg:block"> {list_component} </div>
        //         <div class="w-auto hidden lg:block"> <HeaderButton payload={button_component}/> </div>
        //    </div>
        // };
        html! {
            <section class="bg-palbg">
                // <div class="container px-4 mx-auto">
                <div class={css!(r#"
                    padding-right: 4px;
                    padding-left: 4px;    
                "#)}>
                    // <div class="flex items-center justify-between py-10">
                    <div class={css!(r#"
                        padding-top: 10px;
                        padding-bottom: 10px;
                        align-items: center;
                        flex: auto;
                        justify-content: space-between;
                    "#)}>
                        <Header />
                    </div>
                </div>
            </section>
        }
    }
    fn view_footer(&self, _ctx: &Context<Self>) -> Html {
        fn wrap_button(button_content: Html, button_label: String) -> Html {
            html! {
                <button type="button" class="inline-flex flex-col items-center justify-center px-5 hover:bg-palfg group">
                    <svg class="w-6 h-6 mb-1 text-palwhite dark:text-gray-400 group-hover:text-palacclight" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                        {button_content}
                    </svg>
                    <span class="text-md text-palwhite dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500"> {button_label} </span>
                </button>
            }
        }
        html! {
            <div class="fixed bottom-0 left-0 z-50 w-full h-16 bg-palbg border-t border-palfg">
                <div class="grid h-full max-w-lg grid-cols-4 mx-auto font-medium">
                    {
                        wrap_button(html! {
                            <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z"></path>
                        }, String::from("button1"))
                    }
                    {
                        wrap_button(html! {
                            <>
                                <path d="M4 4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2H4z"></path>
                                <path clip-rule="evenodd" fill-rule="evenodd" d="M18 9H2v5a2 2 0 002 2h12a2 2 0 002-2V9zM4 13a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zm5-1a1 1 0 100 2h1a1 1 0 100-2H9z"></path>
                            </>
                        }, String::from("button2"))
                    }
                    {
                        wrap_button(html! {
                            <path d="M5 4a1 1 0 00-2 0v7.268a2 2 0 000 3.464V16a1 1 0 102 0v-1.268a2 2 0 000-3.464V4zM11 4a1 1 0 10-2 0v1.268a2 2 0 000 3.464V16a1 1 0 102 0V8.732a2 2 0 000-3.464V4zM16 3a1 1 0 011 1v7.268a2 2 0 010 3.464V16a1 1 0 11-2 0v-1.268a2 2 0 010-3.464V4a1 1 0 011-1z"></path>
                        }, String::from("button3"))
                    }
                    {
                        wrap_button(html! {
                            <path clip-rule="evenodd" fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zm-2 4a5 5 0 00-4.546 2.916A5.986 5.986 0 0010 16a5.986 5.986 0 004.546-2.084A5 5 0 0010 11z"></path>
                        }, String::from("button4"))
                    }
                </div>
            </div>
        }
    }
}

// Message enum that is used for managing the life cycle of Components
pub enum Msg {}

// Impl of Component interface
impl Component for App {
    type Message = Msg;
    type Properties = ();
    // Create a new App
    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        // Creates the HTML that will show up in the browser
        html! {
            <div>
                {self.view_header(&ctx)}
                <h1> {"Hello world"} </h1>
                // {self.view_footer(&ctx)}
            </div>
        }
    }
}

//#[function_component]
//fn header_component() -> Html {
//    html! {
//        <>
//            <h1> {"HEADER TITLE"} </h1>
//            <p> {"header text"} </p>
//        </>
//    }
//}

pub fn main() {
    // Create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    // Start the yew framework
    yew::start_app::<App>();
}

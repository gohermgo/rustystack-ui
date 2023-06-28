use yew::prelude::*;

// Create main app that will load all other Components
pub struct App {

}

impl App {
    fn view_header(&self, _ctx: &Context<Self>) -> Html {
        fn header_button(button_content: Html) -> Html {
            html! {
                <div class="inline-block max-w-md">
                    <a class="inline-block border-solid border-2 border-palfg px-5 py-3 text-palwhite font-semibold text-center tracking-tight bg-palacclight rounded-lg focus:ring-4 focus:ring-pal2 transition duration-200 ease-in-out hover:bg-palaccdark hover:border-pal2 hover:border-double hover:shadow-lg dark:hover:shadow-black/30 hover:scale-110 hover:underline">
                        {button_content}
                    </a>
                </div>
            }
        }
	fn small_header_button(button_content: Html) -> Html {
            html! {
                <li class="px-3 py-1 border-solid border-2 border-palfg mr-10 font-medium text-md text-palwhite tracking-tight bg-palbg rounded-lg focus:ring-4 focus:ring-pal2 transition duration-200 ease-in-out hover:underline hover:bg-palaccdark hover:border-pal2 hover:border-double hover:shadow-lg dark:hover:shadow-black/30 hover:scale-105">
                    {button_content}
                </li>
            }
        }
        let title_component: Html = header_button(html! {
            <h1 class="text-4xl font-arvo px-5"> {"yoink"} </h1>
        });
        let list_component: Html = html! {
            <ul class="xl:flex items-center mr-8">
                {
                    small_header_button(html! {
                        <a href="https://www.youtube.com"> {"youtube"} </a>
                    })
                }
                {
                    small_header_button(html! {
                        <a href="https://www.google.com"> {"google"} </a>
                    })
                }
            </ul>
        };
        let button_component: Html = header_button(html! {
            <p> {"some button"} </p>
        });
        let link_component: Html = html! {
            <div class="flex flex-wrap items-center">
                <div class="w-auto hidden lg:block"> {list_component} </div>
                <div class="w-auto hidden lg:block"> {button_component} </div>
           </div>
        };
        html! {
            <section class="bg-palbg">
                <div class="container px-4 mx-auto">
                    <div class="flex items-center justify-between py-10">
                        {title_component}
                        <div class="w-auto"> {link_component} </div>
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
pub enum Msg {

}

// Impl of Component interface
impl Component for App {
    type Message = Msg;
    type Properties = ();
    // Create a new App
    fn create(_ctx: &Context<Self>) -> Self {
        App {
        }
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
                {self.view_footer(&ctx)}
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

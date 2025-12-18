use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

mod components;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    let mut children = use_signal(String::new);
    rsx! {
        // Header
        div {
            id: "hero",
            div { id: "header-wrapper",
                img { 
                    src: HEADER_SVG, 
                    id: "header" 
                }
            }
        }

        div {
            display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "name" , "Name" }

            Input {
                oninput: move |e: FormEvent| children.set(e.value()),
                placeholder: "PV",
                value: children,
                class: "input",
            }
        }
    }
}

use crate::components::input::Input;
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
        Header {}
        Inputs {}
    }
}

#[component]
pub fn Header() -> Element {
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

        div { class: "container" }
    }
}

#[component]
pub fn Inputs() -> Element {
    let mut pv = use_signal(String::new);
    let mut r = use_signal(String::new);
    let mut fv = use_signal(String::new);
    let mut t = use_signal(String::new);

    rsx!{
        // Instrucciones
        div {"Usa los datos segun el credito que quieres conseguir!"}
        div { class: "small_container"}

        // Inputs
        div {
            display: "flex", flex_direction: "column", gap: ".5rem",
            Input {
                oninput: move |e: FormEvent| pv.set(e.value()),
                placeholder: "Cuota inicial",
                value: pv,
                class: "input",
            }

            Input {
                oninput: move |e: FormEvent| r.set(e.value()),
                placeholder: "TCEA", // Poner un component Hover Card checkearlo y tal vez elegir
                                     // entre TCEA TEA o TREA (explicar bien)
                value: r,
                class: "input",
            }

            Input {
                oninput: move |e: FormEvent| fv.set(e.value()),
                placeholder: "Monto solicitado",
                value: fv,
                class: "input",
            }

            Input {
                oninput: move |e: FormEvent| t.set(e.value()),
                placeholder: "Plazo", //Despues poner una opcion para elegir meses o anios
                value: t,
                class: "input",
            }
        }
    }
}

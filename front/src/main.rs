#![allow(non_snake_case)]
use dioxus::prelude::*;

mod components;
mod models;
use components::{Header, Footer, FilmModal};
use models::FilmModalVisibility;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    cx.render(rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }
            Footer {}
            FilmModal {
                on_create_or_update: move |_| {},
                on_cancel: move |_| {
                    is_modal_visible.write().0 = false;
                }
            }
        }   
    })
}

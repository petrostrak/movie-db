#![allow(non_snake_case)]
use dioxus::prelude::*;

mod components;
mod models;
use components::{Header, Footer, FilmModal, FilmCard};
use models::FilmModalVisibility;
use shared::models::Film;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    let selected_film = use_state::<Option<Film>>(cx, || None);
    let force_get_films = use_state(cx, || ());

    cx.render(rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            if let Some(films) = films.get() {
                rsx!(
                    ul {
                        class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
                        {films.iter().map(|film| {
                            rsx!(
                                FilmCard {
                                    key: "{film.id}",
                                    film: film,
                                    on_edit: move |_| {
                                        selected_film.set(Some(film.clone()));
                                        is_modal_visible.write().0 = true
                                    },
                                    on_delete: move |_| {}
                                }
                            )
                        })}
                    }
                )
            }
            Header {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }
            Footer {}
            FilmModal {
                film : selected_film.get().clone(),
                on_create_or_update: move |_| {},
                on_cancel: move |_| {
                    selected_film.set(None);
                    is_modal_visible.write().0 = false;
                }
            }
        }   
    })
}

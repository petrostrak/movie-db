use dioxus::prelude::*;

use crate::components::Button;
use crate::models::ButtonType;

#[derive(Props)]
pub struct FilmModalProps<'a> {
    on_create_or_update: EventHandler<'a, MouseEvent>,
    on_cancel: EventHandler<'a, MouseEvent>,
}

pub fn FilmModal<'a>(cx: Scope<'a, FilmModalProps>) -> Element<'a> {
    cx.render(rsx!(
        article {
            class: "z-50 w-full h-full fixed top-0 right-0 bg-gray-800 bg-opacity-50 flex flex-col justify-center items-center",
            section {
                class: "w-1/3 h-auto bg-white rounded-lg flex flex-col justify-center items-center box-border p-6",
                header {
                    class: "mb-4",
                    h2 {
                        class: "text-xl text-teal-950 font-semibold",
                        "🎬 Film"
                    }
                }
                form {
                    class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Title"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film title",
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Director"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film director",
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Year"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "number",
                            placeholder: "Enter film year",
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Poster"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film poster URL",
                        }
                    }
                }
                footer {
                    class: "flex flex-row justify-center items-center mt-4 gap-x-2",
                    Button {
                        button_type: ButtonType::Secondary,
                        onclick: move |evt| {
                            cx.props.on_cancel.call(evt)
                        },
                        "Cancel"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |evt| {
                            cx.props.on_create_or_update.call(evt);
                        },
                        "Save film"
                    }
                }
            }

        }
    ))
}

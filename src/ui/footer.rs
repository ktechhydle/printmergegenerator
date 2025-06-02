use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx!(
        div { class: "footer",
            br {}
            i { "Copyright (©) MP Software Foundation" }
        }
    )
}

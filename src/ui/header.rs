use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx!(
        div { class: "header",
            div {
                img { src: asset!("assets/icons/printmergegenerator_icon.svg"), width: "125" }
            }
            div {
                h1 { "PrintMergeGenerator" }
                p { "Easily generate print merge files for Corel Draw and InDesign" }
            }
        }
        hr {  }
    )
}

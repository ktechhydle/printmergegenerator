use dioxus::prelude::*;
use print_merge_generator::Footer;
use print_merge_generator::Header;
use print_merge_generator::MainContent;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "PrintMergeGenerator" }
        document::Link { rel: "icon", href: asset!("/assets/icons/printmergegenerator_icon.svg") }
        document::Link { rel: "stylesheet", href: asset!("/assets/stylesheets/style_main.css") }
        Header {  }
        MainContent {  }
        Footer {  }
    }
}

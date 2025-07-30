use crate::framework::generator::generate_numbers;
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn MainContent() -> Element {
    let filename = use_signal(|| "output.txt".to_string());
    let start = use_signal(|| 1);
    let end = use_signal(|| 100);
    let repeat = use_signal(|| 1);
    let prefix = use_signal(|| "No.".to_string());
    let vertical = use_signal(|| false);
    let aligned = use_signal(|| false);
    let mut output = use_signal(|| String::new());
    let mut error_message = use_signal(|| None::<String>);

    let _ = use_resource(move || {
        let prefix = prefix();
        let repeat = repeat();
        let start = start();
        let end = end();
        let vertical = vertical();
        let aligned = aligned();

        async move {
            output.set("Processing...".to_owned());

            gloo_timers::future::sleep(Duration::from_millis(200)).await;

            match generate_numbers(&prefix, repeat, start, end, vertical, aligned).await {
                Ok(result) => {
                    error_message.set(None);
                    output.set(result);
                }
                Err(e) => {
                    error_message.set(Some(e));
                    output.set("...".to_owned());
                }
            }
        }
    });

    rsx!(
        div {
            class: "main-content",

            if let Some(error) = &*error_message.read() {
                dialog { open: "true",
                    p {
                        "{error}"
                    }
                    button { onclick: move |_| {
                        error_message.set(None);
                    }, "Ok" }
                }
            }

            div {
                style: "flex: 1;",
                Controls {
                    filename,
                    start,
                    end,
                    repeat,
                    prefix,
                    vertical,
                    aligned,
                }
            }
            div {
                style: "flex: 1;",
                Preview {
                    filename,
                    output
                }
            }
        }
    )
}

#[component]
fn Controls(
    filename: Signal<String>,
    start: Signal<usize>,
    end: Signal<usize>,
    repeat: Signal<usize>,
    prefix: Signal<String>,
    vertical: Signal<bool>,
    aligned: Signal<bool>,
) -> Element {
    rsx!(
        div { flex: "1",
            p { "Filename (With Extension)" }
            input {
                value: "{filename}",
                oninput: move |e| {
                    filename.set(e.value());
                }
            }
            p { "Start Number" }
            input {
                r#type: "number",
                value: "{start}",
                min: "0",
                oninput: move |e| {
                    let raw = e.value();

                    if let Ok(parsed) = raw.parse() {
                        start.set(parsed);
                    }
                }
            }
            p { "End Number" }
            input {
                r#type: "number",
                value: "{end}",
                min: "1",
                oninput: move |e| {
                    let raw = e.value();
                    if let Ok(parsed) = raw.parse() {
                        end.set(parsed);
                    }
                }
            }
            p { "Repeating Amount" }
            input {
                r#type: "number",
                value: "{repeat}",
                min: "1",
                oninput: move |e| {
                    let raw = e.value();
                    if let Ok(parsed) = raw.parse() {
                        repeat.set(parsed);
                    }
                }
            }
            p { "Prefix" }
            input {
                value: "{prefix}",
                oninput: move |e| prefix.set(e.value())
            }
            div { class: "checkbox-container",
                input {
                    id: "count-numbers-vertically",
                    r#type: "checkbox",
                    checked: *vertical.read(),
                    onchange: move |e| vertical.set(e.checked()),
                }
                label { r#for: "count-numbers-vertically", "Count Numbers Vertically" }
            }
            div { class: "checkbox-container",
                input {
                    id: "number-spots",
                    r#type: "checkbox",
                    checked: *aligned.read(),
                    onchange: move |e| aligned.set(e.checked()),
                },
                label { r#for: "number-spots", "Number Spots" }
            }
        }
    )
}

#[component]
fn Preview(filename: Signal<String>, output: Signal<String>) -> Element {
    let download_file = move |_| {
        let content = output();
        let escaped_content = content.replace('\n', "\\n").replace('\"', "\\\"");
        let js = format!(
            r#"
            const blob = new Blob(["{}"], {{ type: "text/plain" }});
            const url = URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = "{}";
            a.click();
            URL.revokeObjectURL(url);
            "#,
            escaped_content,
            filename()
        );

        document::eval(&js);
    };

    rsx!(
        div { flex: "1",
            p { "Preview" }
            textarea {
                readonly: true,
                value: "{output}"
            }
            button {
               onclick: download_file,
               "Download File"
            }
        }
    )
}

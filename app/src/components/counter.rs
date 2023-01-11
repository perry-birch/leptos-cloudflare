use leptos::*;
use web_sys::console;

/// A simple counter component.
/// 
/// You can use doc comments like this to document your component.
#[component]
pub fn Counter(
    cx: Scope,
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    view! { cx,
        <div>
            <button on:click=move |_| set_value(0)>"Clear"</button>
            <br /><button on:click=move |_| set_value.update(|value| {
                *value -= step;
                console::log_2(&"Dec".into(), &value.to_string().into());
            })>"-"{step}</button>
            <br /><span>"Value #1: [" {value} "]!"</span>
            <br /><span>"Value #2: [" {value()} "]!"</span>
            <br /><span>"Value #3: [" {move || value()} "]!"</span>
            <br /><span>"Value #4: [" {move || value().to_string()} "]!"</span>
            <br /><button on:click=move |_| set_value.update(|value| {
                *value += step;
                console::log_2(&"Inc".into(), &value.to_string().into());
            })>"+"{step}</button>
        </div>
    }
}

use app::*;
use leptos::*;

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    // Verify wasm hydrate binding was called on the client
    console::log_1(&"Preparing to mount client...".into());

    mount_to_body(|cx| {
        view! { cx,  <Counter initial_value=1 step=3 /> }
    });

    // Alternate syntax but maybe from pre 0.1?
    // leptos::hydrate(body().unwrap(), move |cx| {
    //     view! { cx, <Counter initial_value=1 step=3 /> }
    // })
}

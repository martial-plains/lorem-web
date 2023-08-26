use leptos::*;
use lipsum::lipsum_with_rng;
use web_sys::Event;

#[component]
fn Lorem(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 15);
    let (text, set_text) = create_signal(cx, String::new());

    let onchange_number = move |e: Event| {
        let result = event_target_value(&e);
        set_count.set(result.parse().unwrap());
    };

    create_effect(cx, move |_| {
        let lorem = lipsum_with_rng(rand::thread_rng(), count.get());
        set_text.set(lorem);
    });

    view! { cx,
        <div class={"flex h-screen flex-col"}>
            <div>
                <input type="number" id="number" name="number" value={count.get()} min="0" max="50000" on:change=onchange_number />
            </div>

            <div>
                {move || text.get()}
            </div>
        </div>
    }
}

/// entry point
fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,  <Lorem/> })
}

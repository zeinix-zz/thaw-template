use leptos::prelude::*;
use thaw::*;

/// A parameterized incrementing button
#[component]
pub fn CounterBtn(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <Button on_click=move |_| {
            set_count(count() + increment)
        }>

            "Click me: " {count}
        </Button>
    }
}

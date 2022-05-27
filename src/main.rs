use gloo::timers::future::sleep;
use std::time::Duration;
use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

#[function_component]
fn Nested() -> Html {
    let is_div = use_state(|| true);

    {
        let is_div = is_div.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    sleep(Duration::from_secs(1)).await;

                    is_div.set(false);
                });

                || {}
            },
            (),
        );
    }

    if *is_div {
        html! { <div>{"<div>Top</div>"}</div> }
    } else {
        html! { <span>{"<span>Top</span>"}</span> }
    }
}

#[function_component]
fn Top() -> Html {
    html! { <Nested /> }
}

#[function_component]
fn App() -> Html {
    let show_bottom = use_state_eq(|| false);

    {
        let show_bottom = show_bottom.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    sleep(Duration::ZERO).await;

                    show_bottom.set(true);
                });

                || {}
            },
            (),
        );
    }

    html! {
        <>
            <Top />
            if *show_bottom {
                <div>{"<div>Bottom</div>"}</div>
            } else {
                <></>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::default().render();
}

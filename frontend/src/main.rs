use yew::prelude::*;

use crate::component::loader::Loader;

mod component;
mod config;

#[function_component]
pub fn App() -> Html {
    html! {
        <main class="w-screen h-screen bg-slate-100">
            <Suspense fallback={html! { <Loader /> }}>
                <Content />
            </Suspense>
        </main>
    }
}

#[function_component]
pub fn Content() -> HtmlResult {
    let config = config::use_config()?;

    Ok(html! {
        <>
            <p class="font-semibold">{ "Hello axum + yew!" }</p>
            <p>{ config.header.clone() }</p>
        </>
    })
}

fn main() {
    yew::Renderer::<App>::new().render();
}

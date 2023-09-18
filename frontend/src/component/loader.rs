use yew::prelude::*;

#[function_component]
pub fn Loader() -> Html {
    html! {
        <div role="status" class="grid h-screen place-items-center">
            <div>
                <div class="animated-loader text-center select-none"/>
                <div> {"Loading..."} </div>
            </div>
        </div>
    }
}

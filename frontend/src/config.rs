use yew::{
    prelude::*,
    suspense::{Suspension, SuspensionResult},
};

#[hook]
pub fn use_config() -> SuspensionResult<common::config::Config> {
    let state: UseStateHandle<Option<common::config::Config>> = use_state(|| None);

    match state.as_ref() {
        Some(value) => Ok(value.clone()),
        None => {
            let suspention = Suspension::from_future({
                let state = state.clone();
                async move {
                    state.set(Some(fetch_config().await));
                }
            });
            Err(suspention)
        }
    }
}

async fn fetch_config() -> common::config::Config {
    let host = gloo_utils::window().location().origin().unwrap();
    reqwest::get(format!("{host}{}", common::url::CONFIG))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

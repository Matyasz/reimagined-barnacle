use gloo::{console::log, dialogs::alert};
use reqwasm::http::Request;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlFormElement};
use yew::{function_component, html, use_node_ref, Callback};
use yew_router::{history::History, hooks::use_history};

use crate::{
    models::credentials::LoginCredentials,
    routes::AppRoute,
    utilities::form_utils::{validate_form, ValidatedStruct},
};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let form_ref = use_node_ref();

    let onsubmit = {
        let form_ref = form_ref.clone();
        let history = use_history().unwrap();

        Callback::once(move |e: FocusEvent| {
            e.prevent_default();
            let form_element = form_ref.cast::<HtmlFormElement>();
            let vcreds: ValidatedStruct<LoginCredentials> = validate_form(form_element);

            let mut valid_data = true;

            if !vcreds.alerts.is_empty() {
                valid_data = false;

                let mut msg = "".to_owned();
                for alrt in &vcreds.alerts {
                    msg.push_str(&alrt);
                    msg.push_str("\n");
                }

                alert(&msg);
            }

            if valid_data {
                gloo::console::log!(&vcreds.data.email);

                let c = json!(vcreds.data.clone()).to_string();
                log!(&c);
                spawn_local(async move {
                    let resp = Request::post("http://localhost:3000/login")
                        .body(c)
                        .send()
                        .await
                        .unwrap();

                    let x = resp.text().await.unwrap();
                    log!(x);

                    history.push(AppRoute::Home);
                });
            }
        })
    };

    html! {
        <form class="credentials-box" content_type="x-www-form-urlencoded" action="" method="POST" ref={ form_ref } { onsubmit }>
            <div class="credentials-header">
                <h1>{ "login" }</h1>
            </div>

            <div class="credentials-form">
                <div class="credentials-input">
                    <input class="credentials-text" type="email" name="email" placeholder="email"/>
                </div>
                <div class="credentials-input">
                    <input class="credentials-text" type="password" name="password" placeholder="password"/>
                </div>
            </div>
            <div class="credentials-footer">
                <span> { "" } </span>
                <input class="credentials-button" type="submit" value="submit"/>
            </div>
        </form>
    }
}

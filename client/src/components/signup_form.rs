use gloo::{console::log, dialogs::alert};
use reqwasm::http::Request;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlFormElement};
use yew::{function_component, html, use_node_ref, Callback};

use crate::{
    models::credentials::SignupCredentials,
    utilities::form_utils::{validate_form, ValidatedStruct},
};

#[function_component(SignupForm)]
pub fn signup_form() -> Html {
    let user_ref = use_node_ref();

    let onsubmit = {
        let user_ref = user_ref.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let form_element = user_ref.cast::<HtmlFormElement>();
            let vcreds: ValidatedStruct<SignupCredentials> = validate_form(form_element);

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

            if vcreds.data.password != vcreds.data.passconf {
                valid_data = false;
                alert("Passwords do not match");
            }

            if valid_data {
                gloo::console::log!(&vcreds.data.name);

                let c = json!(vcreds.data.clone()).to_string();
                log!(&c);
                spawn_local(async move {
                    let resp = Request::post("http://localhost:3000/signup")
                        .body(c)
                        .send()
                        .await
                        .unwrap();

                    let x = resp.text().await.unwrap();
                    log!(x);
                });
            }
        })
    };

    html! {
        <form class="credentials-box" content_type="x-www-form-urlencoded" ref={ user_ref } { onsubmit }>
            <div class="credentials-header">
                <h1>{ "create account" }</h1>
            </div>

            <div class="credentials-form">
                <div class="credentials-input">
                    <input class="credentials-text" type="email" name="email" placeholder="email" />
                </div>
                <div class="credentials-input">
                    <input class="credentials-text" type="text" name="name" placeholder="name" />
                </div>

                <div class="credentials-input">
                    <input class="credentials-text" type="password" name="password" placeholder="password" />
                </div>
                <div class="credentials-input">
                    <input class="credentials-text" type="password" name="passconf" placeholder="confirm password" />
                </div>
            </div>

            <div class="credentials-footer">
                <span> { "" } </span>
                <input class="credentials-button" type="submit" value="submit" />
            </div>
        </form>
    }
}

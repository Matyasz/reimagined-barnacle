use gloo::dialogs::alert;
// use reqwasm::http::Request;
// use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlFormElement};
use yew::{function_component, html, use_node_ref, Callback};

use crate::{
    models::credentials::SignupCredentials,
    utilities::form_utils::{validate_form, ValidatedStruct},
};

#[function_component(SignupForm)]
pub fn signup_form() -> Html {
    let user_ref = use_node_ref();

    // wasm_bindgen_futures::spawn_local(async move {
    //     let req = Request::post("127.0.0.1:3000/signup");
    //     req.body(Creds {
    //         username: "u".to_string(),
    //     });

    //     let res = req.send().await;
    // });

    let onsubmit = {
        let user_ref = user_ref.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let form_element = user_ref.cast::<HtmlFormElement>();
            let vcreds: ValidatedStruct<SignupCredentials> = validate_form(form_element);

            if !vcreds.alerts.is_empty() {
                let mut msg = "".to_owned();
                for alrt in vcreds.alerts {
                    msg.push_str(&alrt);
                    msg.push_str("\n");
                }

                alert(&msg);
            } else {
                gloo::console::log!(vcreds.data.name);
                // spawn_local(async move {
                //     let creds = SignupCredentials {
                //         email: email.clone(),
                //         name: name.clone(),
                //         password: password.clone(),
                //         passconf: passconf.clone(),
                //     };
                // let resp = Request::post("127.0.0.1:3000/signup").send().await.unwrap();
                // });
            }
        })
    };

    html! {
        <form class="credentials-box" ref={ user_ref } { onsubmit }>
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

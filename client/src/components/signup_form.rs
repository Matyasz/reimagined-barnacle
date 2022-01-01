use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlFormElement};
use yew::{function_component, html, use_node_ref, Callback};

use crate::utilities::form_utils::process_form_field;

pub struct SignupCredentials {
    pub email: String,
    pub name: String,
    pub password: String,
    pub passconf: String,
}

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
            let user_input = user_ref.cast::<HtmlFormElement>();

            let x = user_input
                .unwrap()
                .get_elements_by_class_name("credentials-text");

            let email = process_form_field(&x, "email");
            let name = process_form_field(&x, "name");
            let password = process_form_field(&x, "password");
            let passconf = process_form_field(&x, "confirm-password");

            let creds = SignupCredentials {
                email: email.clone(),
                name: name.clone(),
                password: password.clone(),
                passconf: passconf.clone(),
            };

            // spawn_local(async move {
            //     let resp = Request::post("/path").send().await.unwrap();
            // });
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
                    <input class="credentials-text" type="password" name="confirm-password" placeholder="confirm password" />
                </div>
            </div>

            <div class="credentials-footer">
                <span> { "" } </span>
                <input class="credentials-button" type="submit" value="submit" />
            </div>
        </form>
    }
}

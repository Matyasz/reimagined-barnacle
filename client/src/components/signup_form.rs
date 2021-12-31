use gloo::console::log;
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlInputElement, HtmlFormElement};
use yew::{function_component, html, use_node_ref, Callback};

pub struct Creds {
    pub name: String,
    pub pass: String,
}

#[function_component(SignupForm)]
pub fn signup_form() -> Html {
    let email_ref = use_node_ref();
    let name_ref = use_node_ref();
    let pass_ref = use_node_ref();
    let passconf_ref = use_node_ref();

    // wasm_bindgen_futures::spawn_local(async move {
    //     let req = Request::post("127.0.0.1:3000/signup");
    //     req.body(Creds {
    //         username: "u".to_string(),
    //     });

    //     let res = req.send().await;
    // });

    let onsubmit = {
        let email_ref = email_ref.clone();
        let name_ref = name_ref.clone();
        let pass_ref = pass_ref.clone();
        let passconf_ref = passconf_ref.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let email_input = email_ref.cast::<HtmlInputElement>();
            let name_input = name_ref.cast::<HtmlInputElement>();
            let pass_input = pass_ref.cast::<HtmlInputElement>();
            let passconf_input = passconf_ref.cast::<HtmlInputElement>();

            let email = email_input.unwrap().value();
            let name = name_input.unwrap().value();
            let password = pass_input.unwrap().value();
            let passconf = passconf_input.unwrap().value();

            log!(JsValue::from(&email));
            log!(JsValue::from(&name));
            log!(JsValue::from(&password));
            log!(JsValue::from(&passconf));

            let is_invalid = email.is_empty();
            if is_invalid {
                return;
            }

            // spawn_local(async move {
            //     let resp = Request::post("/path").send().await.unwrap();
            // });
        })
    };

    html! {
        <form class="credentials-box" { onsubmit }>
            <div class="credentials-header">
                <h1>{ "create account" }</h1>
            </div>

            <div class="credentials-form">
                <div class="credentials-input">
                    <input ref={ email_ref } class="credentials-text" type="email" name="email" id="email" placeholder="email" />
                </div>
                <div class="credentials-input">
                    <input ref={ name_ref } class="credentials-text" type="text" name="name" id="name" placeholder="name" />
                </div>

                <div class="credentials-input">
                    <input ref={ pass_ref } class="credentials-text" type="password" name="password" placeholder="password" />
                </div>
                <div class="credentials-input">
                    <input ref={ passconf_ref } class="credentials-text" type="password" name="confirm-password" placeholder="confirm password" />
                </div>
            </div>

            <div class="credentials-footer">
                <span> { "" } </span>
                <input class="credentials-button" type="submit" value="submit" />
            </div>
        </form>
    }
}

// impl Component for SignupForm {
//     type Message = SignupFormMsg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         // let req = Request::post("127.0.0.1:3000/signup");

//         // req.body({"username": 'u'});

//         // .await
//         // .unwrap()
//         // .json().;

//         false
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <form class="credentials-box" action="" method="POST">
//                 <div class="credentials-header">
//                     <h1>{ "create account" }</h1>
//                 </div>

//                 <div class="credentials-form">
//                     <div class="credentials-input">
//                         <input class="credentials-text" type="email" name="email" placeholder="email"/>
//                     </div>
//                     <div class="credentials-input">
//                         <input class="credentials-text" type="text" name="name" placeholder="name"/>
//                     </div>

//                     <div class="credentials-input">
//                         <input class="credentials-text" type="password" name="password" placeholder="password"/>
//                     </div>
//                     <div class="credentials-input">
//                         <input class="credentials-text" type="password" name="confirm-password" placeholder="confirm password" />
//                     </div>
//                 </div>

//                 <div class="credentials-footer">
//                     <span> { "" } </span>
//                     <input class="credentials-button" type="submit" value="submit" />
//                 </div>
//             </form>
//         }
//     }
// }

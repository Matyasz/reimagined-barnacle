use crate::client::components::navbar::NavBar;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum SignupMsg {}

pub struct SignupPage {
    link: ComponentLink<Self>,
}

impl Component for SignupPage {
    type Message = SignupMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <NavBar/>

                <form class="credentials-box" action="" method="POST">
                    <div>
                        <div>
                            //<label for="username">{ "Username:" }</label>
                            <input class="credentials-input" type="text" name="username" placeholder="username"/>
                        </div>
                        <div>
                            //<label for="password">{ "Password:" }</label>
                            <input class="credentials-input" type="password" name="password" placeholder="password"/>
                        </div>
                        <div>
                            //<label for="confirm-password">{ "Confirm Password:" }</label>
                            <input class="credentials-input" type="confirm-password" name="confirm-password" placeholder="confirm password" />
                        </div>
                        <input class="credentials-button" type="submit" value="Login"/>
                    </div>
                </form>
            </div>
        }
    }
}

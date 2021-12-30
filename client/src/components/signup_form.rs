use yew::{html, Component, Context, Html};
// use yew::prelude::*;

pub enum SignupFormMsg {}

pub struct SignupForm {}

impl Component for SignupForm {
    type Message = SignupFormMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <form class="credentials-box" action="" method="POST">
                <div class="credentials-header">
                    <h1>{ "create account" }</h1>
                </div>

                <div class="credentials-form">
                    <div class="credentials-input">
                        <input class="credentials-text" type="email" name="email" placeholder="email"/>
                    </div>
                    <div class="credentials-input">
                        <input class="credentials-text" type="text" name="name" placeholder="name"/>
                    </div>

                    <div class="credentials-input">
                        <input class="credentials-text" type="password" name="password" placeholder="password"/>
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
}

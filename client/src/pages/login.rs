use crate::components::login_form::LoginForm;

use yew::{html, Component, Context, Html};

pub enum LoginMsg {}

pub struct LoginPage {}

impl Component for LoginPage {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <LoginForm />
            </div>
        }
    }
}

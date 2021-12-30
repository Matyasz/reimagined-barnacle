use crate::routes::AppRoute;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

pub enum LoginMsg {}

pub struct NavBar {}

impl Component for NavBar {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="header">
                <Link<AppRoute> to={AppRoute::Login}>
                    <div class="header-button">
                        <h5>{ "login" }</h5>
                    </div>
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Signup}>
                    <div class="header-button">
                        <h5>{ "signup" }</h5>
                    </div>
                </Link<AppRoute>>
            </div>
        }
    }
}

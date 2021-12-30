use crate::components::navbar::NavBar;
use crate::pages::{login::LoginPage, signup::SignupPage};

use crate::routes::AppRoute;

use yew::html::Scope;
use yew::prelude::*; //{html, Component, Context, Html};
use yew_router::prelude::*;

pub enum AppMsg {}
pub struct AppContainer {
    // logged_in: bool,
}

impl Component for AppContainer {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<AppRoute> render={ Switch::render(switch) } />
                </main>
            </BrowserRouter>
        }
    }
}

impl AppContainer {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            < NavBar />
        }
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { "Home" },
        AppRoute::Login => html! { < LoginPage /> },
        AppRoute::Signup => html! { < SignupPage /> },
        AppRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

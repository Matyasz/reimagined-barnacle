use crate::client::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;

pub enum LoginMsg {
    Login,
}

pub struct NavBar {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    message: String,
    logged_in: bool,
}

impl Component for NavBar {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            message: String::from(""),
            logged_in: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginMsg::Login => {
                if !self.logged_in {
                    self.message = String::from("Howdy!");
                    self.logged_in = true;
                } else {
                    self.message = String::from("");
                    self.logged_in = false;
                }
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            //<div class="header">
            //    { &self.message }
            //    <button onclick=self.link.callback(|_| LoginMsg::Login)>{ "login" }</button>
            //</div>
            <div style="justify-content: center">
                <RouterAnchor<AppRoute> route=AppRoute::Hi>
                    <div class="header">
                        { &self.message }
                        <button onclick=self.link.callback(|_| LoginMsg::Login)>{ "login" }</button>
                    </div>
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}

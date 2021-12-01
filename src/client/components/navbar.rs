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
}

impl Component for NavBar {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="header">
                <div class="button">
                    <RouterAnchor<AppRoute> route=AppRoute::Login>
                        //<button onclick=self.link.callback(|_| LoginMsg::Login)>{ "login" }</button>
                        { "login" }
                    </RouterAnchor<AppRoute>>
                </div>
                <div class="button">
                    <RouterAnchor<AppRoute> route=AppRoute::Signup>
                        //<button onclick=self.link.callback(|_| LoginMsg::Login)>{ "login" }</button>
                        { "signup" }
                    </RouterAnchor<AppRoute>>
                </div>
            </div>
        }
    }
}

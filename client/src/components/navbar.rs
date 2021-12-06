use crate::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;

pub enum LoginMsg {}

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
                <RouterAnchor<AppRoute> route=AppRoute::Login>
                    <div class="header-button">
                        //<button onclick=self.link.callback(|_| LoginMsg::Login)>{ "login" }</button>
                        <h5>{ "login" }</h5>
                    </div>
                </RouterAnchor<AppRoute>>
                <RouterAnchor<AppRoute> route=AppRoute::Signup>
                    <div class="header-button">
                        //<button onclick=self.link.callback(|_| LoginMsg::Login)>{ "login" }</button>
                        <h5>{ "signup" }</h5>
                    </div>
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}

use crate::client::components::navbar::NavBar;
use crate::client::components::navred::NavBarRed;

use crate::client::routes::AppRoute;

use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, ShouldRender};
use yew_router::{
    prelude::{Route, RouteAgent, RouteService},
    Switch,
};

pub enum AppMsg {
    RouteChanged(Route<()>),
}

pub struct AppContainer {
    link: ComponentLink<Self>,
    route_service: RouteService<()>,
    route: Route<()>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

impl Component for AppContainer {
    type Message = AppMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(AppMsg::RouteChanged));
        let route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();

        Self {
            link,
            route_service,
            route,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::RouteChanged(route) => {
                self.route_service.set_route(&route.route, ());
                self.route = route;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match AppRoute::switch(self.route.clone()) {
            Some(AppRoute::Home) => html! { <NavBar/> },
            Some(AppRoute::Hi) => html! { <NavBarRed/> },
            None => html! {"oh no"},
        }
    }
}

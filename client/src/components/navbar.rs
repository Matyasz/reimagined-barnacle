use crate::routes::AppRoute;
use yew::{function_component, html};
use yew_router::prelude::*;

// #[derive(Properties, PartialEq)]
// pub struct NavBarProps {
//     pub logged_in: bool,
// }

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <div class="header">
            // if props.logged_in {
            //     <b> { "howdy pard" } </b>
            // }
            <b> { "howdy pard" } </b>

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

// impl Component for NavBar {
//     type Message = LoginMsg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         true
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <div class="header">
//                 <Link<AppRoute> to={AppRoute::Login}>
//                     <div class="header-button">
//                         <h5>{ "login" }</h5>
//                     </div>
//                 </Link<AppRoute>>
//                 <Link<AppRoute> to={AppRoute::Signup}>
//                     <div class="header-button">
//                         <h5>{ "signup" }</h5>
//                     </div>
//                 </Link<AppRoute>>
//             </div>
//         }
//     }
// }

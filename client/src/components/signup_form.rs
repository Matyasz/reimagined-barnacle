use yew::{
    html,
    services::fetch::{FetchTask, Request},
    Component, ComponentLink, Html, ShouldRender,
};
use yewtil::fetch::Fetch;

pub enum SignupFormMsg {}

pub struct SignupForm {
    link: ComponentLink<Self>,
    api: Fetch<Request<Vec<i32>>, Vec<i32>>,
    fetch_task: Option<FetchTask>,
}

impl Component for SignupForm {
    type Message = SignupFormMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            api: Default::default(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <form class="credentials-box" action="" method="POST">
                <div class="credentials-header">
                    <h1>{ "Create Account" }</h1>
                </div>

                <div class="credentials-form">
                    <div class="credentials-input">
                        <input class="credentials-text" type="text" name="username" placeholder="username"/>
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
                    <input class="credentials-button" type="submit" value="submit"/>
                </div>
            </form>
        }
    }
}

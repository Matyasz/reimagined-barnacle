use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum LoginFormMsg {}

pub struct LoginForm {
    link: ComponentLink<Self>,
}

impl Component for LoginForm {
    type Message = LoginFormMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
                    <h1>{ "Login" }</h1>
                </div>

                <div>
                    <div class="credentials-input">
                        <input class="credentials-text" type="text" name="username" placeholder="username"/>
                    </div>
                    <div class="credentials-input">
                        <input class="credentials-text" type="password" name="password" placeholder="password"/>
                    </div>

                    <div class="credentials-footer">
                        <span> { "" } </span>
                        <input class="credentials-button" type="submit" value="Submit"/>
                    </div>
                </div>
            </form>
        }
    }
}

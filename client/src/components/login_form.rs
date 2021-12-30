use yew::{html, Component, Context, Html};

pub enum LoginFormMsg {}

pub struct LoginForm {}

impl Component for LoginForm {
    type Message = LoginFormMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form class="credentials-box" action="" method="POST">
                <div class="credentials-header">
                    <h1>{ "login" }</h1>
                </div>

                <div class="credentials-form">
                    <div class="credentials-input">
                        <input class="credentials-text" type="email" name="email" placeholder="email"/>
                    </div>
                    <div class="credentials-input">
                        <input class="credentials-text" type="password" name="password" placeholder="password"/>
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

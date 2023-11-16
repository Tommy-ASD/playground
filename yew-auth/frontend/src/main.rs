use web_sys::MouseEvent;
use yew::{
    prelude::{function_component, html, Html},
    Callback,
};

use yew_oauth2::{
    agent::{Client, LoginOptions, OAuth2Operations},
    components::{Authenticated, NotAuthenticated},
    oauth2::{use_auth_agent, Config, OAuth2},
};

#[function_component]
fn Inner() -> Html {
    let agent = use_auth_agent().expect(":(");

    let login = {
        let agent = agent.clone();
        Callback::from(move |_| {
            let _ = agent.start_login_opts(LoginOptions::new().with_redirect_url(
                url::Url::parse("http://localhost:8080/api/oauth_callback/github").unwrap(),
            ));
        })
    };
    let logout = Callback::from(move |_| {
        let _ = agent.logout();
    });

    html! {
    <>
        <h1>{ "Hi" }</h1>
        <NotAuthenticated>
            <p>{ ">:( duffde what fuck!!" }</p>
            <button onclick={login}>{ "LOGIN NOW" }</button>
        </NotAuthenticated>
        <Authenticated>
            <p>{ "Whoe youuaua!!!" }</p>
            <button onclick={logout}>{ "DO NOT LOG OUT>:(" }</button>
        </Authenticated>
    </>
    }
}

#[function_component]
fn Main() -> Html {
    let config = Config {
        client_id: "17edc016503e272418b0".into(),
        auth_url: "https://github.com/login/oauth/authorize".into(),
        token_url: "http://localhost:8080/api/oauth_callback/github".into(),
    };

    html! {
        <OAuth2 {config}>
            <Inner/>
        </OAuth2>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}

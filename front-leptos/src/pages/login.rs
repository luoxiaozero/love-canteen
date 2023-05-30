use crate::api::login::*;
use leptos::*;
use melt_ui::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let account = create_rw_signal(cx, String::new());
    let password = create_rw_signal(cx, String::new());

    let login = move |_| {
        login_api(
            LoginData {
                account: account.get_untracked(),
                password: password.get_untracked(),
            },
            |v| {
                log!("login s");
                match  v {
                    Ok(data) => log!("{}", data.token),
                    Err(err) => log!("{err}"),
                };
            },
        );
    };

    view! { cx,
        <div class="text-center p-2">"登录"</div>
        <div class="p-2">
            <Input value=account on_input=account.into() />
        </div>
        <div class="p-2">
            <Input value=password on_input=password.into()/>
        </div>
        <Button on:click=login>"登录"</Button>
    }
}


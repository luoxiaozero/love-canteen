use leptos::*;
use melt_ui::*;
use crate::api::login::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let account = create_rw_signal(cx, String::new());
    let password = create_rw_signal(cx, String::new());
    let login = login_api(cx, move || LoginData {
        account: account.get(),
        password: password.get()
    });

    login.with(cx, |_| {
        log!("login s");
    });

    view! { cx,
        <div class="text-center p-2">"登录"</div>
        <div class="p-2">
            <Input value=account on_input=account.into() />
        </div>
        <div class="p-2">
            <Input value=password on_input=password.into()/>
        </div>
        <Button on:click=move |_| login.refetch()>"登录"</Button>
    }
} 
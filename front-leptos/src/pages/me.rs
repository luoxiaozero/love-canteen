use crate::{components::*, store::Token};
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Me(cx: Scope) -> impl IntoView {
    let navigate = use_navigate(cx);

    let logout = move |_| {
        Token::set(String::new());
        _ = navigate("/login", Default::default());
    };
    view! { cx,
        <div class="h-screen py-2" style="background: #eff2f5">
            <div class="p-4 text-center cursor-pointer" style="background: #fff" on:click=logout>
                "退出登陆"
            </div>
        </div>
        <BottomNav />
    }
}

use std::time::Duration;
use crate::{components::*, store::Token};
use leptos::*;
use leptos_router::use_navigate;
use melt_ui::mobile::*;

#[component]
pub fn Me(cx: Scope) -> impl IntoView {
    let navigate = use_navigate(cx);

    let logout = move |_| {
        Token::set(String::new());
        _ = navigate("/login", Default::default());
        show_toast(
            cx,
            ToastOptions {
                message: "退出成功".to_string(),
                duration: Duration::from_millis(2000),
            },
        );
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

use leptos::*;
use melt_ui::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="text-center p-2">"登录"</div>
        <div class="p-2">
            <Input />
        </div>
        <div class="p-2">
            <Input />
        </div>
    }
} 
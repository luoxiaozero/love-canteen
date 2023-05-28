use crate::components::*;
use leptos::*;

#[component]
pub fn Me(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>"我的"</div>
        <BottomNav />
    }
}

use crate::components::*;
use leptos::*;

#[component]
pub fn Order(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>"订单"</div>
        <BottomNav />
    }
}

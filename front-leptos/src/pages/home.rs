use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let navigate = use_navigate(cx);

    request_animation_frame(move || {
        _ = navigate("/cuisine/menu", Default::default());
    });

    view! { cx,
        <div>"Home"</div>
    }
} 
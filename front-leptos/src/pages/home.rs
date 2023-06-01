use leptos::*;
use leptos_router::use_navigate;
use crate::store::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let navigate = use_navigate(cx);

    request_animation_frame(move || {
        let path = if Token::get().is_none() {
            "login"
        } else {
            "/cuisine/menu"
        };
        _ = navigate(path, Default::default());
    });

    view! { cx,
        <div>"Home"</div>
    }
} 
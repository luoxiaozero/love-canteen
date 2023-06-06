use leptos::*;
use leptos_router::use_navigate;
use crate::store::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let navigate = use_navigate(cx);

    request_animation_frame(move || {
        let path = if Token::get().is_none() {
            "login".to_string()
        } else if let Some(shop_id) = DefaultShopId::get() {
            format!("/shop/menu?shop_id={shop_id}")
        } else {
            "/shop".to_string()
        };
        _ = navigate(&path, Default::default());
    });

    view! { cx,
        <div>"Home"</div>
    }
} 
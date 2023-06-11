use leptos::*;
use leptos_router::{use_navigate, use_location, use_query_map};
use melt_ui::mobile::*;
use leptos_icons::*;
use crate::store::DefaultShopId;

#[component]
pub fn BottomNav(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, String::from(""));
    let navigate = use_navigate(cx);
    let loaction = use_location(cx);
    create_effect(cx, move |_| {
        let path = selected.get();
        let pathname = loaction.pathname.get_untracked();
        let params = use_query_map(cx).get_untracked();
        let query_shop_id = params.get("shop_id");
        if path.is_empty() {
            selected.set(pathname);
        } else if path != pathname {
            let path = if path == "/shop/menu" {
                let shop_id = if let Some(query_shop_id) = query_shop_id {
                    query_shop_id.parse::<i32>().ok()
                } else if let Some(shop_id) = DefaultShopId::get() {
                    Some(shop_id)
                } else {
                    None
                };
                if let Some(shop_id) = shop_id {
                    format!("/shop/menu?shop_id={shop_id}")
                } else {
                    "/shop".to_string()
                }
            } else {
                path.to_string()
            };
            _ = navigate(&path, Default::default());
        }
    });
    view! {cx,
        <Tabbar selected>
            <TabbarItem name="/shop/menu" icon=AiIcon::AiBarsOutlined>
                "菜单"
            </TabbarItem>
            <TabbarItem name="/order" icon=AiIcon::AiGroupOutlined>
                "订单"
            </TabbarItem>
            <TabbarItem name="/me" icon=AiIcon::AiUserOutlined>
                "我的"
            </TabbarItem>
        </Tabbar>
    }
}
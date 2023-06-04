use leptos::*;
use leptos_router::{use_navigate, use_location};
use melt_ui::mobile::*;
use leptos_icons::*;

#[component]
pub fn BottomNav(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, String::from(""));
    
    let navigate = use_navigate(cx);
    let loaction = use_location(cx);
    create_effect(cx, move |_| {
        let path = selected.get();
        let pathname = loaction.pathname.get();
        if path.is_empty() {
            selected.set(pathname);
        } else if path != pathname {
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
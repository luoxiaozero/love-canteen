use crate::{
    api::{shop::get_shop_order_api, user::get_order_api},
    components::*,
};
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Order(cx: Scope) -> impl IntoView {
    let is_shop = create_rw_signal(cx, false);
    let title = create_rw_signal(cx, "订单");
    let order_vec = create_rw_signal(cx, vec![]);

    get_shop_order_api(move |list| {
        if let Ok(list) = list {
            if let Some(list) = list {
                order_vec.set(list);
                title.set("商店订单");
                is_shop.set(true);
            } else {
                get_order_api(move |list| {
                    if let Ok(list) = list {
                        order_vec.set(list);
                    }
                });
            }
        }
    });

    view! { cx,
        <TopNav readonly=true title />
        <div class="h-screen box-border" style="padding: 46px 0 50px; background: #eff2f5">
            <For
                each=move || order_vec.get()
                key=|order| order.id
                view=move |cx, order| {
                    let order_id = order.id;
                    let goto_order_detail = move |_| {
                        let navigate = use_navigate(cx);
                        _ = navigate(&format!("/order/detail?order_id={order_id}&is_shop={}", is_shop.get()), Default::default());
                    };
                    let status = store_value(cx, order.status);
                    view! { cx,
                        <div class="m-2 b-rd py-2 px-4 bg-white cursor-pointer" on:click=goto_order_detail>
                            <div>{ order.shop_id }</div>
                            <div>{ move || order_status_to_text(status.with_value(|v| v.clone()), is_shop.get()) }</div>
                            <div>{ order.create_time }</div>
                        </div>
                    }
                }
            />
        </div>
        <BottomNav />
    }
}

pub fn order_status_to_text(status: String, is_shop: bool) -> String {
    if status == "wait" {
        String::from(if is_shop {
            "等待接单"
        } else {
            "等待商家接单"
        })
    } else {
        String::new()
    }
}

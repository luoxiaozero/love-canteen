use crate::{api::user::get_order_api, components::*};
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Order(cx: Scope) -> impl IntoView {
    let order_vec = create_rw_signal(cx, vec![]);
    get_order_api(move |list| {
        if let Ok(list) = list {
            order_vec.set(list)
        }
    });

    view! { cx,
        <div class="h-screen py-1 box-border" style="background: #eff2f5">
            <For
                each=move || order_vec.get()
                key=|order| order.id
                view=move |cx, order| {
                    let order_id = order.id;
                    let goto_order_detail = move |_| {
                        let navigate = use_navigate(cx);
                        _ = navigate(&format!("/order/detail?order_id={order_id}"), Default::default());
                    };
                    view! { cx,
                        <div class="m-2 b-rd py-2 px-4 bg-white cursor-pointer" on:click=goto_order_detail>
                            <div>{ order.shop_id }</div>
                            <div>{ order_status_to_text(order.status) }</div>
                            <div>{ order.create_time }</div>
                        </div>
                    }
                }
            />
        </div>
        <BottomNav />
    }
}

pub fn order_status_to_text(status: String) -> String {
    if status == "wait" {
        String::from("等待商家接单")
    } else {
        String::new()
    }
}

use crate::{api::user::get_order_api, components::*};
use leptos::*;

#[component]
pub fn Order(cx: Scope) -> impl IntoView {
    let order_vec = create_rw_signal(cx, vec![]);
    get_order_api(move |list| {
        if let Ok(list) = list {
            order_vec.set(list)
        }
    });
    view! { cx,
        <div class="h-screen py-2" style="background: #eff2f5">
            <For
                each=move || order_vec.get()
                key=|order| order.id
                view=move |cx, order| {
                    view! { cx,
                        <div class="py-2 px-4 bg-white">
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

fn order_status_to_text(status: String) -> String {
    if status == "wait" {
        String::from("等待商家接单")
    } else {
        String::new()
    }
}

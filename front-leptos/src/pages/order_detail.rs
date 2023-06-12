use super::order::order_status_to_text;
use crate::{api::user::get_order_detail_api, components::TopNav};
use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn OrderDetail(cx: Scope) -> impl IntoView {
    let order_id = use_query_map(cx)
        .get()
        .get("order_id")
        .expect("order_id fond")
        .parse::<i32>()
        .expect("order_id i32");
    let order = create_rw_signal(cx, None);
    get_order_detail_api(order_id, move |data| {
        if let Ok(data) = data {
            order.set(Some(data))
        }
    });
    view! { cx,
        <TopNav title="订单详情" />
        <div class="h-screen pt-46px box-border" style="background: #eff2f5">
            {
                move || {
                    if let Some(order) = order.get() {
                        view! { cx,
                            <div class="p-3">
                                { order_status_to_text(order.status, false) }
                            </div>
                            <div class="bg-white mx-1 mb-2 b-rd">
                                <div class="py-3 px-4">
                                    "下单时间："
                                    { order.create_time }
                                </div>
                            </div>
                        }.into()
                    } else {
                        None
                    }
                }
            }
            <div class="bg-white m-1 b-rd">
                <For
                    each=move || {
                        if let Some(order) = order.get() {
                            order.food_vec
                        } else {
                            vec![]
                        }
                    }
                    key=|food| food.id
                    view=move |cx, food| view! { cx,
                        <div class="py-3 px-4" style="border-bottom: 1px solid #f2f2f2;">
                            <div>{ food.title }</div>
                            <div>
                                { food.value }
                            </div>
                            <div>"x"{ food.count }</div>
                        </div>
                    }
                />
            </div>
        </div>
    }
}

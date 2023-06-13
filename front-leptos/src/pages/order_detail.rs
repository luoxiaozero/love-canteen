use super::order::order_status_to_text;
use crate::{
    api::{
        shop::{accept_order_api, AcceptOrderData},
        user::get_order_detail_api,
    },
    components::TopNav,
};
use leptos::*;
use leptos_router::use_query_map;
use melt_ui::Button;

#[component]
pub fn OrderDetail(cx: Scope) -> impl IntoView {
    let query_map = use_query_map(cx).get();
    let order_id = query_map
        .get("order_id")
        .expect("order_id fond")
        .parse::<i32>()
        .expect("order_id i32");
    let is_shop = match query_map.get("is_shop") {
        Some(v) => v.parse::<bool>().unwrap_or_default(),
        None => false,
    };
    let order = create_rw_signal(cx, None);
    get_order_detail_api(order_id, move |data| {
        if let Ok(data) = data {
            order.set(Some(data))
        }
    });

    let accept_order = move |_| {
        let data = AcceptOrderData {
            order_id,
            accept: true,
            reason: None,
        };
        accept_order_api(data, move |data| {
            if data.is_ok() {
                get_order_detail_api(order_id, move |data| {
                    if let Ok(data) = data {
                        order.set(Some(data))
                    }
                });
            }
        });
    };
    view! { cx,
        <TopNav title="订单详情" />
        <div class="h-screen pt-46px box-border" style="background: #eff2f5">
            {
                move || {
                    if let Some(order) = order.get() {
                        view! { cx,
                            <div class="p-3">
                                { order_status_to_text(order.status.clone(), is_shop) }
                            </div>
                            {
                                if order.status == "wait" {
                                    view! { cx,
                                        <div style="padding: 6px">
                                            <Button style="width: 100%" on:click=accept_order>
                                                "接单"
                                            </Button>
                                        </div>
                                    }.into()
                                } else {
                                    None
                                }
                            }
                            <div class="bg-white mx-1 mb-2 b-rd">
                                {
                                    if is_shop {
                                        view! { cx,
                                            <div class="py-3 px-4">
                                                "客户："
                                                { order.user_id }
                                            </div>
                                        }.into()
                                    } else {
                                        None
                                    }
                                }
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

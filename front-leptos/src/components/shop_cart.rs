use crate::{
    api::user::{new_shop_order_api, NewOrder, NewOrderFood},
    model::FoodModel,
};
use leptos::*;
use leptos_router::use_query_map;
use melt_ui::mobile::*;
use melt_ui::*;
use std::time::Duration;

#[component]
pub fn ShopCart(cx: Scope) -> impl IntoView {
    let cart = use_shop_cart(cx);
    let shop_id = use_query_map(cx)
        .get()
        .get("shop_id")
        .expect("shop_id fond")
        .parse::<i32>()
        .expect("shop_id i32");

    let add_order = move |_| {
        let food_vec = cart.get();
        if !food_vec.is_empty() {
            let food_vec = food_vec
                .iter()
                .map(|v| {
                    let count = v.count;
                    let FoodModel { id, title, value } = v.data.clone();
                    NewOrderFood {
                        id,
                        title,
                        value,
                        count,
                    }
                })
                .collect();
            let new_order = NewOrder { shop_id, food_vec };

            new_shop_order_api(new_order, move |data| {
                let options;
                if let Err(err) = data {
                    options = ToastOptions {
                        message: err,
                        duration: Duration::from_millis(2000),
                    };
                } else {
                    cart.set(vec![]);
                    options = ToastOptions {
                        message: "提交成功".to_string(),
                        duration: Duration::from_millis(2000),
                    };
                }
                show_toast(cx, options)
            })
        }
    };

    view! {cx,
        <div class="fixed bottom-60px left-0 right-0" style="background: #fff">
            <For
                each=move || cart.get()
                key=|food| (food.data.id, food.count)
                view=move |cx, food| {
                    view! {cx,
                        <div>
                            <div class="p-12px flex">
                                <div class="w-60px">
                                </div>
                                <div class="flex-1">
                                    <div>{ food.data.title }</div>
                                    <div>
                                        { move || food.count }
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }
            />
            {
                move || {
                    if cart.get().is_empty() {
                        None
                    } else {
                        view! {cx,
                            <Button style="width: calc(100% - 16px); margin: 8px;" on:click=add_order>
                                "提交订单"
                            </Button>
                        }.into()
                    }
                }
            }
        </div>
    }
}

#[derive(Clone)]
pub struct ShopCartFood {
    data: FoodModel,
    count: i32,
}

type ShopCartVec = Vec<ShopCartFood>;

pub fn provide_shop_cart(cx: Scope) {
    let cart = create_rw_signal::<ShopCartVec>(cx, vec![]);
    provide_context(cx, cart);
}

pub fn use_shop_cart(cx: Scope) -> RwSignal<ShopCartVec> {
    use_context(cx).expect("Vec<ShopCartFood> provide")
}

pub fn food_to_shop_cart(cart: RwSignal<ShopCartVec>, food: FoodModel) {
    cart.update(|cart| {
        for cart_food in cart.iter_mut() {
            if cart_food.data.id == food.id {
                cart_food.count += 1;
                return;
            }
        }
        let food = ShopCartFood {
            data: food,
            count: 1,
        };
        cart.push(food);
    });
}

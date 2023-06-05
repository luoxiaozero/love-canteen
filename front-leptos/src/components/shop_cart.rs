use leptos::*;

use crate::model::FoodModel;

#[component]
pub fn ShopCart(cx: Scope) -> impl IntoView {
    let cart = use_shop_cart(cx);

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

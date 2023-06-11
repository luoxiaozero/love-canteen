use crate::{
    api::shop::*,
    components::*,
    model::{FoodModel, ShopMenuModel},
    store::DefaultShopId,
};
use leptos::*;
use leptos_router::{use_navigate, use_query_map};
use melt_ui::*;

#[component]
pub fn ShopMenu(cx: Scope) -> impl IntoView {
    let cart = use_shop_cart(cx);
    let shop_id = use_query_map(cx)
        .get()
        .get("shop_id")
        .expect("shop_id fond")
        .parse::<i32>()
        .expect("shop_id i32");
    let is_self_shop = create_rw_signal(cx, false);

    let default_shop_id = create_rw_signal(cx, DefaultShopId::get());
    let right_text = create_rw_signal(cx, "");
    create_effect(cx, move |_| {
        let text = if let Some(default_shop_id) = default_shop_id.get() {
            if default_shop_id == shop_id {
                "已默认"
            } else {
                "修改为默认"
            }
        } else {
            "设置为默认"
        };
        right_text.set(text);
    });
    let right_text_click = SignalSetter::map(cx, move |_| {
        let right_text = right_text.get();
        if right_text.is_empty() || right_text == "已默认" {
            return;
        }
        DefaultShopId::set(shop_id);
        default_shop_id.set(DefaultShopId::get());
    });

    let selected_menu_id = create_rw_signal::<i32>(cx, 1);
    let menu_list = create_rw_signal::<Vec<ShopMenuModel>>(cx, vec![]);

    is_self_shop_api(shop_id, move |is_self| {
        if let Ok(is_self) = is_self {
            is_self_shop.set(is_self);
            get_shop_menu_api(shop_id, move |list| {
                if let Ok(list) = list {
                    menu_list.set(list);
                }
            });
        }
    });

    let food_vec = create_rw_signal::<Vec<FoodModel>>(cx, vec![]);
    create_effect(cx, move |_| {
        let menu_id = selected_menu_id.get();
        get_food_vec_api(menu_id, move |food_vec_data| {
            if let Ok(food_vec_data) = food_vec_data {
                food_vec.set(food_vec_data);
            }
        });
    });

    let new_food = move |_| {
        let navigate = use_navigate(cx);
        _ = navigate(
            &format!("/shop/food/add?menu_id={}", selected_menu_id.get()),
            Default::default(),
        );
    };

    let food_to_cart = move |food_id| {
        food_vec.update_untracked(|food_vec| {
            for food in food_vec.iter() {
                if food.id == food_id {
                    food_to_shop_cart(cart, food.clone());
                }
            }
        });
    };

    view! { cx,
        <TopNav back_path="/shop" title="店铺" right_text=right_text click_right=right_text_click />
        <div class="flex h-screen box-border" style="padding: 46px 0 50px" >
            <div style="background: #f2f2f2" class="w-100px">
                {
                    move || {
                        if is_self_shop.get() {
                            view! {cx,
                                <AddMenu menu_vec=menu_list />
                            }.into()
                        } else {
                            None
                        }
                    }
                }
                <For
                    each=move || menu_list.get()
                    key=|menu_item| menu_item.id
                    view=move |cx, menu_item| {
                        view! { cx,
                            <div class="text-center px-6px py-12px cursor-pointer"
                                class:bg-white=move || selected_menu_id.get() == menu_item.id
                                on:click=move |_| selected_menu_id.set(menu_item.id)>
                                { menu_item.title }
                            </div>
                        }
                    }
                />
            </div>
            <main class="flex-1">
                {
                    move || {
                        if is_self_shop.get() {
                            view! {cx,
                                <Button on:click=new_food style="width: calc(100% - 32px); margin: 8px 16px;">
                                    "+"
                                </Button>
                            }.into()
                        } else {
                            None
                        }
                    }
                }
                <For
                    each=move || food_vec.get()
                    key=|food| food.id
                    view=move |cx, food| {
                        let food_id = food.id;
                        let add = move |_| {
                            food_to_cart(food_id);
                        };
                        view! { cx,
                            <div class="p-12px flex">
                                <div class="w-60px">
                                </div>
                                <div class="flex-1">
                                    <div>{food.title}</div>
                                    <div>{food.value}</div>
                                    <div>
                                        <Button on:click=add>
                                            "加入购物车"
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
            </main>
        </div>
        <ShopCart />
        <BottomNav />
    }
}

#[component]
pub fn AddMenu(cx: Scope, menu_vec: RwSignal<Vec<ShopMenuModel>>) -> impl IntoView {
    let is_show_new_menu = create_rw_signal(cx, false);
    let new_menu_title = create_rw_signal(cx, String::new());
    let open_new_menu_modal = move |_| {
        is_show_new_menu.set(true);
        new_menu_title.set(String::new());
    };

    let new_menu = move |_| {
        new_shop_menu_api(
            NewShopMenu {
                title: new_menu_title.get_untracked(),
            },
            move |shop_menu| {
                if let Ok(shop_menu) = shop_menu {
                    menu_vec.update(|value| {
                        value.push(shop_menu);
                    });
                }
                is_show_new_menu.set(false);
            },
        );
    };

    view! {cx,
        <div class="text-center px-6px py-12px cursor-pointer" on:click=open_new_menu_modal>
            "+"
        </div>
        <Modal show=is_show_new_menu title="新建菜单">
            <Input value=new_menu_title/>
            <Button on:click=new_menu>
                "添加"
            </Button>
        </Modal>
    }
}

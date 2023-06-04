use crate::{
    api::shop::*,
    components::*,
    model::{FoodModel, ShopMenuModel},
};
use leptos::*;
use leptos_router::use_navigate;
use melt_ui::*;

#[component]
pub fn ShopMenu(cx: Scope) -> impl IntoView {
    let selected_menu_id = create_rw_signal::<i32>(cx, 1);
    let menu_list = create_rw_signal::<Vec<ShopMenuModel>>(cx, vec![]);
    get_shop_menu_api(move |list| {
        if let Ok(list) = list {
            menu_list.set(list);
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
                    menu_list.update(|value| {
                        value.push(shop_menu);
                    });
                }
                is_show_new_menu.set(false);
            },
        );
    };

    let new_food = move |_| {
        let navigate = use_navigate(cx);
        _ = navigate(
            &format!("/shop/food/add?menu_id={}", selected_menu_id.get()),
            Default::default(),
        );
    };

    view! { cx,
        <div class="flex h-screen">
            <div style="background: #f2f2f2" class="w-100px">
                <div class="text-center px-6px py-12px cursor-pointer" on:click=open_new_menu_modal>
                    "+"
                </div>
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
                <Button on:click=new_food>
                    "+"
                </Button>
                <For
                    each=move || food_vec.get()
                    key=|food| food.id
                    view=move |cx, food| {
                        view! { cx,
                            <div class="p-12px flex">
                                <div class="w-60px">
                                </div>
                                <div class="flex-1">
                                    <div>{food.title}</div>
                                </div>
                            </div>
                        }
                    }
                />
            </main>
        </div>
        <BottomNav />
        <Modal show=is_show_new_menu title="新建菜单">
            <Input value=new_menu_title/>
            <Button on:click=new_menu>
                "添加"
            </Button>
        </Modal>
    }
}

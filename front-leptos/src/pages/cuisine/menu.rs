use crate::api::shop::*;
use crate::{api::shop, components::*, model::ShopMenuModel};
use leptos::*;
use melt_ui::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleCuisineModel {
    id: u32,
    name: String,
    image_url: String,
}

#[component]
pub fn ShopMenu(cx: Scope) -> impl IntoView {
    let selected_menu_id = create_rw_signal::<u32>(cx, 1);
    let menu_list = create_rw_signal::<Vec<ShopMenuModel>>(cx, vec![]);

    get_shop_menu_api(move |list| {
        if let Ok(list) = list {
            menu_list.set(list);
        }
    });
    let (cuisine_list, set_cuisine_list) = create_signal::<Vec<SimpleCuisineModel>>(
        cx,
        vec![
            SimpleCuisineModel {
                id: 1,
                name: "苹果".to_string(),
                image_url: "".to_string(),
            },
            SimpleCuisineModel {
                id: 2,
                name: "香蕉".to_string(),
                image_url: "".to_string(),
            },
        ],
    );
    let is_show_new_menu = create_rw_signal(cx, false);
    let on_cancel = SignalSetter::map(cx, move |_| {
        is_show_new_menu.set(false);
    });

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
                <For
                    each=move || cuisine_list.get()
                    key=|cuisine| cuisine.id
                    view=move |cx, cuisine| {
                        view! { cx,
                            <div class="p-12px flex">
                                <div class="w-60px">
                                </div>
                                <div class="flex-1">
                                    <div>{cuisine.name}</div>
                                </div>
                            </div>
                        }
                    }
                />
            </main>
        </div>
        <BottomNav />
        <Modal open=is_show_new_menu on_cancel title="新建菜单">
            <Input value=new_menu_title/>
            <Button on:click=new_menu>
                "添加"
            </Button>
        </Modal>
    }
}

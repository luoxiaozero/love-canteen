use leptos::*;
use crate::components::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CuisineMenuModel {
    id: u32,
    name: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleCuisineModel {
    id: u32,
    name: String,
    image_url: String
}

#[component]
pub fn Menu(cx: Scope) -> impl IntoView {
    let selected_menu_id = create_rw_signal::<u32>(cx, 1);
    let (menu_list, set_menu_list) = create_signal::<Vec<CuisineMenuModel>>(cx, vec![
        CuisineMenuModel {
            id: 1,
            name: "水果".to_string()
        }
    ]);
    let (cuisine_list, set_cuisine_list) = create_signal::<Vec<SimpleCuisineModel>>(cx, vec![
        SimpleCuisineModel {
            id: 1,
            name: "苹果".to_string(),
            image_url: "".to_string()
        },
        SimpleCuisineModel {
            id: 2,
            name: "香蕉".to_string(),
            image_url: "".to_string()
        }
    ]);
    view! { cx,
        <div class="flex h-screen">
            <div style="background: #f2f2f2" class="w-100px">
                <For 
                    each=move || menu_list.get() 
                    key=|menu_item| menu_item.id 
                    view=move |cx, menu_item| {
                        view! { cx,
                            <div class="text-center px-6px py-12px cursor-pointer" 
                                class:bg-white=move || selected_menu_id.get() == menu_item.id
                                on:click=move |_| selected_menu_id.set(menu_item.id)>
                                { menu_item.name }
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
    }
}
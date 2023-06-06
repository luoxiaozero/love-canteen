use leptos::*;
use leptos_router::use_navigate;
use crate::api::shop::get_shop_vec_api;
use crate::components::BottomNav;
use crate::model::ShopModel;

#[component]
pub fn ShopVec(cx: Scope) -> impl IntoView {
    let shop_vec = create_rw_signal::<Vec<ShopModel>>(cx, vec![]);
    get_shop_vec_api(move |list| {
        if let Ok(list) = list {
            shop_vec.set(list);
        }
    });
    view! { cx,
        <div class="h-screen py-2" style="background: #eff2f5">
            <For 
                each=move || shop_vec.get()
                key=|shop| shop.id
                view=move |cx, shop| {
                    let navigate = use_navigate(cx);
                    let onclick = move |_| {
                        _  = navigate(&format!("/shop/menu?shop_id={}", shop.id), Default::default());
                    };
                    view! { cx,
                        <div class="py-2 px-4 cursor-ponter" style="background: #fff" on:click=onclick>
                            {shop.id}
                        </div>
                    }
                }
            />
        </div>
        <BottomNav />
    }
}
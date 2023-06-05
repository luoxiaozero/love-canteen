use leptos::*;
use leptos_router::{use_navigate, use_query_map};
use melt_ui::*;
use crate::components::*;
use crate::api::shop::*;

#[component]
pub fn AddFood(cx: Scope) -> impl IntoView {
    let query_map = use_query_map(cx);

    let title = create_rw_signal(cx, String::new());
    let food_value = create_rw_signal(cx, String::new());

    let new_food = move |_| {
        let menu_id = query_map.get_untracked().get("menu_id").unwrap().parse::<i32>().unwrap();
        new_food_api(
            NewFood {
                menu_id_vec: vec![menu_id],
                title: title.get(),
                value: food_value.get()
            },
            move |food| {
                if food.is_ok() {
                    let navigate = use_navigate(cx);
                    _ = navigate("/shop/menu", Default::default());
                }
            },
        );
    };
    view! {cx,
        <div>
            <TopNav back_path="/shop/menu" title="新建食物"/>
            <div class="p-2" style="padding-top: 54px">
                <div>
                    "名字："
                    <Input value=title/>
                </div>
                <div>
                    "价值："
                    <Input value=food_value/>
                </div>
                <div>
                    <Button on:click=new_food>
                        "添加"
                    </Button>
                </div>
            </div>
        </div>
    }
}
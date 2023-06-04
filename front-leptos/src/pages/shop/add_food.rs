use leptos::*;
use leptos_router::use_navigate;
use melt_ui::*;
use crate::components::*;
use crate::api::shop::*;

#[component]
pub fn AddFood(cx: Scope) -> impl IntoView {
    let title = create_rw_signal(cx, String::new());
    let food_value = create_rw_signal(cx, String::new());

    let new_food = move |_| {
        new_food_api(
            NewFood {
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
use crate::{components::provide_shop_cart, pages::*};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_shop_cart(cx);
    view! { cx,
        <Router>
            <Routes base="/shop".to_string() >
                <Route path="" view=move |cx| view! { cx,
                    <ShopVec />
                }/>
                <Route path="/menu" view=move |cx| view! { cx,
                    <ShopMenu />
                }/>
                <Route path="/food/add" view=move |cx| view! { cx,
                    <AddFood />
                }/>
                <Route path="/food" view=move |cx| view! { cx,
                    <Food />
                }/>
            </Routes>
            <Routes>
                <Route path="/" view=move |cx| view! { cx,
                    <Home />
                }/>
                <Route path="/order" view=move |cx| view! { cx,
                    <Order />
                }/>
                <Route path="/order/detail" view=move |cx| view! { cx,
                    <OrderDetail />
                }/>
                <Route path="/me" view=move |cx| view! { cx,
                    <Me />
                }/>
                <Route path="/login" view=move |cx| view! { cx,
                    <Login />
                }/>
            </Routes>
        </Router>
    }
}

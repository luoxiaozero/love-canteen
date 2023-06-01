use crate::pages::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes base="/cuisine".to_string() >
                <Route path="/menu" view=move |cx| view! { cx,
                    <Menu />
                }/>
                <Route path="/input" view=move |cx| view! { cx,
                    "录入菜品"
                }/>
                <Route path="/detail" view=move |cx| view! { cx,
                    "菜品详情"
                }/>
            </Routes>
            <Routes>
                <Route path="/" view=move |cx| view! { cx,
                    <Home />
                }/>
                <Route path="/order" view=move |cx| view! { cx,
                    <Order />
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

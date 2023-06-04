use leptos::*;
use leptos_router::use_navigate;
use melt_ui::mobile::*;

#[component]
pub fn TopNav(
    cx: Scope,
    #[prop(into)] back_path: String,
    #[prop(optional, into)] title: MaybeSignal<String>,
) -> impl IntoView {
    let click_left = SignalSetter::map(cx, move |_| {
        let navigate = use_navigate(cx);
        _ = navigate(&back_path, Default::default());
    });
    view! {cx,
        <NavBar left_arrow=true left_text="返回" title click_left/>
    }
}

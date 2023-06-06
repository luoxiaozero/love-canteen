use leptos::{ev::MouseEvent, *};
use leptos_router::use_navigate;
use melt_ui::mobile::*;

#[component]
pub fn TopNav(
    cx: Scope,
    #[prop(into)] back_path: String,
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] right_text: MaybeSignal<String>,
    #[prop(optional, into)] click_right: Option<SignalSetter<MouseEvent>>,
) -> impl IntoView {
    let click_left = SignalSetter::map(cx, move |_| {
        let navigate = use_navigate(cx);
        _ = navigate(&back_path, Default::default());
    });
    let onclick_right = SignalSetter::map(cx, move |ev| {
        if let Some(click_right) = click_right {
            click_right.set(ev);
        }
    });
    view! {cx,
        <NavBar left_arrow=true left_text="返回" title click_left right_text click_right=onclick_right/>
    }
}

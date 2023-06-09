use leptos::{ev::MouseEvent, *};
use leptos_router::use_navigate;
use melt_ui::mobile::*;

#[component]
pub fn TopNav(
    cx: Scope,
    #[prop(optional, into)] readonly: bool,
    #[prop(optional, into)] back_path: Option<&'static str>,
    #[prop(optional, into)] title: MaybeSignal<&'static str>,
    #[prop(optional, into)] right_text: MaybeSignal<&'static str>,
    #[prop(optional, into)] click_right: Option<SignalSetter<MouseEvent>>,
) -> impl IntoView {
    let click_left = SignalSetter::map(cx, move |_| {
        if readonly {
            return;
        }
        let navigate = use_navigate(cx);
        if let Some(back_path) = back_path {
            _ = navigate(&back_path, Default::default());
        } else {
            _ = window().history().unwrap().back();
        }
    });
    let onclick_right = SignalSetter::map(cx, move |ev| {
        if readonly {
            return;
        }
        if let Some(click_right) = click_right {
            click_right.set(ev);
        }
    });
    let left_arrow = !readonly;
    let left_text = if readonly { "" } else { "返回" };
    view! {cx,
        <NavBar left_arrow left_text title click_left right_text click_right=onclick_right/>
    }
}

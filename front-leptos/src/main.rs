mod app;
mod components;
mod pages;
mod api;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}

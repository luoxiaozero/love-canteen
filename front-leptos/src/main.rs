mod api;
mod app;
mod components;
mod model;
mod pages;
mod store;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}

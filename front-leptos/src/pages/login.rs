use crate::api::login::*;
use crate::components::use_shop_cart;
use crate::store::*;
use leptos::*;
use leptos_router::use_navigate;
use melt_ui::{mobile::*, *};
use sha3::{Digest, Sha3_256};
use std::time::Duration;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let navigate = use_navigate(cx);
    request_animation_frame(move || {
        if Token::get().is_some() {
            _ = navigate("/", Default::default());
        }
    });

    let account = create_rw_signal(cx, String::new());
    let password = create_rw_signal(cx, String::new());
    let error_text = create_rw_signal(cx, String::new());

    let login = move |_| {
        let account = account.get_untracked();
        if account.is_empty() {
            error_text.set(String::from("账号不能为空"));
            return;
        }
        let password = password.get_untracked();

        if password.is_empty() {
            error_text.set(String::from("密码不能为空"));
            return;
        }
        let mut hasher = Sha3_256::new();
        hasher.update(password);
        let hex_vec = hasher.finalize().to_vec();
        let password = hex::encode(hex_vec);

        login_api(LoginData { account, password }, move |v| {
            match v {
                Ok(data) => {
                    let navigate = use_navigate(cx);
                    Token::set(data.token);
                    let cart = use_shop_cart(cx);
                    cart.set(vec![]);
                    _ = navigate("/", Default::default());
                    show_toast(
                        cx,
                        ToastOptions {
                            message: "登陆成功".to_string(),
                            duration: Duration::from_millis(2000),
                        },
                    );
                }
                Err(err) => {
                    error_text.set(err);
                }
            };
        });
    };

    view! { cx,
        <div class="text-center p-2">"登录"</div>
        <div class="p-2">
            <Input value=account />
        </div>
        <div class="p-2">
            <Input value=password type_="password"/>
        </div>
        <div class="p-1" style="color: red">
            { move || error_text.get() }
        </div>
        <Button on:click=login>"登录"</Button>
    }
}

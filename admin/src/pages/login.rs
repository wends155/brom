use crate::auth::{auth_fetch, save_token_to_storage};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct LoginResponse {
    token: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    let navigate = use_navigate();

    view! {
        <div class="min-h-screen flex items-center justify-center bg-muted/30 p-4">
            <div class="w-full max-w-md space-y-8 bg-card p-8 rounded-xl border shadow-lg">
                <div class="text-center space-y-2">
                    <h1 class="text-3xl font-bold tracking-tight">"Welcome back"</h1>
                    <p class="text-muted-foreground">"Enter your credentials to access the admin panel"</p>
                </div>

                <form class="space-y-6" on:submit=move |ev| {
                    ev.prevent_default();
                    set_loading.set(true);
                    set_error.set(None);

                    let email_val = email.get();
                    let password_val = password.get();

                    let navigate = navigate.clone();
                    leptos::task::spawn_local(async move {
                        let payload = LoginRequest {
                            email: email_val,
                            password: password_val,
                        };

                        let resp = auth_fetch("/admin/api/login", "POST", Some(payload)).await;

                        match resp {
                            Ok(resp) if resp.ok() => {
                                if let Ok(data) = resp.json::<LoginResponse>().await {
                                    save_token_to_storage(&data.token);
                                    navigate("/admin", Default::default());
                                } else {
                                    set_error.set(Some("Failed to parse login response".into()));
                                }
                            }
                            Ok(resp) => {
                                set_error.set(Some(format!("Login failed: {}", resp.status())));
                            }
                            Err(e) => {
                                set_error.set(Some(e));
                            }
                        }
                        set_loading.set(false);
                    });
                }>
                    <div class="space-y-2">
                        <label class="text-sm font-semibold leading-none">"Email"</label>
                        <input
                            type="email"
                            placeholder="admin@example.com"
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            prop:value=email
                            required
                        />
                    </div>

                    <div class="space-y-2">
                        <label class="text-sm font-semibold leading-none">"Password"</label>
                        <input
                            type="password"
                            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            prop:value=password
                            required
                        />
                    </div>

                    {move || error.get().map(|err| view! {
                        <div class="p-3 text-sm bg-destructive/10 text-destructive rounded-md border border-destructive/20 font-medium">
                            {err}
                        </div>
                    })}

                    <button
                        type="submit"
                        disabled=loading
                        class="w-full h-10 px-4 py-2 bg-primary text-primary-foreground rounded-md font-semibold hover:bg-primary/90 transition-colors disabled:opacity-50"
                    >
                        {move || if loading.get() {
                            Either::Left("Authenticating...")
                        } else {
                            Either::Right("Sign In")
                        }}
                    </button>
                </form>

                <div class="text-center text-xs text-muted-foreground">
                    "Secure Authentication powered by Brom"
                </div>
            </div>
        </div>
    }
}

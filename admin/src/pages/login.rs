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
        <div class="min-h-screen flex items-center justify-center bg-background p-4 font-body">
            <div class="bg-surface border border-border p-8 w-full max-w-md">
                <div class="w-16 h-16 bg-primary flex items-center justify-center mx-auto mb-6">
                    <span class="text-primary-foreground font-heading font-bold text-3xl">"B"</span>
                </div>
                <div class="text-center space-y-2 mb-8">
                    <h1 class="text-2xl font-heading font-bold tracking-tight text-foreground">"Welcome back"</h1>
                    <p class="text-sm text-muted-foreground font-mono">"Enter your credentials to access the admin panel"</p>
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
                        <label class="text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground leading-none">"Email"</label>
                        <input
                            type="email"
                            placeholder="admin@example.com"
                            class="bg-input border border-border px-3 py-2 text-sm text-foreground font-mono forge-focus w-full"
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            prop:value=email
                            required
                        />
                    </div>

                    <div class="space-y-2">
                        <label class="text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground leading-none">"Password"</label>
                        <input
                            type="password"
                            class="bg-input border border-border px-3 py-2 text-sm text-foreground font-mono forge-focus w-full"
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            prop:value=password
                            required
                        />
                    </div>

                    {move || error.get().map(|err| view! {
                        <div class="p-3 text-sm bg-destructive/10 text-destructive border border-destructive/20 font-mono">
                            {err}
                        </div>
                    })}

                    <button
                        type="submit"
                        disabled=loading
                        class="w-full py-2 bg-primary text-primary-foreground font-heading font-semibold hover:bg-primary/90 transition-colors disabled:opacity-50"
                    >
                        {move || if loading.get() {
                            Either::Left("Authenticating...")
                        } else {
                            Either::Right("Sign In")
                        }}
                    </button>
                </form>

                <div class="text-center text-xs text-muted-foreground/50 font-mono mt-8">
                    "Secure Authentication powered by Brom"
                </div>
            </div>
        </div>
    }
}

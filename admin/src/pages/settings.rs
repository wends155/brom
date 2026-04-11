use crate::auth::auth_fetch;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiKey {
    pub prefix: String,
    pub name: String,
    pub permissions: String,
    pub status: String,
    pub created_at: String,
}

/// API key management page.
#[component]
pub fn Settings() -> impl IntoView {
    let (refresh_keys, set_refresh_keys) = signal(0);
    let (new_key_name, set_new_key_name) = signal(String::new());
    let (show_new_key, set_show_new_key) = signal(None::<String>);

    let keys = LocalResource::new(move || {
        let _ = refresh_keys.get();
        async move {
            let resp = auth_fetch("/admin/api/keys", "GET", None::<()>)
                .await
                .ok()?;
            resp.json::<Vec<ApiKey>>().await.ok()
        }
    });

    let create_key = move |_| {
        let name = new_key_name.get();
        if name.is_empty() {
            return;
        }

        spawn_local(async move {
            let body = serde_json::json!({ "name": name, "permissions": "read_write" });
            if let Ok(resp) = auth_fetch("/admin/api/keys", "POST", Some(body)).await {
                // Combined into one inner logic to satisfy Clippy while waiting for let_chains
                let key_opt = resp
                    .json::<serde_json::Value>()
                    .await
                    .ok()
                    .and_then(|data| {
                        data.get("key")
                            .and_then(|k| k.as_str())
                            .map(|s| s.to_string())
                    });
                if let Some(key) = key_opt {
                    set_show_new_key.set(Some(key));
                    set_new_key_name.set(String::new());
                    set_refresh_keys.update(|n| *n += 1);
                }
            }
        });
    };

    let revoke_key = move |prefix: String| {
        spawn_local(async move {
            if auth_fetch(&format!("/admin/api/keys/{}", prefix), "DELETE", None::<()>)
                .await
                .is_ok()
            {
                set_refresh_keys.update(|n| *n += 1);
            }
        });
    };

    view! {
        <div class="space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-2xl font-heading font-bold text-foreground">"API Keys"</h1>
                    <p class="text-sm text-muted-foreground mt-1">"Manage programmatic access to your CMS."</p>
                </div>
            </div>

            <div class="bg-surface p-6 border border-border space-y-4">
                <h2 class="text-sm font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Generate New Key"</h2>
                <div class="flex gap-4">
                    <input
                        type="text"
                        placeholder="Key Name (e.g. CI/CD)"
                        class="flex-1 bg-background border border-border px-3 py-2 text-sm font-mono focus:outline-none focus:ring-1 focus:ring-primary"
                        prop:value=new_key_name
                        on:input=move |e| set_new_key_name.set(event_target_value(&e))
                    />
                    <button
                        on:click=create_key
                        class="bg-primary text-primary-foreground px-4 py-2 font-heading font-semibold hover:bg-primary/90 transition-colors duration-100"
                    >
                        "Generate"
                    </button>
                </div>

                {move || show_new_key.get().map(|key| view! {
                    <div class="mt-4 p-4 bg-success/10 border border-success/30 rounded">
                        <p class="text-xs text-success font-semibold mb-2">"Key generated! Copy it now, you won't see it again:"</p>
                        <div class="flex gap-2 font-mono text-sm break-all">
                            <span class="flex-1 select-all">{key}</span>
                            <button
                                on:click=move |_| set_show_new_key.set(None)
                                class="text-xs text-muted-foreground hover:text-foreground"
                            >
                                "[Dismiss]"
                            </button>
                        </div>
                    </div>
                })}
            </div>

            <div class="border border-border overflow-hidden">
                <table class="w-full text-sm text-left font-mono">
                    <thead class="bg-surface border-b border-border">
                        <tr>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Prefix"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Name"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Status"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Created"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground text-right">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <Suspense fallback=move || view! { <tr><td colspan="5" class="p-8 text-center text-muted-foreground font-mono">"Loading keys..."</td></tr> }>
                            {move || keys.get().map(|res| {
                                if let Some(key_list) = &*res {
                                    key_list.iter().enumerate().map(|(i, key)| {
                                        let prefix = key.prefix.clone();
                                        let name = key.name.clone();
                                        let status = key.status.clone();
                                        let created_at = key.created_at.clone();
                                        let stripe = if i % 2 == 0 { "" } else { "bg-surface/30" };
                                        let status_class = "bg-success/20 text-success px-2 py-0.5 text-xs font-semibold";
                                        view! {
                                            <tr class=format!("border-b border-border/50 hover:bg-accent transition-colors duration-100 {}", stripe)>
                                                <td class="px-4 py-3 text-foreground">{prefix.clone()}</td>
                                                <td class="px-4 py-3 text-foreground">{name}</td>
                                                <td class="px-4 py-3">
                                                    <span class=status_class>{status}</span>
                                                </td>
                                                <td class="px-4 py-3 text-muted-foreground">{created_at}</td>
                                                <td class="px-4 py-3 text-right">
                                                    <button
                                                        on:click=move |_| revoke_key(prefix.clone())
                                                        class="text-destructive hover:text-destructive/80 font-medium text-sm transition-colors duration-100"
                                                    >
                                                        "Revoke"
                                                    </button>
                                                </td>
                                            </tr>
                                        }.into_any()
                                    }).collect::<Vec<_>>()
                                } else {
                                    vec![view! { <tr><td colspan="5" class="p-8 text-center text-destructive font-mono">"Failed to load keys"</td></tr> }.into_any()]
                                }
                            })}
                        </Suspense>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

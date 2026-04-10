use crate::auth::auth_fetch;
use crate::context::schema_ctx::use_schema;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde_json::Value;

#[component]
pub fn EditorForm() -> impl IntoView {
    let params = use_params_map();
    let schema_ctx = use_schema();

    // Resource to fetch the record data
    let data = LocalResource::new(move || async move {
        let p = params.get();
        let entity_info = p.get("entity").zip(p.get("id"));
        if let Some((entity, id)) = entity_info {
            let url = format!("/admin/api/entities/{}/{}", entity, id);
            let resp = auth_fetch(&url, "GET", None::<()>).await?;
            if resp.ok() {
                resp.json::<Value>().await.map_err(|e| e.to_string())
            } else {
                Err(format!(
                    "Failed to fetch record {}/{}: {}",
                    entity,
                    id,
                    resp.status()
                ))
            }
        } else {
            Err("Missing entity or id".to_string())
        }
    });

    view! {
        <div class="max-w-4xl mx-auto space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h2 class="text-2xl font-heading font-bold text-foreground">
                        {move || {
                            let p = params.get();
                            format!("Edit {}", p.get("entity").unwrap_or_default())
                        }}
                    </h2>
                    <p class="text-sm text-muted-foreground font-mono mt-1">
                        {move || {
                            let p = params.get();
                            format!("ID: {}", p.get("id").unwrap_or_default())
                        }}
                    </p>
                </div>
                <div class="space-x-4">
                    <button class="px-4 py-2 border border-border text-foreground hover:bg-muted transition-colors duration-100 font-heading font-medium">
                        "Cancel"
                    </button>
                    <button class="px-4 py-2 bg-primary text-primary-foreground font-heading font-semibold hover:bg-primary/90 transition-colors duration-100">
                        "Save Changes"
                    </button>
                </div>
            </div>

            <div class="bg-surface border border-border p-8">
                <Suspense fallback=move || view! { <div class="text-center text-muted-foreground font-mono">"Loading form..."</div> }>
                    {move || {
                        data.get().map(|res| {
                            match &*res {
                                Ok(item) => {
                                    let entity_name = params.get().get("entity").unwrap_or_default();
                                    let schema = schema_ctx.schemas.get().and_then(|s| s.as_ref().ok().cloned()).and_then(|ss| {
                                        ss.into_iter().find(|s| s.table_name == entity_name)
                                    });

                                    if let Some(schema) = schema {
                                        view! {
                                            <form class="space-y-6">
                                                {schema.fields.iter().filter(|f| !f.hidden && f.name != "id").map(|f| {
                                                    let val: Value = item.get(&f.name).cloned().unwrap_or(Value::Null);
                                                    let label = f.name.clone();
                                                    view! {
                                                        <div class="space-y-2">
                                                            <label class="text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground leading-none">
                                                                {label}
                                                            </label>
                                                            <input
                                                                class="bg-input border border-border px-3 py-2 text-sm font-mono text-foreground forge-focus w-full disabled:cursor-not-allowed disabled:opacity-50"
                                                                value=val.to_string().replace("\"", "")
                                                            />
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </form>
                                        }.into_any()
                                    } else {
                                        view! { <div class="text-center text-muted-foreground font-mono">"Schema not found"</div> }.into_any()
                                    }
                                }
                                Err(e) => {
                                    let e = e.clone();
                                    view! { <div class="text-center text-destructive font-mono">{e}</div> }.into_any()
                                }
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

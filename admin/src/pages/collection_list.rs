use crate::auth::auth_fetch;
use crate::components::table::DataTable;
use crate::context::schema_ctx::use_schema;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use serde_json::Value;

#[component]
pub fn CollectionList() -> impl IntoView {
    let params = use_params_map();
    let schema_ctx = use_schema();

    // Resource to fetch the collection data
    let data = LocalResource::new(move || async move {
        let entity = params.get().get("entity");
        if let Some(entity_name) = entity {
            let url = format!("/api/v1/{}", entity_name);
            let resp = auth_fetch(&url, "GET", None::<()>).await?;
            if resp.ok() {
                resp.json::<Vec<Value>>().await.map_err(|e| e.to_string())
            } else {
                Err(format!(
                    "Failed to fetch data for {}: {}",
                    entity_name,
                    resp.status()
                ))
            }
        } else {
            Ok(Vec::new())
        }
    });

    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-heading font-bold text-foreground">
                    {move || params.get().get("entity").unwrap_or_default()}
                </h2>
                <A
                    href=move || format!("/admin/{}/new", params.get().get("entity").unwrap_or_default())
                    attr:class="bg-primary text-primary-foreground px-4 py-2 font-heading font-semibold hover:bg-primary/90 transition-colors duration-100"
                >
                    "Create New"
                </A>
            </div>

            <Suspense fallback=move || view! { <div class="p-8 text-center text-muted-foreground font-mono">"Loading data..."</div> }>
                {move || {
                    data.get().map(|res| {
                        match &*res {
                            Ok(items) => {
                                let entity_name = params.get().get("entity").unwrap_or_default();
                                let schema = schema_ctx.schemas.get().and_then(|s| s.as_ref().ok().cloned()).and_then(|ss| {
                                    ss.iter().find(|s| s.table_name == entity_name).cloned()
                                });

                                if let Some(schema) = schema {
                                    let fields = schema.fields.clone();

                                    let headers: Vec<String> = fields
                                        .iter()
                                        .filter(|f| !f.hidden)
                                        .map(|f| f.name.clone())
                                        .collect();

                                    let table_rows: Vec<Vec<String>> = items.iter().map(|item| {
                                        fields.iter().filter(|f| !f.hidden).map(|f| {
                                            item.get(&f.name)
                                                .map(|val| val.to_string().replace("\"", ""))
                                                .unwrap_or_default()
                                        }).collect()
                                    }).collect();

                                    view! {
                                        <DataTable
                                            headers=headers
                                            rows=table_rows
                                            entity_name=entity_name
                                        />
                                    }.into_any()
                                } else {
                                    view! { <div class="p-8 text-center text-muted-foreground font-mono">"Schema not found"</div> }.into_any()
                                }
                            }
                            Err(e) => {
                                let e = e.clone();
                                view! { <div class="p-8 text-center text-destructive font-mono">{e}</div> }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

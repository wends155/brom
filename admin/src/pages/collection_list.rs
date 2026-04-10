use crate::auth::auth_fetch;
use crate::context::schema_ctx::use_schema;
use leptos::prelude::*;
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
            let url = format!("/admin/api/entities/{}", entity_name);
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
                <h2 class="text-3xl font-bold tracking-tight">
                    {move || params.get().get("entity").unwrap_or_default()}
                </h2>
                <button class="bg-primary text-primary-foreground px-4 py-2 rounded-md hover:bg-primary/90 transition-colors">
                    "Add New"
                </button>
            </div>

            <div class="border rounded-lg overflow-hidden bg-card">
                <Suspense fallback=move || view! { <div class="p-8 text-center text-muted-foreground">"Loading data..."</div> }>
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

                                        view! {
                                            <table class="w-full text-sm text-left">
                                                <thead class="bg-muted/50 border-b">
                                                    <tr>
                                                        {fields.iter().filter(|f| !f.hidden).map(|f| {
                                                            view! { <th class="px-4 py-3 font-semibold">{f.name.clone()}</th> }
                                                        }).collect::<Vec<_>>()}
                                                        <th class="px-4 py-3 font-semibold text-right">"Actions"</th>
                                                    </tr>
                                                </thead>
                                                <tbody class="divide-y">
                                                    {items.iter().map(|item| {
                                                        let item_id = item.get("id").and_then(|id| id.as_i64()).unwrap_or(0);
                                                        let fields_inner = fields.clone();
                                                        let entity_name_inner = entity_name.clone();

                                                        view! {
                                                            <tr class="hover:bg-muted/30 transition-colors">
                                                                {fields_inner.iter().filter(|f| !f.hidden).map(|f| {
                                                                    let val = item.get(&f.name).cloned().unwrap_or(Value::Null);
                                                                    view! { <td class="px-4 py-3 font-medium">{val.to_string().replace("\"", "")}</td> }
                                                                }).collect::<Vec<_>>()}
                                                                <td class="px-4 py-3 text-right">
                                                                    <a
                                                                        href=format!("/admin/collection/{}/{}", entity_name_inner, item_id)
                                                                        class="text-primary hover:underline font-medium"
                                                                    >
                                                                        "Edit"
                                                                    </a>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </tbody>
                                            </table>
                                        }.into_any()
                                    } else {
                                        view! { <div class="p-8 text-center text-muted-foreground">"Schema not found"</div> }.into_any()
                                    }
                                }
                                Err(e) => {
                                    let e = e.clone();
                                    view! { <div class="p-8 text-center text-destructive">{e}</div> }.into_any()
                                }
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

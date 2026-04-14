use crate::auth::auth_fetch;
use crate::components::inputs::{
    ForgeCheckbox, ForgeInput, ForgeLabel, ForgeSelect, ForgeTextarea,
};
use crate::context::schema_ctx::use_schema;
use brom_core::entity::FieldType;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use serde_json::Value;

#[component]
pub fn EditorForm() -> impl IntoView {
    let params = use_params_map();
    let schema_ctx = use_schema();
    let navigate = use_navigate();

    let form_data = RwSignal::new(serde_json::Map::new());

    // Resource to fetch the record data
    let data = LocalResource::new(move || async move {
        let p = params.get();
        let entity_info = p.get("entity").zip(p.get("id"));
        if let Some((entity, id)) = entity_info {
            if id == "new" {
                return Ok(Value::Object(serde_json::Map::new()));
            }

            let url = format!("/api/v1/{}/{}", entity, id);
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

    // Sync form data when resource loads
    Effect::new(move |_| {
        if let Some(obj) = data
            .get()
            .and_then(|res| res.as_ref().ok()?.as_object().cloned())
        {
            form_data.set(obj);
        }
    });

    let nav_for_save = navigate.clone();
    let on_save = move |_| {
        let p = params.get_untracked();
        let entity = p.get("entity").map(|s| s.to_string());
        let id = p.get("id").map(|s| s.to_string());
        let current_data = form_data.get_untracked();

        if let (Some(entity), Some(id)) = (entity, id) {
            let navigate = nav_for_save.clone();
            leptos::task::spawn_local(async move {
                let url = if id == "new" {
                    format!("/api/v1/{}", entity)
                } else {
                    format!("/api/v1/{}/{}", entity, id)
                };
                let method = if id == "new" { "POST" } else { "PUT" };

                let resp = auth_fetch(&url, method, Some(Value::Object(current_data))).await;
                match resp {
                    Ok(r) if r.ok() => {
                        navigate(&format!("/admin/collection/{}", entity), Default::default());
                    }
                    Ok(r) => {
                        leptos::logging::error!("Save failed: {}", r.status());
                    }
                    Err(e) => {
                        leptos::logging::error!("Network error: {}", e);
                    }
                }
            });
        }
    };

    let nav_for_cancel = navigate.clone();
    let on_cancel = move |_| {
        let p = params.get_untracked();
        if let Some(entity) = p.get("entity") {
            nav_for_cancel(&format!("/admin/collection/{}", entity), Default::default());
        }
    };

    view! {
        <div class="max-w-4xl mx-auto space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h2 class="text-2xl font-heading font-bold text-foreground">
                        {move || {
                            let p = params.get();
                            let entity = p.get("entity").unwrap_or_default();
                            let id = p.get("id").unwrap_or_default();
                            if id == "new" {
                                format!("Create New {}", entity)
                            } else {
                                format!("Edit {}", entity)
                            }
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
                    <button
                        on:click=on_cancel
                        class="px-4 py-2 border border-border text-foreground hover:bg-muted transition-colors duration-100 font-heading font-medium"
                    >
                        "Cancel"
                    </button>
                    <button
                        on:click=on_save
                        class="px-4 py-2 bg-primary text-primary-foreground font-heading font-semibold hover:bg-primary/90 transition-colors duration-100"
                    >
                        "Save Changes"
                    </button>
                </div>
            </div>

            <div class="bg-surface border border-border p-8">
                <Suspense fallback=move || view! { <div class="text-center text-muted-foreground font-mono">"Loading form..."</div> }>
                    {move || {
                        data.get().map(|res| {
                            match &*res {
                                Ok(_) => {
                                    let entity_name = params.get().get("entity").unwrap_or_default();
                                    let schemas = schema_ctx.schemas.get();
                                    let schema = schemas.and_then(|s| s.as_ref().ok().cloned()).and_then(|ss| {
                                        ss.into_iter().find(|s| s.table_name == entity_name)
                                    });

                                    if let Some(schema) = schema {
                                        view! {
                                            <div class="space-y-6">
                                                {schema.fields.iter().filter(|f| !f.hidden && f.name != "id").map(|f| {
                                                    let field_name = f.name.clone();
                                                    let field_type = f.field_type.clone();
                                                    let ui_widget = f.ui_widget.clone();
                                                    let field_name_for_label = f.name.clone();

                                                    view! {
                                                        <div class="group space-y-1">
                                                            <ForgeLabel text=field_name.clone() />
                                                            {
                                                                let field_name_inner = field_name.clone();
                                                                let field_name_for_callback = field_name.clone();
                                                                let field_type_inner = field_type.clone();
                                                                match (field_type_inner, ui_widget.as_deref()) {
                                                                    (FieldType::Boolean, _) => {
                                                                        view! {
                                                                            <ForgeCheckbox
                                                                                checked=Signal::derive(move || {
                                                                                    form_data.get().get(&field_name_inner).and_then(|v| v.as_bool()).unwrap_or(false)
                                                                                })
                                                                                on_change=Callback::new(move |v: bool| {
                                                                                    form_data.update(|map| { map.insert(field_name_for_callback.clone(), Value::Bool(v)); });
                                                                                })
                                                                            />
                                                                        }.into_any()
                                                                    }
                                                                    (_, Some("textarea")) => {
                                                                        view! {
                                                                            <ForgeTextarea
                                                                                value=Signal::derive(move || {
                                                                                    form_data.get().get(&field_name_inner).and_then(|v| v.as_str()).unwrap_or_default().to_string()
                                                                                })
                                                                                on_change=Callback::new(move |v: String| {
                                                                                    form_data.update(|map| { map.insert(field_name_for_callback.clone(), Value::String(v)); });
                                                                                })
                                                                            />
                                                                        }.into_any()
                                                                    }
                                                                    (FieldType::Link { ref target }, _) => {
                                                                        let target_entity = StoredValue::new(target.clone());
                                                                        let is_open = RwSignal::new(false);
                                                                        let field_name = StoredValue::new(field_name_inner.clone());
                                                                        let field_name_cb = StoredValue::new(field_name_for_callback.clone());
                                                                        let label = StoredValue::new(field_name_for_label.clone());

                                                                        view! {
                                                                            <div
                                                                                on:click=move |_| is_open.set(true)
                                                                                on:focusin=move |_| is_open.set(true)
                                                                            >
                                                                                <Show when=move || !is_open.get()>
                                                                                    <ForgeSelect
                                                                                        value=Signal::derive(move || {
                                                                                            form_data.get().get(&field_name.get_value())
                                                                                                .map(|v| match v {
                                                                                                    Value::String(s) => s.clone(),
                                                                                                    Value::Null => String::new(),
                                                                                                    other => other.to_string(),
                                                                                                })
                                                                                                .unwrap_or_default()
                                                                                        })
                                                                                        on_change=Callback::new(move |v: String| {
                                                                                            let json_val = v.parse::<i64>().map(Value::from).unwrap_or(Value::String(v));
                                                                                            form_data.update(|map| { map.insert(field_name_cb.get_value(), json_val); });
                                                                                        })
                                                                                        options=Signal::derive(Vec::new)
                                                                                        placeholder=format!("Select {}...", label.get_value())
                                                                                    />
                                                                                </Show>
                                                                                <Show when=move || is_open.get()>
                                                                                    {
                                                                                        let link_options = LocalResource::new(move || {
                                                                                            let entity = target_entity.get_value();
                                                                                            async move {
                                                                                                let url = format!("/api/v1/{}", entity);
                                                                                                let resp = auth_fetch(&url, "GET", None::<()>).await.ok()?;
                                                                                                let items: Vec<Value> = resp.json().await.ok()?;
                                                                                                Some(items.into_iter().filter_map(|item| {
                                                                                                    let id = item.get("id")?.to_string();
                                                                                                    let label = item.get("name")
                                                                                                        .or_else(|| item.get("title"))
                                                                                                        .and_then(|v| v.as_str())
                                                                                                        .unwrap_or(&id)
                                                                                                        .to_string();
                                                                                                    Some((id, label))
                                                                                                }).collect::<Vec<(String, String)>>())
                                                                                            }
                                                                                        });
                                                                                        view! {
                                                                                            <Suspense fallback=move || view! { <span class="text-muted-foreground text-xs font-mono">"Loading options..."</span> }>
                                                                                                {move || {
                                                                                                    let opts = link_options.get()
                                                                                                        .and_then(|r| (*r).clone())
                                                                                                        .unwrap_or_default();
                                                                                                    let opts_signal = Signal::derive(move || opts.clone());
                                                                                                    view! {
                                                                                                        <ForgeSelect
                                                                                                            value=Signal::derive(move || {
                                                                                                                form_data.get().get(&field_name.get_value())
                                                                                                                    .map(|v| match v {
                                                                                                                        Value::String(s) => s.clone(),
                                                                                                                        Value::Null => String::new(),
                                                                                                                        other => other.to_string(),
                                                                                                                    })
                                                                                                                    .unwrap_or_default()
                                                                                                            })
                                                                                                            on_change=Callback::new(move |v: String| {
                                                                                                                let json_val = v.parse::<i64>().map(Value::from).unwrap_or(Value::String(v));
                                                                                                                form_data.update(|map| { map.insert(field_name_cb.get_value(), json_val); });
                                                                                                            })
                                                                                                            options=opts_signal
                                                                                                            placeholder=format!("Select {}...", label.get_value())
                                                                                                        />
                                                                                                    }
                                                                                                }}
                                                                                            </Suspense>
                                                                                        }
                                                                                    }
                                                                                </Show>
                                                                            </div>
                                                                        }.into_any()
                                                                    }
                                                                    (field_type_inner, _) => {
                                                                        let input_type = match field_type_inner {
                                                                            FieldType::Integer | FieldType::Float => "number",
                                                                            FieldType::DateTime => "datetime-local",
                                                                            _ => "text",
                                                                        };
                                                                        view! {
                                                                            <ForgeInput
                                                                                type_=input_type
                                                                                value=Signal::derive(move || {
                                                                                    let val = form_data.get().get(&field_name_inner).cloned().unwrap_or(Value::Null);
                                                                                    match val {
                                                                                        Value::String(s) => s,
                                                                                        Value::Number(n) => n.to_string(),
                                                                                        Value::Bool(b) => b.to_string(),
                                                                                        _ => "".to_string(),
                                                                                    }
                                                                                })
                                                                                on_change=Callback::new(move |v: String| {
                                                                                    let json_val = match field_type_inner {
                                                                                        FieldType::Integer => v.parse::<i64>().map(Value::from).unwrap_or(Value::Null),
                                                                                        FieldType::Float => v.parse::<f64>().map(Value::from).unwrap_or(Value::Null),
                                                                                        _ => Value::String(v),
                                                                                    };
                                                                                    form_data.update(|map| { map.insert(field_name_for_callback.clone(), json_val); });
                                                                                })
                                                                            />
                                                                        }.into_any()
                                                                    }
                                                                }
                                                            }
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
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

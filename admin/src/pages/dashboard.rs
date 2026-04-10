use crate::components::stat_card::StatCard;
use crate::context::schema_ctx::use_schema;
use leptos::prelude::*;

/// Dashboard overview page showing CMS statistics.
///
/// Displays stat cards derived from the global schema context,
/// providing a quick overview of registered collections.
#[component]
pub fn Dashboard() -> impl IntoView {
    let schema_ctx = use_schema();

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-heading font-bold text-foreground">"Dashboard"</h1>
                <p class="text-sm text-muted-foreground mt-1">"Overview of your content management system."</p>
            </div>

            <Suspense fallback=move || view! {
                <div class="text-muted-foreground text-sm font-mono">"Loading statistics..."</div>
            }>
                {move || {
                    schema_ctx.schemas.get().map(|res| {
                        match &*res {
                            Ok(schemas) => {
                                let collection_count = schemas.len().to_string();
                                let field_count: usize = schemas.iter().map(|s| s.fields.len()).sum();
                                view! {
                                    <div class="grid grid-cols-3 gap-6">
                                        <StatCard
                                            title="Collections"
                                            value=collection_count
                                            subtitle="Registered entity types"
                                        />
                                        <StatCard
                                            title="Total Fields"
                                            value=field_count.to_string()
                                            subtitle="Across all schemas"
                                        />
                                        <StatCard
                                            title="API Status"
                                            value="Active"
                                            subtitle="All endpoints operational"
                                        />
                                    </div>
                                }.into_any()
                            }
                            Err(e) => {
                                let e = e.clone();
                                view! {
                                    <div class="text-destructive font-mono">{e}</div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

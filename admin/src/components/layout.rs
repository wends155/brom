use crate::context::schema_ctx::use_schema;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Layout() -> impl IntoView {
    let schema_ctx = use_schema();
    let navigate = use_navigate();

    let nav_for_schemas = navigate.clone();
    let nav_for_system = navigate.clone();

    view! {
        <div class="flex h-screen bg-background text-foreground">
            // Sidebar
            <aside class="w-64 border-r bg-muted/30 flex flex-col">
                <div class="p-6 border-b">
                    <h1 class="text-xl font-bold tracking-tight">"Brom Admin"</h1>
                </div>

                <nav class="flex-1 overflow-y-auto p-4 space-y-1">
                    <div class="px-3 py-2 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                        "Collections"
                    </div>

                    <Suspense fallback=move || view! { <div class="px-3 py-2 text-sm text-muted-foreground italic">"Loading..."</div> }>
                        {
                            let navigate = nav_for_schemas.clone();
                            move || {
                                let navigate = navigate.clone();
                                schema_ctx.schemas.get().map(|res| {
                                    match &*res {
                                        Ok(schemas) => {
                                            schemas.into_iter().map(|s| {
                                                let navigate = navigate.clone();
                                                let table_name = s.table_name.clone();
                                                view! {
                                                    <a
                                                        href=format!("/admin/collection/{}", table_name)
                                                        on:click={
                                                            let navigate = navigate.clone();
                                                            let url = format!("/admin/collection/{}", table_name);
                                                            move |ev| {
                                                                ev.prevent_default();
                                                                navigate(&url, Default::default());
                                                            }
                                                        }
                                                        class="flex items-center px-3 py-2 text-sm font-medium rounded-md hover:bg-accent hover:text-accent-foreground transition-colors"
                                                    >
                                                        {table_name.clone()}
                                                    </a>
                                                }
                                            }).collect::<Vec<_>>().into_any()
                                        }
                                        Err(_) => view! { <div class="px-3 py-2 text-sm text-destructive">"Error loading schema"</div> }.into_any()
                                    }
                                })
                            }
                        }
                    </Suspense>

                    <div class="pt-4 px-3 py-2 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                        "System"
                    </div>
                    <a
                        href="/admin/api-keys"
                        on:click={
                            let navigate = nav_for_system.clone();
                            move |ev| {
                                ev.prevent_default();
                                navigate("/admin/api-keys", Default::default());
                            }
                        }
                        class="flex items-center px-3 py-2 text-sm font-medium rounded-md hover:bg-accent hover:text-accent-foreground transition-colors"
                    >
                        "API Keys"
                    </a>
                </nav>

                <div class="p-4 border-t">
                    <button
                        class="w-full flex items-center px-3 py-2 text-sm font-medium rounded-md text-destructive hover:bg-destructive/10 transition-colors"
                    >
                        "Logout"
                    </button>
                </div>
            </aside>

            // Main Content
            <main class="flex-1 overflow-y-auto p-8">
                <Outlet />
            </main>
        </div>
    }
}

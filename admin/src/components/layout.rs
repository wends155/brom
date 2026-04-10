use crate::components::breadcrumbs::Breadcrumbs;
use crate::context::schema_ctx::use_schema;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::{use_location, use_navigate};

#[component]
pub fn Layout() -> impl IntoView {
    let schema_ctx = use_schema();
    let navigate = use_navigate();
    let location = use_location();

    let nav_for_schemas = navigate.clone();
    let nav_for_system = navigate.clone();

    view! {
        <div class="flex h-screen bg-background text-foreground font-body">
            // Sidebar (w-56)
            <aside class="w-56 border-r border-border bg-surface flex flex-col">
                // Monogram header
                <div class="p-4 border-b border-border">
                    <div class="w-10 h-10 bg-primary flex items-center justify-center">
                        <span class="text-primary-foreground font-heading font-bold text-xl">"B"</span>
                    </div>
                </div>

                // Navigation
                <nav class="flex-1 overflow-y-auto py-4">
                    // "Collections" section label
                    <div class="px-6 py-2 text-xs font-semibold text-muted-foreground uppercase tracking-wider font-heading">
                        "Collections"
                    </div>

                    <Suspense fallback=move || view! { <div class="px-6 py-2 text-sm text-muted-foreground italic font-mono">"Loading..."</div> }>
                        {
                            let navigate = nav_for_schemas.clone();
                            let location = location.clone();
                            move || {
                                let navigate = navigate.clone();
                                let location = location.clone();
                                schema_ctx.schemas.get().map(move |res| {
                                    match &*res {
                                        Ok(schemas) => {
                                            schemas.iter().map(|s| {
                                                let navigate = navigate.clone();
                                                let table_name = s.table_name.clone();
                                                let url = format!("/admin/collection/{}", table_name);
                                                let active_class = move || {
                                                    if location.pathname.get().starts_with(&url) {
                                                        "flex items-center px-6 py-2 text-sm font-medium font-heading hover:bg-accent hover:text-accent-foreground transition-colors forge-nav-active text-foreground"
                                                    } else {
                                                        "flex items-center px-6 py-2 text-sm font-medium font-heading hover:bg-accent hover:text-accent-foreground transition-colors text-muted-foreground border-l-[3px] border-transparent"
                                                    }
                                                };
                                                view! {
                                                    <a
                                                        href=url.clone()
                                                        on:click={
                                                            let navigate = navigate.clone();
                                                            let url = url.clone();
                                                            move |ev| {
                                                                ev.prevent_default();
                                                                navigate(&url, Default::default());
                                                            }
                                                        }
                                                        class=active_class
                                                    >
                                                        {table_name.clone()}
                                                    </a>
                                                }
                                            }).collect::<Vec<_>>().into_any()
                                        }
                                        Err(_) => view! { <div class="px-6 py-2 text-sm text-destructive font-mono">"Error loading schema"</div> }.into_any()
                                    }
                                })
                            }
                        }
                    </Suspense>

                    // "System" section label
                    <div class="pt-6 px-6 py-2 text-xs font-semibold text-muted-foreground uppercase tracking-wider font-heading">
                        "System"
                    </div>
                    {
                        let location = location.clone();
                        let active_class = move || {
                            if location.pathname.get().starts_with("/admin/api-keys") {
                                "flex items-center px-6 py-2 text-sm font-medium font-heading hover:bg-accent hover:text-accent-foreground transition-colors forge-nav-active text-foreground"
                            } else {
                                "flex items-center px-6 py-2 text-sm font-medium font-heading hover:bg-accent hover:text-accent-foreground transition-colors text-muted-foreground border-l-[3px] border-transparent"
                            }
                        };
                        view! {
                            <a
                                href="/admin/api-keys"
                                on:click={
                                    let navigate = nav_for_system.clone();
                                    move |ev| {
                                        ev.prevent_default();
                                        navigate("/admin/api-keys", Default::default());
                                    }
                                }
                                class=active_class
                            >
                                "API Keys"
                            </a>
                        }
                    }
                </nav>
            </aside>

            // Main area
            <div class="flex-1 flex flex-col overflow-hidden">
                // Top bar
                <header class="h-14 border-b border-border bg-surface flex items-center justify-between px-6">
                    <Breadcrumbs />
                    <div class="flex items-center space-x-4">
                        // User avatar circle
                        <div class="w-8 h-8 bg-primary/20 border border-primary/30 flex items-center justify-center">
                            <span class="text-primary text-xs font-bold font-heading">"A"</span>
                        </div>
                        <span class="text-sm text-muted-foreground font-heading">"Admin"</span>
                        <button class="text-sm text-muted-foreground hover:text-destructive transition-colors duration-100 font-heading">
                            "Logout"
                        </button>
                    </div>
                </header>

                // Content
                <main class="flex-1 overflow-y-auto p-8">
                    <div class="forge-enter">
                        <Outlet />
                    </div>
                </main>
            </div>
        </div>
    }
}

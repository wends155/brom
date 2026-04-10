use crate::components::layout::Layout;
use crate::context::schema_ctx::provide_schema_context;
use crate::pages::collection_list::CollectionList;
use crate::pages::dashboard::Dashboard;
use crate::pages::form_editor::EditorForm;
use crate::pages::login::Login;
use crate::pages::settings::Settings;
use leptos::prelude::*;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Provides metadata (title, etc.)
    provide_meta_context();

    // Provide global schema context
    provide_schema_context();

    view! {
        <Title text="Brom Admin" />
        <Router>
            <Routes fallback=|| view! { "Page not found." }>
                <Route path=path!("/admin/login") view=Login />

                <ParentRoute path=path!("/admin") view=Layout>
                    <Route path=path!("") view=Dashboard />
                    <Route path=path!("collection/:entity") view=CollectionList />
                    <Route path=path!("collection/:entity/:id") view=EditorForm />
                    <Route path=path!("api-keys") view=Settings />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

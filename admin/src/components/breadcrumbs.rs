use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

/// Renders breadcrumb navigation from the current route path.
///
/// Splits the URL path into displayable segments and renders
/// clickable links separated by `>` chevrons. The first crumb
/// is always "Dashboard" linking to `/admin`.
#[component]
pub fn Breadcrumbs() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    view! {
        <nav class="flex items-center space-x-2 text-sm text-muted-foreground font-body">
            {move || {
                let path = location.pathname.get();
                let segments: Vec<&str> = path
                    .trim_start_matches('/')
                    .split('/')
                    .filter(|s| !s.is_empty() && *s != "admin")
                    .collect();

                let mut crumbs = vec![("Dashboard".to_string(), "/admin".to_string())];

                let mut accumulated = "/admin".to_string();
                for seg in &segments {
                    accumulated = format!("{}/{}", accumulated, seg);
                    let label = seg.replace(['-', '_'], " ");
                    // Capitalize first letter
                    let label = label
                        .split_whitespace()
                        .map(|w| {
                            let mut chars = w.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                    crumbs.push((label, accumulated.clone()));
                }

                let last_idx = crumbs.len().saturating_sub(1);

                crumbs
                    .into_iter()
                    .enumerate()
                    .map(|(i, (label, href))| {
                        let is_last = i == last_idx;
                        let navigate = navigate.clone();
                        view! {
                            {if i > 0 {
                                Some(view! { <span class="text-muted-foreground/50">"›"</span> })
                            } else {
                                None
                            }}
                            {if is_last {
                                view! { <span class="text-foreground font-medium">{label}</span> }.into_any()
                            } else {
                                view! {
                                    <a
                                        href=href.clone()
                                        on:click={
                                            let navigate = navigate.clone();
                                            let href = href.clone();
                                            move |ev| {
                                                ev.prevent_default();
                                                navigate(&href, Default::default());
                                            }
                                        }
                                        class="hover:text-foreground transition-colors duration-100"
                                    >
                                        {label}
                                    </a>
                                }.into_any()
                            }}
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </nav>
    }
}

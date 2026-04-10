use leptos::prelude::*;

/// Dashboard stat card with numeric value emphasis.
///
/// Renders a bordered card with the value prominently displayed
/// in `JetBrains Mono` and a muted title and subtitle.
#[component]
pub fn StatCard(
    #[prop(into)] title: String,
    #[prop(into)] value: String,
    #[prop(into)] subtitle: String,
) -> impl IntoView {
    view! {
        <div class="border border-border p-6 bg-surface">
            <p class="text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground mb-2">
                {title}
            </p>
            <p class="text-3xl font-mono font-bold text-foreground mb-1">
                {value}
            </p>
            <p class="text-sm text-muted-foreground">
                {subtitle}
            </p>
        </div>
    }
}

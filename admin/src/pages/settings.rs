use leptos::prelude::*;

/// API key management page.
///
/// Displays a table of API keys with their metadata and provides
/// controls for creating new keys and revoking existing ones.
/// Currently renders mock data; real API integration is deferred to Phase 5B.
#[component]
pub fn Settings() -> impl IntoView {
    // Mock data for UI development
    let mock_keys = vec![
        (
            "brom_8a2f...".to_string(),
            "Production API".to_string(),
            "read_write".to_string(),
            "Active".to_string(),
            "2026-04-01".to_string(),
        ),
        (
            "brom_c4e1...".to_string(),
            "Staging Read".to_string(),
            "read".to_string(),
            "Active".to_string(),
            "2026-03-15".to_string(),
        ),
        (
            "brom_91ab...".to_string(),
            "Legacy Script".to_string(),
            "read".to_string(),
            "Revoked".to_string(),
            "2026-01-10".to_string(),
        ),
    ];

    view! {
        <div class="space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-2xl font-heading font-bold text-foreground">"API Keys"</h1>
                    <p class="text-sm text-muted-foreground mt-1">"Manage programmatic access to your CMS."</p>
                </div>
                <button class="bg-primary text-primary-foreground px-4 py-2 font-heading font-semibold hover:bg-primary/90 transition-colors duration-100">
                    "Create New Key"
                </button>
            </div>

            <div class="border border-border overflow-hidden">
                <table class="w-full text-sm text-left font-mono">
                    <thead class="bg-surface border-b border-border">
                        <tr>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Key Prefix"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Name"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Permissions"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Status"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">"Created"</th>
                            <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground text-right">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {mock_keys.into_iter().enumerate().map(|(i, (prefix, name, perms, status, created))| {
                            let stripe = if i % 2 == 0 { "" } else { "bg-surface/30" };
                            let status_class = if status == "Active" {
                                "bg-success/20 text-success px-2 py-0.5 text-xs font-semibold"
                            } else {
                                "bg-destructive/20 text-destructive px-2 py-0.5 text-xs font-semibold"
                            };
                            let is_revoked = status == "Revoked";
                            view! {
                                <tr class=format!("border-b border-border/50 hover:bg-accent transition-colors duration-100 {}", stripe)>
                                    <td class="px-4 py-3 text-foreground">{prefix}</td>
                                    <td class="px-4 py-3 text-foreground">{name}</td>
                                    <td class="px-4 py-3 text-foreground">{perms}</td>
                                    <td class="px-4 py-3">
                                        <span class=status_class>{status}</span>
                                    </td>
                                    <td class="px-4 py-3 text-muted-foreground">{created}</td>
                                    <td class="px-4 py-3 text-right">
                                        {if !is_revoked {
                                            Some(view! {
                                                <button class="text-destructive hover:text-destructive/80 font-medium text-sm transition-colors duration-100">
                                                    "Revoke"
                                                </button>
                                            })
                                        } else {
                                            None
                                        }}
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
                <div class="px-4 py-3 bg-surface/50 border-t border-border text-xs text-muted-foreground font-mono flex justify-between items-center">
                    <span>"1–3 of 3 items"</span>
                    <span class="text-muted-foreground/50">"api_keys"</span>
                </div>
            </div>
        </div>
    }
}

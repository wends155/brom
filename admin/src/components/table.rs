use leptos::prelude::*;

/// Generic data table with Forge Dark styling.
///
/// Renders a dense table with `JetBrains Mono` font for data rows,
/// alternating micro-stripe backgrounds, and a pagination info footer.
#[component]
pub fn DataTable(
    #[prop(into)] headers: Vec<String>,
    #[prop(into)] rows: Vec<Vec<String>>,
    #[prop(into)] entity_name: String,
    #[prop(optional)] total_count: Option<usize>,
) -> impl IntoView {
    let row_count = rows.len();
    let total = total_count.unwrap_or(row_count);

    view! {
        <div class="border border-border overflow-hidden">
            <table class="w-full text-sm text-left font-mono">
                <thead class="bg-surface border-b border-border">
                    <tr>
                        {headers.iter().map(|h| {
                            view! {
                                <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground">
                                    {h.clone()}
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                        <th class="px-4 py-3 text-xs font-heading font-semibold uppercase tracking-wider text-muted-foreground text-right">
                            "Actions"
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {rows.into_iter().enumerate().map(|(i, row)| {
                        let stripe = if i % 2 == 0 { "" } else { "bg-surface/30" };
                        let edit_href = if !row.is_empty() {
                            format!("/admin/collection/{}/{}", entity_name, row[0])
                        } else {
                            "#".to_string()
                        };
                        view! {
                            <tr class=format!("border-b border-border/50 hover:bg-accent transition-colors duration-100 {}", stripe)>
                                {row.into_iter().map(|cell| {
                                    view! {
                                        <td class="px-4 py-3 text-foreground">
                                            {cell}
                                        </td>
                                    }
                                }).collect::<Vec<_>>()}
                                <td class="px-4 py-3 text-right">
                                    <a
                                        href=edit_href
                                        class="text-primary hover:text-primary/80 font-medium transition-colors duration-100"
                                    >
                                        "Edit"
                                    </a>
                                </td>
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
            <div class="px-4 py-3 bg-surface/50 border-t border-border text-xs text-muted-foreground font-mono flex justify-between items-center">
                <span>{format!("1–{} of {} items", row_count, total)}</span>
                <span class="text-muted-foreground/50">{entity_name}</span>
            </div>
        </div>
    }
}

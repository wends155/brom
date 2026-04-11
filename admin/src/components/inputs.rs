use leptos::prelude::*;

#[component]
pub fn ForgeLabel(
    #[prop(into)] text: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <label class=format!("block text-xs font-bold uppercase tracking-wider text-muted-foreground mb-1.5 {}", class)>
            {text}
        </label>
    }
}

#[component]
pub fn ForgeInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] type_: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let t = if type_.is_empty() {
        "text".to_string()
    } else {
        type_
    };
    view! {
        <input
            type=t
            class=format!("bg-input border border-border px-3 py-2 text-sm font-mono text-foreground forge-focus w-full disabled:cursor-not-allowed disabled:opacity-50 {}", class)
            placeholder=placeholder
            prop:value=move || value.get()
            disabled=move || disabled.get()
            on:input=move |ev| {
                on_change.run(event_target_value(&ev));
            }
        />
    }
}

#[component]
pub fn ForgeCheckbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            class="w-4 h-4 rounded-none bg-input border-border text-primary focus:ring-primary focus:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50"
            prop:checked=move || checked.get()
            disabled=move || disabled.get()
            on:change=move |ev| {
                on_change.run(event_target_checked(&ev));
            }
        />
    }
}

#[component]
pub fn ForgeTextarea(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] rows: u32,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let r = if rows == 0 { 4 } else { rows };
    view! {
        <textarea
            class=format!("bg-input border border-border px-3 py-2 text-sm font-mono text-foreground forge-focus w-full disabled:cursor-not-allowed disabled:opacity-50 min-h-[100px] {}", class)
            placeholder=placeholder
            prop:value=move || value.get()
            disabled=move || disabled.get()
            rows=r
            on:input=move |ev| {
                on_change.run(event_target_value(&ev));
            }
        />
    }
}

#[component]
pub fn ForgeSelect(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] options: Signal<Vec<(String, String)>>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <select
            class=format!("bg-input border border-border px-3 py-2 text-sm font-mono text-foreground forge-focus w-full disabled:cursor-not-allowed disabled:opacity-50 {}", class)
            disabled=move || disabled.get()
            on:change=move |ev| {
                on_change.run(event_target_value(&ev));
            }
            prop:value=move || value.get()
        >
            <option value="" disabled selected=move || value.get().is_empty()>
                {if placeholder.is_empty() { "Select...".to_string() } else { placeholder.clone() }}
            </option>
            {move || options.get().into_iter().map(|(val, label)| {
                let selected = value.get() == val;
                view! {
                    <option value=val selected=selected>{label}</option>
                }
            }).collect::<Vec<_>>()}
        </select>
    }
}

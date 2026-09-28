//! Mentions Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{SelectionSize, MentionTrigger, MentionOption};
use crate::try_use_theme;

/// Mentions component.
///
/// A textarea with @mention support, showing suggestions when typing
/// trigger characters (like @, #, or /).
///
/// # Props
///
/// - `value` - Controlled text content signal
/// - `options` - Available mention options
/// - `triggers` - Trigger characters (default: '@')
/// - `placeholder` - Placeholder text
/// - `size` - Component size (Sm, Md, Lg)
/// - `disabled` - Whether the input is disabled
/// - `readonly` - Whether the input is readonly
/// - `rows` - Number of visible text rows
/// - `max_length` - Maximum text length
/// - `auto_size` - Auto-resize based on content
/// - `loading` - Show loading indicator
/// - `filter_option` - Custom filter function
/// - `class` - Additional CSS classes
/// - `on_change` - Text change handler
/// - `on_select` - Mention selection handler
/// - `on_search` - Search/filter handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{Mentions, MentionOption, MentionTrigger};
///
/// let text = RwSignal::new(String::new());
///
/// let users = vec![
///     MentionOption::new("jdoe", "John Doe").with_avatar("/avatars/jdoe.png"),
///     MentionOption::new("jane", "Jane Smith").with_avatar("/avatars/jane.png"),
/// ];
///
/// view! {
///     <Mentions
///         value=text
///         options=users
///         placeholder="Type @ to mention someone..."
///         on_select=move |opt| log::info!("Mentioned: {}", opt.value)
///     />
/// }
/// ```
#[component]
pub fn Mentions(
    /// Controlled text content.
    #[prop(into)]
    value: RwSignal<String>,
    /// Available mention options.
    #[prop(into)]
    options: Vec<MentionOption>,
    /// Trigger characters.
    #[prop(optional, into)]
    triggers: Option<Vec<MentionTrigger>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Component size.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Whether disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether readonly.
    #[prop(optional)]
    readonly: bool,
    /// Visible text rows.
    #[prop(optional)]
    #[prop(default = 3)]
    rows: u32,
    /// Maximum text length.
    #[prop(optional)]
    max_length: Option<usize>,
    /// Auto-resize.
    #[prop(optional)]
    auto_size: bool,
    /// Show loading.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Text change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Selection handler.
    #[prop(optional, into)]
    on_select: Option<Callback<MentionOption>>,
    /// Search handler.
    #[prop(optional, into)]
    on_search: Option<Callback<(char, String)>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let triggers = triggers.unwrap_or_else(|| vec![MentionTrigger::default()]);

    // Internal state
    let is_suggesting = RwSignal::new(false);
    let search_text = RwSignal::new(String::new());
    let active_trigger = RwSignal::new(None::<char>);
    let cursor_position = RwSignal::new(0usize);
    let trigger_start = RwSignal::new(0usize);
    let highlighted_index = RwSignal::new(0i32);

    // Store options and triggers
    let options_stored = StoredValue::new(options.clone());
    let triggers_stored = StoredValue::new(triggers.clone());

    // Filter options based on search
    let filtered_options = Memo::new(move |_| {
        let search = search_text.get().to_lowercase();
        if search.is_empty() {
            options_stored.get_value()
        } else {
            options_stored.get_value()
                .into_iter()
                .filter(|opt| {
                    opt.label.to_lowercase().contains(&search) ||
                    opt.value.to_lowercase().contains(&search)
                })
                .collect()
        }
    });

    // Build CSS classes
    let mentions_prefix = format!("fx-mentions-{}", design_system);

    let wrapper_class = {
        let prefix = mentions_prefix.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                size.class(&prefix),
            ];

            if disabled {
                parts.push(format!("{}-disabled", prefix));
            }
            if readonly {
                parts.push(format!("{}-readonly", prefix));
            }
            if is_suggesting.get() {
                parts.push(format!("{}-suggesting", prefix));
            }
            if loading {
                parts.push(format!("{}-loading", prefix));
            }

            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }

            parts.join(" ")
        }
    };

    // Check if character is a trigger
    let is_trigger_char = move |c: char| -> bool {
        triggers_stored.get_value().iter().any(|t| t.character == c)
    };

    // Handle input change
    let on_change_clone = on_change.clone();
    let on_search_clone = on_search.clone();
    let handle_input = move |ev: ev::Event| {
        if disabled || readonly {
            return;
        }

        let input_value = event_target_value(&ev);
        value.set(input_value.clone());

        if let Some(ref cb) = on_change_clone {
            cb.run(input_value.clone());
        }

        // Check for trigger character
        // This is a simplified implementation - a real one would track cursor position
        let chars: Vec<char> = input_value.chars().collect();
        let len = chars.len();

        // Look for trigger pattern: trigger_char followed by word characters
        if len > 0 {
            let mut found_trigger = None;
            let mut search_start = 0;

            // Search backwards from end for trigger
            for i in (0..len).rev() {
                let c = chars[i];

                if c.is_whitespace() {
                    break;
                }

                if is_trigger_char(c) {
                    // Check if at start or preceded by whitespace
                    if i == 0 || chars[i - 1].is_whitespace() {
                        found_trigger = Some(c);
                        search_start = i + 1;
                        break;
                    }
                }
            }

            if let Some(trigger) = found_trigger {
                let search: String = chars[search_start..].iter().collect();
                active_trigger.set(Some(trigger));
                search_text.set(search.clone());
                trigger_start.set(search_start - 1); // Include trigger char position
                is_suggesting.set(true);
                highlighted_index.set(0);

                if let Some(ref cb) = on_search_clone {
                    cb.run((trigger, search));
                }
            } else {
                is_suggesting.set(false);
                active_trigger.set(None);
                search_text.set(String::new());
            }
        } else {
            is_suggesting.set(false);
        }
    };

    // Handle mention selection
    let on_select_clone = on_select.clone();
    let on_change_select = on_change.clone();
    let select_mention = move |option: MentionOption| {
        if option.disabled {
            return;
        }

        let current = value.get();
        let start = trigger_start.get();

        // Replace trigger + search with mention
        let before: String = current.chars().take(start).collect();
        let trigger = active_trigger.get().unwrap_or('@');

        // Find the trigger in triggers to get prefix
        let prefix = triggers_stored.get_value()
            .iter()
            .find(|t| t.character == trigger)
            .and_then(|t| t.prefix.clone())
            .unwrap_or_default();

        let mention_text = format!("{}{}{} ", trigger, prefix, option.value);
        let new_value = format!("{}{}", before, mention_text);

        value.set(new_value.clone());
        is_suggesting.set(false);
        active_trigger.set(None);
        search_text.set(String::new());

        if let Some(ref cb) = on_select_clone {
            cb.run(option);
        }
        if let Some(ref cb) = on_change_select {
            cb.run(new_value);
        }
    };

    // Handle keyboard navigation
    let select_mention_kb = select_mention.clone();
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if !is_suggesting.get() {
            return;
        }

        let options = filtered_options.get();
        let options_len = options.len() as i32;

        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                if options_len > 0 {
                    highlighted_index.update(|i| {
                        *i = (*i + 1).min(options_len - 1);
                    });
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                if options_len > 0 {
                    highlighted_index.update(|i| {
                        *i = (*i - 1).max(0);
                    });
                }
            }
            "Enter" => {
                if options_len > 0 {
                    ev.prevent_default();
                    let idx = highlighted_index.get() as usize;
                    if let Some(opt) = options.get(idx) {
                        select_mention_kb(opt.clone());
                    }
                }
            }
            "Escape" => {
                ev.prevent_default();
                is_suggesting.set(false);
            }
            _ => {}
        }
    };

    // CSS class names
    let textarea_class = format!("{}-textarea", mentions_prefix);
    let dropdown_class = format!("{}-dropdown", mentions_prefix);
    let option_class = format!("{}-option", mentions_prefix);
    let avatar_class = format!("{}-avatar", mentions_prefix);
    let content_class = format!("{}-content", mentions_prefix);
    let label_class = format!("{}-label", mentions_prefix);
    let desc_class = format!("{}-description", mentions_prefix);
    let loading_class = format!("{}-loading-indicator", mentions_prefix);
    let empty_class = format!("{}-empty", mentions_prefix);

    view! {
        <div class=wrapper_class>
            <textarea
                class=textarea_class
                prop:value=move || value.get()
                placeholder=placeholder.clone()
                disabled=disabled
                readonly=readonly
                rows=rows
                maxlength=max_length.map(|l| l.to_string())
                on:input=handle_input
                on:keydown=handle_keydown
            />

            // Suggestions dropdown
            <div
                class=dropdown_class
                class:hidden=move || !is_suggesting.get()
                role="listbox"
            >
                {move || {
                    if loading {
                        return view! {
                            <div class=loading_class.clone()>
                                <span class=format!("{}-spinner", mentions_prefix)>"⟳"</span>
                                " Loading..."
                            </div>
                        }.into_any();
                    }

                    let options = filtered_options.get();

                    if options.is_empty() {
                        return view! {
                            <div class=empty_class.clone()>
                                "No matches found"
                            </div>
                        }.into_any();
                    }

                    view! {
                        <div>
                            {options.into_iter().enumerate().map(|(idx, opt)| {
                                let opt_clone = opt.clone();
                                let select_handler = select_mention.clone();
                                let is_highlighted = move || highlighted_index.get() == idx as i32;
                                let is_disabled = opt.disabled;

                                let option_item_class = {
                                    let base = option_class.clone();
                                    if is_disabled {
                                        format!("{} {}-disabled", base, option_class)
                                    } else {
                                        base
                                    }
                                };

                                view! {
                                    <div
                                        class=option_item_class
                                        class:highlighted=is_highlighted
                                        role="option"
                                        aria-selected=move || is_highlighted().to_string()
                                        on:mousedown=move |ev| {
                                            ev.prevent_default();
                                            select_handler(opt_clone.clone());
                                        }
                                        on:mouseenter=move |_| {
                                            if !is_disabled {
                                                highlighted_index.set(idx as i32);
                                            }
                                        }
                                    >
                                        // Avatar
                                        {opt.avatar.clone().map(|avatar| view! {
                                            <img
                                                class=avatar_class.clone()
                                                src=avatar
                                                alt=""
                                            />
                                        })}

                                        // Content
                                        <div class=content_class.clone()>
                                            <span class=label_class.clone()>{opt.label.clone()}</span>
                                            {opt.description.clone().map(|desc| view! {
                                                <span class=desc_class.clone()>{desc}</span>
                                            })}
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mention_trigger_default() {
        let trigger = MentionTrigger::default();
        assert_eq!(trigger.character, '@');
    }

    #[test]
    fn test_mention_option_builder() {
        let opt = MentionOption::new("jdoe", "John Doe")
            .with_avatar("/avatar.png")
            .with_description("Engineer");

        assert_eq!(opt.value, "jdoe");
        assert_eq!(opt.label, "John Doe");
        assert_eq!(opt.avatar, Some("/avatar.png".to_string()));
        assert_eq!(opt.description, Some("Engineer".to_string()));
    }
}

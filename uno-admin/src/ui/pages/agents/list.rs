//! Agent list page with database integration

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::handler::{list_agents, create_agent, CreateAgentForm};
use crate::models::entity::AgentEntity;

#[cfg(feature = "hydrate")]
use ember_fx_icons::{Flag, FlagIcon, FlagAspect};

/// Single-value slider component for commission percentage
#[component]
fn CommissionSlider(
    value: RwSignal<f64>,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 10.0)] max: f64,
) -> impl IntoView {
    let slider_ref = NodeRef::<leptos::html::Div>::new();

    let calculate_value = move |client_x: i32| -> f64 {
        if let Some(el) = slider_ref.get() {
            let rect = el.get_bounding_client_rect();
            let x = (client_x as f64 - rect.left()).max(0.0).min(rect.width());
            let pct = x / rect.width();
            let new_value = min + pct * (max - min);
            // Round to 1 decimal place
            (new_value * 10.0).round() / 10.0
        } else {
            value.get()
        }
    };

    let on_mousedown = move |e: web_sys::MouseEvent| {
        e.prevent_default();
        value.set(calculate_value(e.client_x()));

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use std::cell::RefCell;
            use std::rc::Rc;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // Use Rc<RefCell> to store closures so we can remove them later
            let move_closure: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> = Rc::new(RefCell::new(None));
            let up_closure: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> = Rc::new(RefCell::new(None));

            let move_closure_clone = move_closure.clone();
            let up_closure_clone = up_closure.clone();
            let document_clone = document.clone();

            let on_move = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                value.set(calculate_value(e.client_x()));
            }) as Box<dyn FnMut(_)>);

            let on_up = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                // Remove both listeners on mouseup
                if let Some(move_cb) = move_closure_clone.borrow().as_ref() {
                    let _ = document_clone.remove_event_listener_with_callback(
                        "mousemove",
                        move_cb.as_ref().unchecked_ref(),
                    );
                }
                if let Some(up_cb) = up_closure_clone.borrow().as_ref() {
                    let _ = document_clone.remove_event_listener_with_callback(
                        "mouseup",
                        up_cb.as_ref().unchecked_ref(),
                    );
                }
            }) as Box<dyn FnMut(_)>);

            // Add event listeners
            let _ = document.add_event_listener_with_callback(
                "mousemove",
                on_move.as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "mouseup",
                on_up.as_ref().unchecked_ref(),
            );

            // Store closures so they stay alive and can be removed
            *move_closure.borrow_mut() = Some(on_move);
            *up_closure.borrow_mut() = Some(on_up);
        }
    };

    let percentage = move || {
        let v = value.get();
        ((v - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    };

    view! {
        <div
            node_ref=slider_ref
            style="height: 8px; background: #334155; border-radius: 4px; cursor: pointer; position: relative;"
            on:mousedown=on_mousedown
        >
            // Filled track
            <div
                style=move || format!(
                    "position: absolute; left: 0; top: 0; bottom: 0; width: {}%; background: #3b82f6; border-radius: 4px;",
                    percentage()
                )
            ></div>
            // Handle
            <div
                style=move || format!(
                    "position: absolute; top: 50%; left: {}%; transform: translate(-50%, -50%); width: 18px; height: 18px; background: white; border: 2px solid #3b82f6; border-radius: 50%; cursor: grab; box-shadow: 0 2px 4px rgba(0,0,0,0.2);",
                    percentage()
                )
            ></div>
        </div>
    }
}

/// Country dropdown with flag icons
#[component]
fn CountryDropdown(
    value: RwSignal<String>,
) -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = signal(false);
    let (search_query, set_search_query) = signal(String::new());

    #[cfg(feature = "hydrate")]
    {
        // Get selected country info
        let selected_flag = move || {
            let code = value.get();
            if code.is_empty() {
                None
            } else {
                FlagIcon::from_code(&code)
            }
        };

        let selected_name = move || {
            selected_flag().map(|f| f.name()).unwrap_or("Select country...")
        };

        // Filter countries based on search
        let filtered_countries = move || {
            let query = search_query.get().to_lowercase();
            let all_flags: Vec<_> = FlagIcon::all().iter().cloned().collect();

            if query.is_empty() {
                all_flags
            } else {
                all_flags
                    .into_iter()
                    .filter(|f| {
                        f.name().to_lowercase().contains(&query) ||
                        f.code().to_lowercase().contains(&query)
                    })
                    .collect()
            }
        };

        view! {
            <div style="position: relative;">
                // Trigger button
                <button
                    type="button"
                    style="width: 100%; padding: 8px 12px; border-radius: 8px; border: 1px solid #475569; background: #334155; color: #f1f5f9; display: flex; align-items: center; gap: 8px; text-align: left; cursor: pointer;"
                    on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
                >
                    {move || selected_flag().map(|f| view! {
                        <Flag icon=f aspect=FlagAspect::Square style="width: 20px; height: 20px; border-radius: 2px;".to_string() />
                    })}
                    <span style="flex: 1;">{selected_name}</span>
                    <Icon name=IconName::ChevronDown size=16 />
                </button>

                // Dropdown panel
                <Show when=move || show_dropdown.get()>
                    <div style="position: absolute; z-index: 100; margin-top: 4px; width: 100%; max-height: 240px; overflow: auto; border-radius: 8px; border: 1px solid #475569; background: #1e293b; box-shadow: 0 10px 25px rgba(0,0,0,0.3);">
                        // Search input
                        <div style="position: sticky; top: 0; padding: 8px; background: #1e293b; border-bottom: 1px solid #334155;">
                            <input
                                type="text"
                                style="width: 100%; padding: 6px 10px; font-size: 14px; border-radius: 6px; border: 1px solid #475569; background: #334155; color: #f1f5f9;"
                                placeholder="Search countries..."
                                prop:value=move || search_query.get()
                                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                            />
                        </div>

                        // Country list
                        <div style="padding: 4px 0;">
                            {move || filtered_countries().into_iter().map(|flag| {
                                let code = flag.code().to_string();
                                let code_clone = code.clone();
                                let name = flag.name();
                                let is_selected = value.get() == code;
                                view! {
                                    <button
                                        type="button"
                                        style=move || format!(
                                            "width: 100%; padding: 8px 12px; display: flex; align-items: center; gap: 8px; text-align: left; cursor: pointer; border: none; background: {}; color: #f1f5f9;",
                                            if is_selected { "#3b82f6" } else { "transparent" }
                                        )
                                        on:click=move |_| {
                                            value.set(code_clone.clone());
                                            set_show_dropdown.set(false);
                                            set_search_query.set(String::new());
                                        }
                                    >
                                        <Flag icon=flag aspect=FlagAspect::Square style="width: 20px; height: 20px; border-radius: 2px;".to_string() />
                                        <span style="font-size: 14px;">{name}</span>
                                        <span style="font-size: 12px; color: #94a3b8; margin-left: auto;">{code.to_uppercase()}</span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </Show>
            </div>
        }.into_any()
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // SSR fallback - simple text input
        view! {
            <input
                type="text"
                style="width: 100%; padding: 8px 12px; border-radius: 8px; border: 1px solid #475569; background: #334155; color: #f1f5f9;"
                placeholder="Country code (e.g., us, gb)"
                prop:value=move || value.get()
                on:input=move |ev| value.set(event_target_value(&ev))
            />
        }.into_any()
    }
}

/// Agent list page
#[component]
pub fn AgentListPage() -> impl IntoView {
    let navigate = use_navigate();

    // Server resource for listing agents
    let agents_resource = Resource::new(
        || (),
        |_| async move { list_agents().await }
    );

    // Form state - using RwSignal for easier passing to components
    let (show_form, set_show_form) = signal(false);
    let form_name = RwSignal::new(String::new());
    let form_email = RwSignal::new(String::new());
    let form_referral_code = RwSignal::new(String::new());
    let form_country = RwSignal::new(String::new());
    let form_commission = RwSignal::new(3.0f64);
    let (form_error, set_form_error) = signal(Option::<String>::None);
    let (submitting, set_submitting) = signal(false);

    let nav = navigate.clone();
    let on_agent_click = move |id: String| {
        nav(&format!("/agents/{}", id), Default::default());
    };

    // Submit handler
    let submit_form = move || {
        let name = form_name.get();
        let email = form_email.get();
        let country = form_country.get();
        let commission = form_commission.get();
        let referral_code = form_referral_code.get();

        // Validate
        if name.is_empty() {
            set_form_error.set(Some("Name is required".to_string()));
            return;
        }
        if email.is_empty() {
            set_form_error.set(Some("Email is required".to_string()));
            return;
        }
        if country.is_empty() {
            set_form_error.set(Some("Country is required".to_string()));
            return;
        }

        set_form_error.set(None);
        set_submitting.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::spawn_local;

            let form = CreateAgentForm {
                name,
                email,
                country,
                commission_percent: commission,
                referral_code: if referral_code.is_empty() { None } else { Some(referral_code) },
            };

            spawn_local(async move {
                match create_agent(form).await {
                    Ok(_) => {
                        set_show_form.set(false);
                        form_name.set(String::new());
                        form_email.set(String::new());
                        form_referral_code.set(String::new());
                        form_country.set(String::new());
                        form_commission.set(3.0);
                        set_submitting.set(false);
                        agents_resource.refetch();
                    }
                    Err(e) => {
                        set_form_error.set(Some(e.to_string()));
                        set_submitting.set(false);
                    }
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            set_submitting.set(false);
        }
    };

    view! {
        <div>
            <Header title="Agents".to_string() show_search=false />

            <div class="px-4 py-4 space-y-4">
                // Header with Add button
                <div class="flex items-center justify-between">
                    <h2 class="text-lg font-semibold text-slate-900 dark:text-white">"Agent Management"</h2>
                    <button
                        class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition flex items-center gap-2"
                        on:click=move |_| set_show_form.set(true)
                    >
                        <Icon name=IconName::Plus size=16 />
                        "Add Agent"
                    </button>
                </div>

                // Add Agent Form Modal
                {move || {
                    if show_form.get() {
                        let on_submit = submit_form.clone();
                        Some(view! {
                            // Modal overlay - fixed position over everything
                            <div style="position: fixed; inset: 0; z-index: 9999; display: flex; align-items: center; justify-content: center;">
                                // Backdrop - semi-transparent overlay
                                <div
                                    style="position: absolute; inset: 0; background: rgba(0, 0, 0, 0.4); backdrop-filter: blur(1px);"
                                    on:click=move |_| set_show_form.set(false)
                                />

                                // Modal content - positioned above backdrop
                                <div style="position: relative; z-index: 1; background: #1e293b; border-radius: 16px; padding: 20px; width: 100%; max-width: 380px; margin: 16px; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);">
                                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;">
                                        <h3 style="font-size: 18px; font-weight: 600; color: #f1f5f9;">"Add New Agent"</h3>
                                        <button
                                            style="padding: 6px; border-radius: 8px; background: transparent; border: none; cursor: pointer; color: #94a3b8;"
                                            on:click=move |_| set_show_form.set(false)
                                        >
                                            <Icon name=IconName::Close size=18 />
                                        </button>
                                    </div>

                                    // Error message
                                    {move || form_error.get().map(|e| view! {
                                        <div style="margin-bottom: 12px; padding: 10px; border-radius: 8px; background: rgba(239, 68, 68, 0.1); color: #f87171; font-size: 14px;">
                                            {e}
                                        </div>
                                    })}

                                    // Form fields with proper padding
                                    <div style="display: flex; flex-direction: column; gap: 14px;">
                                        // Name field
                                        <div style="padding: 0 2px;">
                                            <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                                                "Name"
                                            </label>
                                            <input
                                                type="text"
                                                style="width: 100%; padding: 10px 12px; border-radius: 8px; border: 1px solid #475569; background: #334155; color: #f1f5f9; font-size: 14px;"
                                                placeholder="Agent name"
                                                prop:value=move || form_name.get()
                                                on:input=move |ev| form_name.set(event_target_value(&ev))
                                            />
                                        </div>

                                        // Email field
                                        <div style="padding: 0 2px;">
                                            <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                                                "Email"
                                            </label>
                                            <input
                                                type="email"
                                                style="width: 100%; padding: 10px 12px; border-radius: 8px; border: 1px solid #475569; background: #334155; color: #f1f5f9; font-size: 14px;"
                                                placeholder="agent@example.com"
                                                prop:value=move || form_email.get()
                                                on:input=move |ev| form_email.set(event_target_value(&ev))
                                            />
                                        </div>

                                        // Referral Code field (NEW)
                                        <div style="padding: 0 2px;">
                                            <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                                                "Referral Code"
                                            </label>
                                            <input
                                                type="text"
                                                style="width: 100%; padding: 10px 12px; border-radius: 8px; border: 1px solid #475569; background: #334155; color: #f1f5f9; font-size: 14px;"
                                                placeholder="Optional referral code"
                                                prop:value=move || form_referral_code.get()
                                                on:input=move |ev| form_referral_code.set(event_target_value(&ev))
                                            />
                                        </div>

                                        // Country dropdown with flags
                                        <div style="padding: 0 2px;">
                                            <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                                                "Country"
                                            </label>
                                            <CountryDropdown value=form_country />
                                        </div>

                                        // Commission slider (0-10%)
                                        <div style="padding: 0 2px;">
                                            <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                                                "Commission %"
                                            </label>
                                            <div style="display: flex; align-items: center; gap: 12px;">
                                                <div style="flex: 1;">
                                                    <CommissionSlider
                                                        value=form_commission
                                                        min=0.0
                                                        max=10.0
                                                    />
                                                </div>
                                                <span style="font-size: 14px; font-weight: 500; color: #f1f5f9; min-width: 45px; text-align: right;">
                                                    {move || format!("{:.1}%", form_commission.get())}
                                                </span>
                                            </div>
                                        </div>
                                    </div>

                                    // Submit buttons
                                    <div style="margin-top: 20px; display: flex; gap: 10px;">
                                        <button
                                            style="flex: 1; padding: 10px 16px; border-radius: 8px; border: 1px solid #475569; background: transparent; color: #94a3b8; font-size: 14px; cursor: pointer;"
                                            on:click=move |_| set_show_form.set(false)
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            style=move || format!(
                                                "flex: 1; padding: 10px 16px; border-radius: 8px; border: none; background: #22d3ee; color: #0f172a; font-size: 14px; font-weight: 500; cursor: pointer; {}",
                                                if submitting.get() { "opacity: 0.5;" } else { "" }
                                            )
                                            disabled=move || submitting.get()
                                            on:click=move |_| on_submit()
                                        >
                                            {move || if submitting.get() { "Creating..." } else { "Create Agent" }}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Agent list content
                <Suspense fallback=move || view! {
                    <div class="bg-white dark:bg-slate-800 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
                        // Progress bar with animated fill - uniform pill shape
                        <div class="relative h-9 bg-slate-200 dark:bg-slate-700" style="border-radius: 18px;">
                            <div class="absolute inset-y-0 left-0 w-1/2 bg-cyan-500 animate-pulse flex items-center justify-center" style="border-radius: 18px;">
                                <span class="text-white text-sm font-medium">"Loading..."</span>
                            </div>
                        </div>
                    </div>
                }>
                    {move || {
                        match agents_resource.get() {
                            None => view! {
                                <div class="text-center py-12 text-slate-500 dark:text-slate-400">
                                    "Loading..."
                                </div>
                            }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                                    <strong>"Error: "</strong>{e.to_string()}
                                </div>
                            }.into_any(),
                            Some(Ok(agents)) => {
                                if agents.is_empty() {
                                    view! {
                                        <div class="text-center py-12">
                                            <div class="text-slate-400 dark:text-slate-500 mb-4">
                                                <Icon name=IconName::Users size=48 class="mx-auto opacity-50".to_string() />
                                            </div>
                                            <p class="text-slate-500 dark:text-slate-400 mb-4">
                                                "No agents yet. Click \"Add Agent\" to create one."
                                            </p>
                                        </div>
                                    }.into_any()
                                } else {
                                    let click_handler = on_agent_click.clone();
                                    view! {
                                        <div class="bg-white dark:bg-slate-800 rounded-2xl overflow-hidden border border-slate-200 dark:border-slate-700">
                                            <div class="p-4 border-b border-slate-200 dark:border-slate-700">
                                                <h3 class="text-base font-semibold text-slate-900 dark:text-white">
                                                    {format!("Agents ({})", agents.len())}
                                                </h3>
                                            </div>
                                            <div class="divide-y divide-slate-200 dark:divide-slate-700">
                                                {agents.into_iter().map(|agent| {
                                                    let id = agent.id.clone();
                                                    let click = click_handler.clone();
                                                    view! {
                                                        <AgentRow
                                                            agent=agent
                                                            on_click=move || click(id.clone())
                                                        />
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

/// Agent row component
#[component]
fn AgentRow(
    agent: AgentEntity,
    on_click: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div
            class="p-4 hover:bg-slate-50 dark:hover:bg-slate-700/50 cursor-pointer transition"
            on:click=move |_| on_click()
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    // Avatar
                    <div class="w-10 h-10 rounded-full bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center">
                        <span class="text-sm font-medium text-primary-600 dark:text-primary-400">
                            {agent.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                        </span>
                    </div>

                    // Info
                    <div>
                        <div class="font-medium text-slate-900 dark:text-white">{agent.name.clone()}</div>
                        <div class="text-sm text-slate-500 dark:text-slate-400">{agent.email.clone()}</div>
                    </div>
                </div>

                // Right side info
                <div class="text-right">
                    <div class="text-sm font-medium text-slate-900 dark:text-white">
                        {format!("{}%", agent.commission_percent)}
                    </div>
                    <div class="text-xs text-slate-500 dark:text-slate-400">{agent.country.clone()}</div>
                </div>
            </div>
        </div>
    }
}

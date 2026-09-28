//! Schema list page for managing content type schemas
//!
//! Displays all available content schemas with options to create, edit, and delete.

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, LoadingOverlay, SpinnerSize};
use crate::api::schema_types::ContentSchema;
use crate::api::schema_client::{get_schemas, delete_schema};
use crate::pages::cms::CmsMenuBar;
use crate::context::use_user_role;

/// Panel styling constants
const PANEL_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 12px;";

/// Schema list page
#[component]
pub fn SchemaListPage() -> impl IntoView {
    let navigate = use_navigate();
    let role_ctx = use_user_role();
    let can_manage_schemas = move || role_ctx.user.get().role.can_publish(); // Publishers and admins can manage schemas

    // Fetch schemas
    let (refresh_trigger, set_refresh_trigger) = signal(0u32);
    let schemas_resource = Resource::new(
        move || refresh_trigger.get(),
        |_| async move {
            get_schemas().await.ok().unwrap_or_default()
        }
    );

    // Delete confirmation state
    let (delete_confirm, set_delete_confirm) = signal(Option::<String>::None);
    let (is_deleting, set_is_deleting) = signal(false);
    let loading_message = RwSignal::new(Option::<String>::None);
    let (error_message, set_error_message) = signal(Option::<String>::None);

    // Navigation handlers - use Callback for cloneable event handlers
    let nav = navigate.clone();
    let on_edit_click = move |id: String| {
        nav(&format!("/schemas/{}", id), Default::default());
    };

    let nav_new = navigate.clone();
    let on_new_click = Callback::new(move |_: web_sys::MouseEvent| {
        nav_new("/schemas/new", Default::default());
    });

    // Delete handler
    let do_delete = move |schema_id: String| {
        set_is_deleting.set(true);
        loading_message.set(Some("Deleting schema...".to_string()));
        set_error_message.set(None);

        spawn_local(async move {
            match delete_schema(schema_id.clone()).await {
                Ok(_) => {
                    set_delete_confirm.set(None);
                    set_refresh_trigger.update(|t| *t += 1);
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Failed to delete schema: {}", e)));
                }
            }
            set_is_deleting.set(false);
            loading_message.set(None);
        });
    };

    view! {
        <div>
            // Full-page loading overlay for delete operations
            <LoadingOverlay
                visible=Signal::derive(move || is_deleting.get())
                message=Signal::derive(move || loading_message.get())
            />

            <Header title="CMS".to_string() show_search=false />
            <CmsMenuBar />

            <div class="px-4 py-4 space-y-4">
                // Action bar
                <div class="flex items-center justify-between">
                    <h2 class="text-lg font-semibold text-slate-900 dark:text-white">
                        "Content Schemas"
                    </h2>
                    {
                        let on_new = on_new_click.clone();
                        move || can_manage_schemas().then(|| {
                            let on_new = on_new.clone();
                            view! {
                                <button
                                    class="flex items-center justify-center gap-2 px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition font-medium"
                                    on:click=move |e| on_new.run(e)
                                >
                                    <Icon name=IconName::Plus size=18 />
                                    "New Schema"
                                </button>
                            }
                        })
                    }
                </div>

                // Error message
                {move || error_message.get().map(|msg| view! {
                    <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                        {msg}
                        <button
                            class="ml-2 underline"
                            on:click=move |_| set_error_message.set(None)
                        >
                            "Dismiss"
                        </button>
                    </div>
                })}

                // Delete confirmation modal
                {move || delete_confirm.get().map(|schema_id| {
                    let schema_id_for_delete = schema_id.clone();
                    view! {
                        <div style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 50;">
                            <div style="background: #1e293b; border-radius: 12px; padding: 24px; max-width: 400px; width: 100%; margin: 16px;">
                                <h3 style="color: #e2e8f0; font-size: 18px; font-weight: 600; margin-bottom: 12px;">
                                    "Delete Schema?"
                                </h3>
                                <p style="color: #94a3b8; font-size: 14px; margin-bottom: 20px;">
                                    "Are you sure you want to delete the schema \"" {schema_id.clone()} "\"? "
                                    "This action cannot be undone and may affect existing content."
                                </p>
                                <div style="display: flex; gap: 12px; justify-content: flex-end;">
                                    <button
                                        style="padding: 8px 16px; background: #334155; color: #e2e8f0; border-radius: 8px; border: none; cursor: pointer;"
                                        on:click=move |_| set_delete_confirm.set(None)
                                        disabled=move || is_deleting.get()
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        style="padding: 8px 16px; background: #ef4444; color: white; border-radius: 8px; border: none; cursor: pointer;"
                                        on:click=move |_| do_delete(schema_id_for_delete.clone())
                                        disabled=move || is_deleting.get()
                                    >
                                        {move || if is_deleting.get() { "Deleting..." } else { "Delete" }}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                })}

                // Schemas list
                <Suspense fallback=move || view! {
                    <div style=format!("{}; padding: 48px; display: flex; align-items: center; justify-content: center;", PANEL_STYLE)>
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                }>
                    {
                        let on_new_click = on_new_click.clone();
                        move || {
                        match schemas_resource.get() {
                            None => view! {
                                <div style="display: flex; align-items: center; justify-content: center; padding: 48px;">
                                    <ProgressSpinner size=SpinnerSize::Default />
                                </div>
                            }.into_any(),
                            Some(schemas) if schemas.is_empty() => {
                                let on_new = on_new_click.clone();
                                view! {
                                    <div style=PANEL_STYLE>
                                        <div style="padding: 48px; text-align: center;">
                                            <div style="opacity: 0.4; margin-bottom: 16px;">
                                                <Icon name=IconName::Document size=48 class="mx-auto".to_string() />
                                            </div>
                                            <p style="color: #64748b; margin-bottom: 16px;">
                                                "No schemas defined yet"
                                            </p>
                                            {
                                                let on_new = on_new.clone();
                                                move || can_manage_schemas().then(|| {
                                                    let on_new = on_new.clone();
                                                    view! {
                                                        <button
                                                            class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition"
                                                            on:click=move |e| on_new.run(e)
                                                        >
                                                            "Create your first schema"
                                                        </button>
                                                    }
                                                })
                                            }
                                        </div>
                                    </div>
                                }.into_any()
                            },
                            Some(schemas) => {
                                view! {
                                    <div style=PANEL_STYLE>
                                        <table style="width: 100%; border-collapse: collapse;">
                                            <thead>
                                                <tr style="border-bottom: 1px solid #1e293b;">
                                                    <th style="padding: 14px 16px; text-align: left; color: #94a3b8; font-weight: 500; font-size: 13px;">"ID"</th>
                                                    <th style="padding: 14px 16px; text-align: left; color: #94a3b8; font-weight: 500; font-size: 13px;">"Name"</th>
                                                    <th style="padding: 14px 16px; text-align: center; color: #94a3b8; font-weight: 500; font-size: 13px;">"Fields"</th>
                                                    <th style="padding: 14px 16px; text-align: center; color: #94a3b8; font-weight: 500; font-size: 13px;">"System"</th>
                                                    <th style="padding: 14px 16px; text-align: right; color: #94a3b8; font-weight: 500; font-size: 13px;">"Actions"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {schemas.into_iter().map(|schema| {
                                                    let schema_id = schema.id.clone();
                                                    let schema_id_for_edit = schema.id.clone();
                                                    let schema_id_for_delete = schema.id.clone();
                                                    let click = on_edit_click.clone();
                                                    let is_system = schema.is_system;
                                                    let can_edit = can_manage_schemas();

                                                    view! {
                                                        <SchemaTableRow
                                                            schema=schema
                                                            on_edit=move || click(schema_id_for_edit.clone())
                                                            on_delete=move || set_delete_confirm.set(Some(schema_id_for_delete.clone()))
                                                            can_edit=can_edit
                                                        />
                                                    }
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    </div>
                                }.into_any()
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

/// Schema table row component
#[component]
fn SchemaTableRow(
    schema: ContentSchema,
    on_edit: impl Fn() + 'static,
    on_delete: impl Fn() + 'static,
    can_edit: bool,
) -> impl IntoView {
    let field_count = schema.fields.len();
    let is_system = schema.is_system;

    view! {
        <tr style="border-bottom: 1px solid #1e293b; transition: background 0.15s;" class="hover:bg-[#1e293b]">
            // ID
            <td style="padding: 14px 16px;">
                <code style="background: #334155; padding: 2px 8px; border-radius: 4px; color: #e2e8f0; font-size: 13px;">
                    {schema.id.clone()}
                </code>
            </td>

            // Name
            <td style="padding: 14px 16px;">
                <div style="font-weight: 500; color: #e2e8f0;">
                    {schema.name.clone()}
                </div>
                {schema.description.as_ref().map(|desc| view! {
                    <div style="font-size: 12px; color: #64748b; margin-top: 2px;">
                        {desc.clone()}
                    </div>
                })}
            </td>

            // Fields count
            <td style="text-align: center; padding: 14px 16px; color: #cbd5e1;">
                {field_count}
            </td>

            // System schema indicator
            <td style="text-align: center; padding: 14px 16px;">
                {if is_system {
                    view! {
                        <span style="background: #334155; color: #94a3b8; padding: 2px 8px; border-radius: 4px; font-size: 12px;">
                            "System"
                        </span>
                    }.into_any()
                } else {
                    view! {
                        <span style="color: #64748b; font-size: 12px;">"-"</span>
                    }.into_any()
                }}
            </td>

            // Actions
            <td style="text-align: right; padding: 14px 16px;">
                <div style="display: flex; gap: 8px; justify-content: flex-end;">
                    <button
                        style="background: transparent; border: none; color: #94a3b8; cursor: pointer; padding: 4px;"
                        on:click=move |_| on_edit()
                        title="Edit"
                    >
                        <Icon name=IconName::Edit size=16 />
                    </button>
                    {if can_edit && !is_system {
                        view! {
                            <button
                                style="background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 4px;"
                                on:click=move |_| on_delete()
                                title="Delete"
                            >
                                <Icon name=IconName::Trash size=16 />
                            </button>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </td>
        </tr>
    }
}

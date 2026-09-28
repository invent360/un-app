//! Schema editor page for creating and editing content type schemas
//!
//! Provides a form interface for defining schema fields and settings.

use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, LoadingOverlay, SpinnerSize};
use crate::api::schema_types::{
    FieldDefinition, FieldType, SchemaSettings, TextFieldConfig,
    SelectOption, MultiSelectFieldConfig, ReferenceFieldConfig, ReferenceListFieldConfig,
};
use crate::api::schema_client::{get_schema, create_schema, update_schema};
use crate::pages::cms::CmsMenuBar;

/// Panel styling constants
const PANEL_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 24px;";
const INPUT_STYLE: &str = "background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px;";
const LABEL_STYLE: &str = "display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;";

/// Schema editor page
#[component]
pub fn SchemaEditorPage() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let is_new = move || {
        params.get_untracked().get("id").map(|id| id == "new").unwrap_or(true)
    };

    let schema_id = move || {
        params.get_untracked().get("id").and_then(|id| {
            if id == "new" { None } else { Some(id.clone()) }
        })
    };

    // State
    let (is_loading, set_is_loading) = signal(false);
    let loading_message = RwSignal::new(Option::<String>::None);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (success_message, set_success_message) = signal(Option::<String>::None);

    // Form state
    let (id, set_id) = signal(String::new());
    let (name, set_name) = signal(String::new());
    let (name_plural, set_name_plural) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let fields = RwSignal::new(Vec::<FieldDefinition>::new());

    // Settings
    let (has_slug, set_has_slug) = signal(true);
    let (title_field, set_title_field) = signal("title".to_string());
    let (translatable, set_translatable) = signal(false);
    let (versioned, set_versioned) = signal(true);
    let (orderable, set_orderable) = signal(false);
    let (schema_version, set_schema_version) = signal(1i32);
    let (is_system, set_is_system) = signal(false);
    let (created_at, set_created_at) = signal(String::new());

    // Track which field is expanded for configuration
    let (expanded_field, set_expanded_field) = signal(Option::<usize>::None);

    // Fetch existing schema if editing
    let schema_resource = Resource::new(
        move || schema_id(),
        |id_opt| async move {
            match id_opt {
                Some(id) => get_schema(id).await.ok(),
                None => None,
            }
        }
    );

    // Populate form when schema loads
    Effect::new(move |_| {
        if let Some(Some(schema)) = schema_resource.get() {
            set_id.set(schema.id.clone());
            set_name.set(schema.name.clone());
            set_name_plural.set(schema.name_plural.clone());
            set_description.set(schema.description.unwrap_or_default());
            fields.set(schema.fields.clone());
            set_has_slug.set(schema.settings.has_slug);
            set_title_field.set(schema.settings.title_field.clone());
            set_translatable.set(schema.settings.translatable);
            set_versioned.set(schema.settings.versioned);
            set_orderable.set(schema.settings.orderable);
            set_schema_version.set(schema.version);
            set_is_system.set(schema.is_system);
            set_created_at.set(schema.created_at.to_rfc3339());
        }
    });

    // Add a new field
    let add_field = move |_: web_sys::MouseEvent| {
        let mut current = fields.get();
        let field_num = current.len() + 1;
        current.push(FieldDefinition {
            key: format!("field_{}", field_num),
            label: format!("Field {}", field_num),
            description: None,
            field_type: FieldType::Text(TextFieldConfig::default()),
            required: false,
            translatable: false,
            show_in_list: false,
            searchable: false,
            default_value: None,
            conditions: None,
        });
        fields.set(current);
    };

    // Remove a field
    let remove_field = move |index: usize| {
        let mut current = fields.get();
        if index < current.len() {
            current.remove(index);
            fields.set(current);
        }
    };

    // Update field key
    let update_field_key = move |index: usize, new_key: String| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.key = new_key;
        }
        fields.set(current);
    };

    // Update field label
    let update_field_label = move |index: usize, new_label: String| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.label = new_label;
        }
        fields.set(current);
    };

    // Update field required
    let update_field_required = move |index: usize, required: bool| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.required = required;
        }
        fields.set(current);
    };

    // Update field type
    let update_field_type = move |index: usize, type_str: String| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.field_type = match type_str.as_str() {
                "text" => FieldType::Text(TextFieldConfig::default()),
                "textarea" => FieldType::Text(TextFieldConfig { multiline: true, ..Default::default() }),
                "richtext" => FieldType::RichText(Default::default()),
                "number" => FieldType::Number(Default::default()),
                "boolean" => FieldType::Boolean(Default::default()),
                "select" => FieldType::Select(Default::default()),
                "multiselect" => FieldType::MultiSelect(Default::default()),
                "date" => FieldType::Date(Default::default()),
                "media" => FieldType::Media(Default::default()),
                "medialist" => FieldType::MediaList(Default::default()),
                "list" => FieldType::List(Default::default()),
                "repeater" => FieldType::Repeater(Default::default()),
                "group" => FieldType::Group(Default::default()),
                "reference" => FieldType::Reference(ReferenceFieldConfig {
                    allowed_types: vec![],
                    display_field: "title".to_string(),
                }),
                "referencelist" => FieldType::ReferenceList(ReferenceListFieldConfig {
                    allowed_types: vec![],
                    display_field: "title".to_string(),
                    min_items: None,
                    max_items: None,
                }),
                _ => FieldType::Text(TextFieldConfig::default()),
            };
        }
        fields.set(current);
    };

    // Update field description
    let update_field_description = move |index: usize, desc: String| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.description = if desc.is_empty() { None } else { Some(desc) };
        }
        fields.set(current);
    };

    // Update field translatable flag
    let update_field_translatable = move |index: usize, trans: bool| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.translatable = trans;
        }
        fields.set(current);
    };

    // Update field show_in_list flag
    let update_field_show_in_list = move |index: usize, show: bool| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.show_in_list = show;
        }
        fields.set(current);
    };

    // Update field searchable flag
    let update_field_searchable = move |index: usize, searchable: bool| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            field.searchable = searchable;
        }
        fields.set(current);
    };

    // Update text field config
    let update_text_config = move |index: usize, min_len: Option<usize>, max_len: Option<usize>, placeholder: Option<String>| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            if let FieldType::Text(ref mut config) = field.field_type {
                config.min_length = min_len;
                config.max_length = max_len;
                config.placeholder = placeholder;
            }
        }
        fields.set(current);
    };

    // Update number field config
    let update_number_config = move |index: usize, min: Option<f64>, max: Option<f64>, step: Option<f64>| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            if let FieldType::Number(ref mut config) = field.field_type {
                config.min = min;
                config.max = max;
                config.step = step;
            }
        }
        fields.set(current);
    };

    // Update date field config
    let update_date_config = move |index: usize, include_time: bool| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            if let FieldType::Date(ref mut config) = field.field_type {
                config.include_time = include_time;
            }
        }
        fields.set(current);
    };

    // Update select options
    let update_select_options = move |index: usize, options_str: String| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            let options: Vec<SelectOption> = options_str
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|line| {
                    let parts: Vec<&str> = line.splitn(2, '|').collect();
                    SelectOption {
                        value: parts.first().unwrap_or(&"").trim().to_string(),
                        label: parts.get(1).unwrap_or(parts.first().unwrap_or(&"")).trim().to_string(),
                        description: None,
                    }
                })
                .collect();

            match &mut field.field_type {
                FieldType::Select(ref mut config) => config.options = options,
                FieldType::MultiSelect(ref mut config) => config.options = options,
                _ => {}
            }
        }
        fields.set(current);
    };

    // Update reference config
    let update_reference_config = move |index: usize, allowed_types: Vec<String>, display_field: String| {
        let mut current = fields.get();
        if let Some(field) = current.get_mut(index) {
            match &mut field.field_type {
                FieldType::Reference(ref mut config) => {
                    config.allowed_types = allowed_types;
                    config.display_field = display_field;
                }
                FieldType::ReferenceList(ref mut config) => {
                    config.allowed_types = allowed_types;
                    config.display_field = display_field;
                }
                _ => {}
            }
        }
        fields.set(current);
    };

    // Save handler
    let nav = navigate.clone();
    let save_action = Action::new(move |_: &()| {
        let is_new_schema = is_new();
        let schema_id_val = id.get();
        let name_val = name.get();
        let name_plural_val = name_plural.get();
        let desc_val = description.get();
        let fields_val = fields.get();
        let version_val = schema_version.get();
        let is_system_val = is_system.get();
        let created_at_val = created_at.get();
        let nav = nav.clone();

        // Check if there's a slug field in the schema
        let has_slug_field = fields_val.iter().any(|f| f.key == "slug");

        let settings = SchemaSettings {
            has_slug: has_slug.get(),
            slug_field: if has_slug.get() && has_slug_field { Some("slug".to_string()) } else { None },
            title_field: title_field.get(),
            preview_fields: vec![],
            orderable: orderable.get(),
            translatable: translatable.get(),
            versioned: versioned.get(),
            singleton: false,
        };

        async move {
            set_is_loading.set(true);
            loading_message.set(Some("Saving schema...".to_string()));
            set_error_message.set(None);
            set_success_message.set(None);

            // Validate
            if schema_id_val.is_empty() {
                set_error_message.set(Some("Schema ID is required".to_string()));
                set_is_loading.set(false);
                loading_message.set(None);
                return;
            }
            if name_val.is_empty() {
                set_error_message.set(Some("Schema name is required".to_string()));
                set_is_loading.set(false);
                loading_message.set(None);
                return;
            }

            // Auto-generate name_plural if empty
            let name_plural_final = if name_plural_val.is_empty() {
                format!("{}s", name_val)
            } else {
                name_plural_val
            };

            let fields_json = serde_json::to_string(&fields_val).unwrap_or_default();
            let settings_json = serde_json::to_string(&settings).unwrap_or_default();

            let result = if is_new_schema {
                create_schema(
                    schema_id_val.clone(),
                    name_val,
                    name_plural_final,
                    if desc_val.is_empty() { None } else { Some(desc_val) },
                    fields_json,
                    settings_json,
                ).await
            } else {
                update_schema(
                    schema_id_val.clone(),
                    name_val,
                    name_plural_final,
                    if desc_val.is_empty() { None } else { Some(desc_val) },
                    fields_json,
                    settings_json,
                    version_val,
                    is_system_val,
                    created_at_val,
                ).await
            };

            set_is_loading.set(false);
            loading_message.set(None);

            match result {
                Ok(_) => {
                    set_success_message.set(Some("Schema saved successfully".to_string()));
                    // Navigate to list after short delay
                    if is_new_schema {
                        nav(&format!("/schemas/{}", schema_id_val), Default::default());
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }
        }
    });

    let nav_back = navigate.clone();
    let on_back = move |_: web_sys::MouseEvent| {
        nav_back("/schemas", Default::default());
    };

    view! {
        <div>
            // Full-page loading overlay for save operations
            <LoadingOverlay
                visible=Signal::derive(move || is_loading.get())
                message=Signal::derive(move || loading_message.get())
            />

            <Header title="CMS".to_string() show_search=false />
            <CmsMenuBar />

            <div class="px-4 py-4 space-y-4">
                // Header
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <button
                            class="p-2 hover:bg-slate-700 rounded-lg transition"
                            on:click=on_back
                        >
                            <Icon name=IconName::ArrowLeft size=20 />
                        </button>
                        <h2 class="text-lg font-semibold text-slate-900 dark:text-white">
                            {move || if is_new() { "New Schema" } else { "Edit Schema" }}
                        </h2>
                    </div>
                    <button
                        class="flex items-center justify-center gap-2 px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition font-medium disabled:opacity-50"
                        on:click=move |_| { let _ = save_action.dispatch(()); }
                        disabled=move || is_loading.get()
                    >
                        {move || if is_loading.get() { "Saving..." } else { "Save Schema" }}
                    </button>
                </div>

                // Messages
                {move || error_message.get().map(|msg| view! {
                    <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                        {msg}
                    </div>
                })}
                {move || success_message.get().map(|msg| view! {
                    <div class="p-4 rounded-lg bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 text-sm">
                        {msg}
                    </div>
                })}

                // Form - Stacked layout
                <div class="space-y-4">
                    // Basic Information
                    <div style=PANEL_STYLE>
                        <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0; margin-bottom: 20px;">
                            "Basic Information"
                        </h3>

                        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px; margin-bottom: 16px;">
                            <div>
                                <label style=LABEL_STYLE>"Schema ID *"</label>
                                <input
                                    type="text"
                                    style=INPUT_STYLE
                                    placeholder="e.g., blog_post"
                                    prop:value=move || id.get()
                                    on:input=move |ev| set_id.set(event_target_value(&ev))
                                    disabled=move || !is_new()
                                />
                                <p style="font-size: 12px; color: #64748b; margin-top: 4px;">
                                    "Unique identifier (lowercase, underscores allowed)"
                                </p>
                            </div>
                            <div>
                                <label style=LABEL_STYLE>"Name *"</label>
                                <input
                                    type="text"
                                    style=INPUT_STYLE
                                    placeholder="e.g., Blog Post"
                                    prop:value=move || name.get()
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                />
                            </div>
                            <div>
                                <label style=LABEL_STYLE>"Name (Plural)"</label>
                                <input
                                    type="text"
                                    style=INPUT_STYLE
                                    placeholder="e.g., Blog Posts"
                                    prop:value=move || name_plural.get()
                                    on:input=move |ev| set_name_plural.set(event_target_value(&ev))
                                />
                                <p style="font-size: 12px; color: #64748b; margin-top: 4px;">
                                    "Auto-generated if empty"
                                </p>
                            </div>
                        </div>

                        <div>
                            <label style=LABEL_STYLE>"Description"</label>
                            <textarea
                                style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px; min-height: 80px; resize: vertical;"
                                placeholder="Describe this content type..."
                                prop:value=move || description.get()
                                on:input=move |ev| set_description.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    // Settings
                    <div style=PANEL_STYLE>
                        <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0; margin-bottom: 20px;">
                            "Settings"
                        </h3>

                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 24px;">
                            <div>
                                <label style=LABEL_STYLE>"Title Field"</label>
                                <input
                                    type="text"
                                    style=INPUT_STYLE
                                    placeholder="title"
                                    prop:value=move || title_field.get()
                                    on:input=move |ev| set_title_field.set(event_target_value(&ev))
                                />
                                <p style="font-size: 12px; color: #64748b; margin-top: 4px;">
                                    "Field key used as the title"
                                </p>
                            </div>

                            <div style="display: flex; flex-wrap: wrap; gap: 24px; align-items: start;">
                                <div>
                                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                        <input
                                            type="checkbox"
                                            style="width: 16px; height: 16px;"
                                            checked=move || has_slug.get()
                                            on:change=move |ev| set_has_slug.set(event_target_checked(&ev))
                                        />
                                        <span style="color: #e2e8f0; font-size: 14px;">"Has Slug"</span>
                                    </label>
                                    <p style="font-size: 12px; color: #64748b; margin-top: 4px; margin-left: 24px;">
                                        "Content has a unique URL slug"
                                    </p>
                                </div>

                                <div>
                                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                        <input
                                            type="checkbox"
                                            style="width: 16px; height: 16px;"
                                            checked=move || versioned.get()
                                            on:change=move |ev| set_versioned.set(event_target_checked(&ev))
                                        />
                                        <span style="color: #e2e8f0; font-size: 14px;">"Versioned"</span>
                                    </label>
                                    <p style="font-size: 12px; color: #64748b; margin-top: 4px; margin-left: 24px;">
                                        "Track version history"
                                    </p>
                                </div>

                                <div>
                                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                        <input
                                            type="checkbox"
                                            style="width: 16px; height: 16px;"
                                            checked=move || translatable.get()
                                            on:change=move |ev| set_translatable.set(event_target_checked(&ev))
                                        />
                                        <span style="color: #e2e8f0; font-size: 14px;">"Translatable"</span>
                                    </label>
                                    <p style="font-size: 12px; color: #64748b; margin-top: 4px; margin-left: 24px;">
                                        "Enable multi-language support"
                                    </p>
                                </div>

                                <div>
                                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                        <input
                                            type="checkbox"
                                            style="width: 16px; height: 16px;"
                                            checked=move || orderable.get()
                                            on:change=move |ev| set_orderable.set(event_target_checked(&ev))
                                        />
                                        <span style="color: #e2e8f0; font-size: 14px;">"Orderable"</span>
                                    </label>
                                    <p style="font-size: 12px; color: #64748b; margin-top: 4px; margin-left: 24px;">
                                        "Allow manual ordering of items"
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Fields
                    <div style=PANEL_STYLE>
                            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px;">
                                <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0;">
                                    "Fields"
                                </h3>
                                <button
                                    style="padding: 8px 16px; background: #334155; color: #e2e8f0; border-radius: 8px; border: none; cursor: pointer; font-size: 14px;"
                                    on:click=add_field
                                >
                                    "+ Add Field"
                                </button>
                            </div>

                            <div style="display: flex; flex-direction: column; gap: 12px;">
                                {move || {
                                    let field_list = fields.get();
                                    if field_list.is_empty() {
                                        view! {
                                            <div style="text-align: center; padding: 32px; color: #64748b;">
                                                "No fields defined. Click \"Add Field\" to start."
                                            </div>
                                        }.into_any()
                                    } else {
                                        field_list.iter().enumerate().map(|(index, field)| {
                                            let field_key = field.key.clone();
                                            let field_label = field.label.clone();
                                            let field_required = field.required;
                                            let field_translatable = field.translatable;
                                            let field_show_in_list = field.show_in_list;
                                            let field_searchable = field.searchable;
                                            let field_description = field.description.clone().unwrap_or_default();
                                            let field_type = field.field_type.clone();
                                            let field_type_str = get_field_type_str(&field.field_type);
                                            let is_expanded = move || expanded_field.get() == Some(index);

                                            view! {
                                                <div style="background: #1e293b; border-radius: 8px; padding: 16px;">
                                                    // Main row
                                                    <div style="display: grid; grid-template-columns: 1fr 1fr 180px auto auto auto; gap: 12px; align-items: end;">
                                                        <div>
                                                            <label style="font-size: 12px; color: #64748b; margin-bottom: 4px; display: block;">"Key"</label>
                                                            <input
                                                                type="text"
                                                                style="background: #0f172a; border: 1px solid #334155; border-radius: 6px; padding: 8px 12px; width: 100%; color: #e2e8f0; font-size: 13px; height: 38px; box-sizing: border-box;"
                                                                prop:value=field_key.clone()
                                                                on:input=move |ev| update_field_key(index, event_target_value(&ev))
                                                            />
                                                        </div>
                                                        <div>
                                                            <label style="font-size: 12px; color: #64748b; margin-bottom: 4px; display: block;">"Label"</label>
                                                            <input
                                                                type="text"
                                                                style="background: #0f172a; border: 1px solid #334155; border-radius: 6px; padding: 8px 12px; width: 100%; color: #e2e8f0; font-size: 13px; height: 38px; box-sizing: border-box;"
                                                                prop:value=field_label.clone()
                                                                on:input=move |ev| update_field_label(index, event_target_value(&ev))
                                                            />
                                                        </div>
                                                        <div>
                                                            <label style="font-size: 12px; color: #64748b; margin-bottom: 4px; display: block;">"Type"</label>
                                                            <select
                                                                style="background: #0f172a; border: 1px solid #334155; border-radius: 6px; padding: 8px 32px 8px 12px; width: 100%; color: #e2e8f0; font-size: 13px; height: 38px; box-sizing: border-box; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 24 24%22 fill=%22none%22 stroke=%22%2394a3b8%22 stroke-width=%222%22%3E%3Cpath d=%22M6 9l6 6 6-6%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 12px center;"
                                                                on:change=move |ev| update_field_type(index, event_target_value(&ev))
                                                            >
                                                                <optgroup label="Basic">
                                                                    <option value="text" selected=move || field_type_str == "text">"Text"</option>
                                                                    <option value="textarea" selected=move || field_type_str == "textarea">"Textarea"</option>
                                                                    <option value="richtext" selected=move || field_type_str == "richtext">"Rich Text"</option>
                                                                    <option value="number" selected=move || field_type_str == "number">"Number"</option>
                                                                    <option value="boolean" selected=move || field_type_str == "boolean">"Boolean"</option>
                                                                    <option value="date" selected=move || field_type_str == "date">"Date"</option>
                                                                </optgroup>
                                                                <optgroup label="Selection">
                                                                    <option value="select" selected=move || field_type_str == "select">"Select"</option>
                                                                    <option value="multiselect" selected=move || field_type_str == "multiselect">"Multi-Select"</option>
                                                                </optgroup>
                                                                <optgroup label="Media">
                                                                    <option value="media" selected=move || field_type_str == "media">"Media"</option>
                                                                    <option value="medialist" selected=move || field_type_str == "medialist">"Media List"</option>
                                                                </optgroup>
                                                                <optgroup label="Complex">
                                                                    <option value="list" selected=move || field_type_str == "list">"List (Strings)"</option>
                                                                    <option value="repeater" selected=move || field_type_str == "repeater">"Repeater"</option>
                                                                    <option value="group" selected=move || field_type_str == "group">"Group"</option>
                                                                </optgroup>
                                                                <optgroup label="References">
                                                                    <option value="reference" selected=move || field_type_str == "reference">"Reference"</option>
                                                                    <option value="referencelist" selected=move || field_type_str == "referencelist">"Reference List"</option>
                                                                </optgroup>
                                                            </select>
                                                        </div>
                                                        <div style="display: flex; align-items: center; gap: 6px; height: 38px;">
                                                            <input
                                                                type="checkbox"
                                                                style="width: 16px; height: 16px;"
                                                                checked=field_required
                                                                on:change=move |ev| update_field_required(index, event_target_checked(&ev))
                                                            />
                                                            <span style="font-size: 12px; color: #94a3b8;">"Required"</span>
                                                        </div>
                                                        <button
                                                            style="display: flex; align-items: center; justify-content: center; background: transparent; border: none; color: #94a3b8; cursor: pointer; width: 38px; height: 38px;"
                                                            on:click=move |_| {
                                                                if is_expanded() {
                                                                    set_expanded_field.set(None);
                                                                } else {
                                                                    set_expanded_field.set(Some(index));
                                                                }
                                                            }
                                                            title="Configure field"
                                                        >
                                                            <Icon name=IconName::Settings size=18 />
                                                        </button>
                                                        <button
                                                            style="display: flex; align-items: center; justify-content: center; background: transparent; border: none; color: #ef4444; cursor: pointer; width: 38px; height: 38px;"
                                                            on:click=move |_| remove_field(index)
                                                            title="Remove field"
                                                        >
                                                            <Icon name=IconName::Trash size=18 />
                                                        </button>
                                                    </div>

                                                    // Configuration panel (expandable)
                                                    {move || {
                                                        let field_type = field_type.clone();
                                                        let field_description = field_description.clone();
                                                        is_expanded().then(|| view! {
                                                            <div style="margin-top: 16px; padding-top: 16px; border-top: 1px solid #334155;">
                                                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 16px;">
                                                                    // Description
                                                                    <div style="grid-column: span 2;">
                                                                        <label style="font-size: 12px; color: #64748b; margin-bottom: 4px; display: block;">"Description"</label>
                                                                        <input
                                                                            type="text"
                                                                            style="background: #0f172a; border: 1px solid #334155; border-radius: 6px; padding: 8px 12px; width: 100%; color: #e2e8f0; font-size: 13px;"
                                                                            placeholder="Help text for this field..."
                                                                            prop:value=field_description.clone()
                                                                            on:input=move |ev| update_field_description(index, event_target_value(&ev))
                                                                        />
                                                                    </div>

                                                                    // Common flags
                                                                    <div style="display: flex; gap: 16px; grid-column: span 2;">
                                                                        <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
                                                                            <input
                                                                                type="checkbox"
                                                                                style="width: 14px; height: 14px;"
                                                                                checked=field_translatable
                                                                                on:change=move |ev| update_field_translatable(index, event_target_checked(&ev))
                                                                            />
                                                                            <span style="font-size: 12px; color: #94a3b8;">"Translatable"</span>
                                                                        </label>
                                                                        <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
                                                                            <input
                                                                                type="checkbox"
                                                                                style="width: 14px; height: 14px;"
                                                                                checked=field_show_in_list
                                                                                on:change=move |ev| update_field_show_in_list(index, event_target_checked(&ev))
                                                                            />
                                                                            <span style="font-size: 12px; color: #94a3b8;">"Show in List"</span>
                                                                        </label>
                                                                        <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
                                                                            <input
                                                                                type="checkbox"
                                                                                style="width: 14px; height: 14px;"
                                                                                checked=field_searchable
                                                                                on:change=move |ev| update_field_searchable(index, event_target_checked(&ev))
                                                                            />
                                                                            <span style="font-size: 12px; color: #94a3b8;">"Searchable"</span>
                                                                        </label>
                                                                    </div>
                                                                </div>

                                                                // Type-specific configuration
                                                                {render_field_config(index, &field_type, update_text_config, update_number_config, update_date_config, update_select_options, update_reference_config)}
                                                            </div>
                                                        })
                                                    }}
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
    }
}

/// Get field type string from FieldType enum
fn get_field_type_str(field_type: &FieldType) -> &'static str {
    match field_type {
        FieldType::Text(config) if config.multiline => "textarea",
        FieldType::Text(_) => "text",
        FieldType::RichText(_) => "richtext",
        FieldType::Number(_) => "number",
        FieldType::Boolean(_) => "boolean",
        FieldType::Select(_) => "select",
        FieldType::MultiSelect(_) => "multiselect",
        FieldType::Date(_) => "date",
        FieldType::Media(_) => "media",
        FieldType::MediaList(_) => "medialist",
        FieldType::List(_) => "list",
        FieldType::Repeater(_) => "repeater",
        FieldType::Group(_) => "group",
        FieldType::Json(_) => "json",
        FieldType::Reference(_) => "reference",
        FieldType::ReferenceList(_) => "referencelist",
    }
}

/// Render type-specific field configuration
fn render_field_config(
    index: usize,
    field_type: &FieldType,
    update_text_config: impl Fn(usize, Option<usize>, Option<usize>, Option<String>) + Copy + 'static,
    update_number_config: impl Fn(usize, Option<f64>, Option<f64>, Option<f64>) + Copy + 'static,
    update_date_config: impl Fn(usize, bool) + Copy + 'static,
    update_select_options: impl Fn(usize, String) + Copy + 'static,
    update_reference_config: impl Fn(usize, Vec<String>, String) + Copy + 'static,
) -> impl IntoView {
    let config_label_style = "font-size: 12px; color: #64748b; margin-bottom: 4px; display: block;";
    let config_input_style = "background: #0f172a; border: 1px solid #334155; border-radius: 6px; padding: 8px 12px; width: 100%; color: #e2e8f0; font-size: 13px;";

    match field_type {
        FieldType::Text(config) => {
            let min_len_val = config.min_length;
            let max_len_val = config.max_length;
            let placeholder_val = config.placeholder.clone();
            let placeholder_val2 = placeholder_val.clone();
            let placeholder_val3 = placeholder_val.clone();
            let min_len = min_len_val.map(|v| v.to_string()).unwrap_or_default();
            let max_len = max_len_val.map(|v| v.to_string()).unwrap_or_default();
            let placeholder = placeholder_val.clone().unwrap_or_default();

            view! {
                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;">
                    <div>
                        <label style=config_label_style>"Min Length"</label>
                        <input
                            type="number"
                            style=config_input_style
                            placeholder="No minimum"
                            prop:value=min_len
                            on:change=move |ev| {
                                let val = event_target_value(&ev).parse().ok();
                                update_text_config(index, val, max_len_val, placeholder_val.clone());
                            }
                        />
                    </div>
                    <div>
                        <label style=config_label_style>"Max Length"</label>
                        <input
                            type="number"
                            style=config_input_style
                            placeholder="No maximum"
                            prop:value=max_len
                            on:change=move |ev| {
                                let val = event_target_value(&ev).parse().ok();
                                update_text_config(index, min_len_val, val, placeholder_val2.clone());
                            }
                        />
                    </div>
                    <div>
                        <label style=config_label_style>"Placeholder"</label>
                        <input
                            type="text"
                            style=config_input_style
                            placeholder="Placeholder text..."
                            prop:value=placeholder
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                update_text_config(index, min_len_val, max_len_val, if val.is_empty() { None } else { Some(val) });
                            }
                        />
                    </div>
                </div>
            }.into_any()
        }

        FieldType::Number(config) => {
            let min_num = config.min;
            let max_num = config.max;
            let step_num = config.step;
            let min_val = min_num.map(|v| v.to_string()).unwrap_or_default();
            let max_val = max_num.map(|v| v.to_string()).unwrap_or_default();
            let step_val = step_num.map(|v| v.to_string()).unwrap_or_default();

            view! {
                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;">
                    <div>
                        <label style=config_label_style>"Minimum"</label>
                        <input
                            type="number"
                            style=config_input_style
                            placeholder="No minimum"
                            prop:value=min_val
                            on:change=move |ev| {
                                let val = event_target_value(&ev).parse().ok();
                                update_number_config(index, val, max_num, step_num);
                            }
                        />
                    </div>
                    <div>
                        <label style=config_label_style>"Maximum"</label>
                        <input
                            type="number"
                            style=config_input_style
                            placeholder="No maximum"
                            prop:value=max_val
                            on:change=move |ev| {
                                let val = event_target_value(&ev).parse().ok();
                                update_number_config(index, min_num, val, step_num);
                            }
                        />
                    </div>
                    <div>
                        <label style=config_label_style>"Step"</label>
                        <input
                            type="number"
                            style=config_input_style
                            placeholder="1"
                            prop:value=step_val
                            on:change=move |ev| {
                                let val = event_target_value(&ev).parse().ok();
                                update_number_config(index, min_num, max_num, val);
                            }
                        />
                    </div>
                </div>
            }.into_any()
        }

        FieldType::Date(config) => {
            let include_time = config.include_time;

            view! {
                <div>
                    <label style="display: flex; align-items: center; gap: 6px; cursor: pointer;">
                        <input
                            type="checkbox"
                            style="width: 14px; height: 14px;"
                            checked=include_time
                            on:change=move |ev| update_date_config(index, event_target_checked(&ev))
                        />
                        <span style="font-size: 12px; color: #94a3b8;">"Include Time"</span>
                    </label>
                    <p style="font-size: 11px; color: #64748b; margin-top: 4px; margin-left: 20px;">
                        "Allow selecting both date and time"
                    </p>
                </div>
            }.into_any()
        }

        FieldType::Select(config) => {
            let options_str = config.options.iter()
                .map(|o| if o.value == o.label { o.value.clone() } else { format!("{}|{}", o.value, o.label) })
                .collect::<Vec<_>>()
                .join("\n");

            view! {
                <div>
                    <label style=config_label_style>"Options (one per line, format: value|label)"</label>
                    <textarea
                        style="background: #0f172a; border: 1px solid #334155; border-radius: 6px; padding: 8px 12px; width: 100%; color: #e2e8f0; font-size: 13px; min-height: 100px; resize: vertical;"
                        placeholder="option1|Option 1\noption2|Option 2"
                        prop:value=options_str
                        on:change=move |ev| update_select_options(index, event_target_value(&ev))
                    />
                    <p style="font-size: 11px; color: #64748b; margin-top: 4px;">
                        "Enter each option on a new line. Use 'value|label' format or just 'value' if both are the same."
                    </p>
                </div>
            }.into_any()
        }

        FieldType::MultiSelect(config) => {
            let options_str = config.options.iter()
                .map(|o| if o.value == o.label { o.value.clone() } else { format!("{}|{}", o.value, o.label) })
                .collect::<Vec<_>>()
                .join("\n");

            view! {
                <div>
                    <label style=config_label_style>"Options (one per line, format: value|label)"</label>
                    <textarea
                        style="background: #0f172a; border: 1px solid #334155; border-radius: 6px; padding: 8px 12px; width: 100%; color: #e2e8f0; font-size: 13px; min-height: 100px; resize: vertical;"
                        placeholder="option1|Option 1\noption2|Option 2"
                        prop:value=options_str
                        on:change=move |ev| update_select_options(index, event_target_value(&ev))
                    />
                    <p style="font-size: 11px; color: #64748b; margin-top: 4px;">
                        "Enter each option on a new line. Users can select multiple values."
                    </p>
                </div>
            }.into_any()
        }

        FieldType::Reference(config) => {
            let allowed_types_vec = config.allowed_types.clone();
            let display_field_str = config.display_field.clone();
            let allowed_types = allowed_types_vec.join(", ");
            let display_field = display_field_str.clone();

            view! {
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                    <div>
                        <label style=config_label_style>"Allowed Content Types (comma-separated)"</label>
                        <input
                            type="text"
                            style=config_input_style
                            placeholder="blog_post, page, article"
                            prop:value=allowed_types
                            on:change=move |ev| {
                                let types: Vec<String> = event_target_value(&ev)
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                                update_reference_config(index, types, display_field_str.clone());
                            }
                        />
                    </div>
                    <div>
                        <label style=config_label_style>"Display Field"</label>
                        <input
                            type="text"
                            style=config_input_style
                            placeholder="title"
                            prop:value=display_field
                            on:change=move |ev| {
                                update_reference_config(index, allowed_types_vec.clone(), event_target_value(&ev));
                            }
                        />
                        <p style="font-size: 11px; color: #64748b; margin-top: 4px;">
                            "Field to show when displaying referenced content"
                        </p>
                    </div>
                </div>
            }.into_any()
        }

        FieldType::ReferenceList(config) => {
            let allowed_types_vec = config.allowed_types.clone();
            let display_field_str = config.display_field.clone();
            let allowed_types = allowed_types_vec.join(", ");
            let display_field = display_field_str.clone();

            view! {
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
                    <div>
                        <label style=config_label_style>"Allowed Content Types (comma-separated)"</label>
                        <input
                            type="text"
                            style=config_input_style
                            placeholder="blog_post, page, article"
                            prop:value=allowed_types
                            on:change=move |ev| {
                                let types: Vec<String> = event_target_value(&ev)
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                                update_reference_config(index, types, display_field_str.clone());
                            }
                        />
                    </div>
                    <div>
                        <label style=config_label_style>"Display Field"</label>
                        <input
                            type="text"
                            style=config_input_style
                            placeholder="title"
                            prop:value=display_field
                            on:change=move |ev| {
                                update_reference_config(index, allowed_types_vec.clone(), event_target_value(&ev));
                            }
                        />
                        <p style="font-size: 11px; color: #64748b; margin-top: 4px;">
                            "Field to show when displaying referenced content"
                        </p>
                    </div>
                </div>
            }.into_any()
        }

        // For other types, show a generic message
        _ => {
            view! {
                <p style="font-size: 12px; color: #64748b; font-style: italic;">
                    "No additional configuration available for this field type."
                </p>
            }.into_any()
        }
    }
}

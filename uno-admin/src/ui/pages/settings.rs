use leptos::prelude::*;
use crate::components::layout::Header;
use crate::components::common::NavPositionToggle;
use crate::context::{use_theme, Theme, use_user_role, UserRole};
use crate::storage::{load_license_configs, save_license_configs};

#[cfg(target_arch = "wasm32")]
use crate::logic::parse_license_csv;

/// Settings page with theme toggle and other options
#[component]
pub fn SettingsPage() -> impl IntoView {
    let theme_ctx = use_theme();
    let role_ctx = use_user_role();
    let (csv_status, set_csv_status) = signal(Option::<String>::None);
    let (config_count, set_config_count) = signal(load_license_configs().len());

    view! {
        <div>
            <Header title="Settings".to_string() show_search=false />

            <div class="px-4 py-4 space-y-4">
                // Role Selector Section (Development/Testing)
                <div class="bg-white dark:bg-slate-800 rounded-2xl p-4">
                    <h3 class="text-base font-semibold text-slate-900 dark:text-white mb-4">
                        "User Role (Development)"
                    </h3>
                    <div class="text-sm text-slate-500 dark:text-slate-400 mb-4">
                        "Switch between roles to test permission-based UI visibility"
                    </div>

                    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                        {[
                            (UserRole::Author, "Author", "Can create and edit content"),
                            (UserRole::Reviewer, "Reviewer", "Can review and approve content"),
                            (UserRole::Publisher, "Publisher", "Can publish and revert content"),
                            (UserRole::Admin, "Admin", "Full access to all features"),
                        ].into_iter().map(|(role, name, desc)| {
                            view! {
                                <button
                                    class=move || format!(
                                        "p-3 rounded-xl border-2 text-left transition {}",
                                        if role_ctx.user.get().role == role {
                                            "border-primary-500 bg-primary-50 dark:bg-primary-900/20"
                                        } else {
                                            "border-slate-200 dark:border-slate-700 hover:border-primary-300"
                                        }
                                    )
                                    on:click=move |_| {
                                        role_ctx.set_role(role);
                                        #[cfg(feature = "hydrate")]
                                        {
                                            use gloo_storage::{LocalStorage, Storage};
                                            let _ = LocalStorage::set("cms_user_role", role.as_str());
                                        }
                                    }
                                >
                                    <div class="font-medium text-slate-900 dark:text-white">{name}</div>
                                    <div class="text-xs text-slate-500 dark:text-slate-400 mt-1">{desc}</div>
                                </button>
                            }
                        }).collect_view()}
                    </div>

                    <div class="mt-4 p-3 rounded-lg bg-slate-100 dark:bg-slate-700/50">
                        <div class="text-sm">
                            <span class="text-slate-500 dark:text-slate-400">"Current role: "</span>
                            <span class="font-medium text-slate-900 dark:text-white">
                                {move || role_ctx.user.get().role.display_name()}
                            </span>
                        </div>
                    </div>
                </div>

                // License Configuration Section
                <div class="bg-white dark:bg-slate-800 rounded-2xl p-4">
                    <h3 class="text-base font-semibold text-slate-900 dark:text-white mb-4">
                        "License Configuration"
                    </h3>

                    <div class="py-3 border-b border-slate-100 dark:border-slate-700">
                        <div class="mb-3">
                            <div class="font-medium text-slate-900 dark:text-white">
                                "Import License Splits CSV"
                            </div>
                            <div class="text-sm text-slate-500 dark:text-slate-400">
                                "Upload a CSV file with license_id, lease_code, uno_share, agent_share, ulo_share, ulo, agent columns"
                            </div>
                        </div>

                        // CSV Upload
                        <CsvUploader
                            set_config_count=set_config_count
                            set_csv_status=set_csv_status
                        />

                        // Status message
                        {move || csv_status.get().map(|msg| {
                            let is_error = msg.starts_with("Error");
                            view! {
                                <div class={format!(
                                    "mt-3 p-3 rounded-lg text-sm {}",
                                    if is_error {
                                        "bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400"
                                    } else {
                                        "bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400"
                                    }
                                )}>
                                    {msg}
                                </div>
                            }
                        })}

                        // Current configs count
                        <div class="mt-3 flex items-center justify-between p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                            <span class="text-sm text-slate-600 dark:text-slate-300">
                                "Stored configurations"
                            </span>
                            <span class="font-medium text-slate-900 dark:text-white">
                                {move || config_count.get().to_string()}
                            </span>
                        </div>

                        // Clear configs button
                        {move || {
                            if config_count.get() > 0 {
                                Some(view! {
                                    <button
                                        class="mt-3 w-full px-4 py-2 text-sm font-medium text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/30 transition"
                                        on:click=move |_| {
                                            save_license_configs(&vec![]);
                                            set_config_count.set(0);
                                            set_csv_status.set(Some("All configurations cleared".to_string()));
                                        }
                                    >
                                        "Clear All Configurations"
                                    </button>
                                })
                            } else {
                                None
                            }
                        }}
                    </div>

                    // CSV Format Help
                    <div class="py-3">
                        <div class="text-sm text-slate-500 dark:text-slate-400">
                            <div class="font-medium text-slate-700 dark:text-slate-300 mb-2">"CSV Format:"</div>
                            <code class="block p-2 bg-slate-100 dark:bg-slate-700 rounded text-xs font-mono overflow-x-auto">
                                "license_id,lease_code,uno_share,agent_share,ulo_share,ulo,agent\n0x015311...,c9668f39-...,47%,3%,50%,NGA Lagos1,Staffman"
                            </code>
                        </div>
                    </div>
                </div>

                // Theme setting
                <div class="settings-card bg-white dark:bg-slate-800 rounded-2xl p-4">
                    <h3 class="text-base font-semibold text-slate-900 dark:text-white mb-4">
                        "Appearance"
                    </h3>

                    // Theme selector
                    <div class="py-3 border-b border-slate-100 dark:border-slate-700">
                        <div class="mb-3">
                            <div class="font-medium text-slate-900 dark:text-white">
                                "Theme"
                            </div>
                            <div class="text-sm text-slate-500 dark:text-slate-400">
                                "Choose your preferred color theme"
                            </div>
                        </div>

                        <div class="theme-selector flex gap-2">
                            {Theme::all().into_iter().map(|t| {
                                let is_active = move || theme_ctx.current() == t;
                                view! {
                                    <button
                                        type="button"
                                        class=move || format!(
                                            "theme-option flex-1 py-2 px-3 rounded-lg text-sm font-medium transition-all {}",
                                            if is_active() {
                                                "theme-option-active"
                                            } else {
                                                "theme-option-inactive"
                                            }
                                        )
                                        on:click=move |_| theme_ctx.set_theme(t)
                                    >
                                        <ThemeIcon theme=t />
                                        <span class="mt-1">{t.label()}</span>
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Navigation position setting
                    <div class="flex items-center justify-between py-3">
                        <div>
                            <div class="font-medium text-slate-900 dark:text-white">
                                "Navigation Position"
                            </div>
                            <div class="text-sm text-slate-500 dark:text-slate-400">
                                "Choose where the navigation appears"
                            </div>
                        </div>

                        <NavPositionToggle />
                    </div>
                </div>

                // Placeholder sections
                <div class="bg-white dark:bg-slate-800 rounded-2xl p-4">
                    <h3 class="text-base font-semibold text-slate-900 dark:text-white mb-4">
                        "Account"
                    </h3>
                    <SettingsItem label="Profile" description="Manage your account details" />
                    <SettingsItem label="Notifications" description="Configure notification preferences" />
                    <SettingsItem label="Security" description="Password and authentication" />
                </div>

                <div class="bg-white dark:bg-slate-800 rounded-2xl p-4">
                    <h3 class="text-base font-semibold text-slate-900 dark:text-white mb-4">
                        "About"
                    </h3>
                    <SettingsItem label="Version" description="1.0.0" />
                    <SettingsItem label="Help & Support" description="Get help with the app" />
                </div>
            </div>
        </div>
    }
}

/// CSV file uploader component
#[component]
fn CsvUploader(
    set_config_count: WriteSignal<usize>,
    set_csv_status: WriteSignal<Option<String>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use web_sys::{Event, HtmlInputElement, FileReader};
        use wasm_bindgen::closure::Closure;

        view! {
            <div class="relative">
                <input
                    type="file"
                    accept=".csv"
                    class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
                    on:change=move |ev: Event| {
                        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
                        if let Some(files) = input.files() {
                            if let Some(file) = files.get(0) {
                                let reader = FileReader::new().unwrap();
                                let reader_clone = reader.clone();

                                let onload = Closure::wrap(Box::new(move |_: Event| {
                                    if let Ok(result) = reader_clone.result() {
                                        if let Some(text) = result.as_string() {
                                            match parse_license_csv(&text) {
                                                Ok(configs) => {
                                                    let count = configs.len();
                                                    save_license_configs(&configs);
                                                    set_config_count.set(count);
                                                    set_csv_status.set(Some(format!("Imported {} license configs", count)));
                                                }
                                                Err(e) => {
                                                    set_csv_status.set(Some(format!("Error: {}", e)));
                                                }
                                            }
                                        } else {
                                            set_csv_status.set(Some("Failed to read file".to_string()));
                                        }
                                    }
                                }) as Box<dyn FnMut(_)>);

                                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                onload.forget();

                                let _ = reader.read_as_text(&file);
                            }
                        }
                    }
                />
                <div class="flex items-center justify-center gap-3 p-4 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-xl hover:border-primary-500 dark:hover:border-primary-500 transition">
                    <svg class="w-6 h-6 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                    </svg>
                    <span class="text-sm text-slate-600 dark:text-slate-300">
                        "Click to upload CSV file"
                    </span>
                </div>
            </div>
        }.into_any()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (set_config_count, set_csv_status);
        view! {
            <div class="p-4 text-center text-slate-500">
                "CSV upload only available in browser"
            </div>
        }.into_any()
    }
}

/// Theme icon for selector
#[component]
fn ThemeIcon(theme: Theme) -> impl IntoView {
    match theme {
        Theme::Light => view! {
            <svg class="theme-preview-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="5"/>
                <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
            </svg>
        }.into_any(),
        Theme::Dark => view! {
            <svg class="theme-preview-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
        }.into_any(),
        Theme::DarkBlue => view! {
            <svg class="theme-preview-icon unity-blue-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#00d4ff" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <path d="M12 6v6l4 2"/>
            </svg>
        }.into_any(),
    }
}

/// Single settings item row
#[component]
fn SettingsItem(
    #[prop(into)] label: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between py-3 border-b border-slate-100 dark:border-slate-700 last:border-b-0">
            <div>
                <div class="font-medium text-slate-900 dark:text-white">{label}</div>
                <div class="text-sm text-slate-500 dark:text-slate-400">{description}</div>
            </div>
            <svg class="w-5 h-5 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
            </svg>
        </div>
    }
}

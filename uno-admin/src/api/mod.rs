pub mod config;
pub mod content_client;
pub mod faq_client;
pub mod faq_types;
pub mod handlers;
pub mod home_client;
pub mod schema_client;
pub mod schema_types;
pub mod section_types;
pub mod types;
pub mod uno_client;

#[cfg(feature = "ssr")]
pub mod http_client;

pub use config::UnityApiConfig;
pub use handlers::*;
pub use types::*;

#[cfg(feature = "ssr")]
pub use uno_client::UnoLicenseClient;

#[cfg(feature = "ssr")]
pub use content_client::{
    AuditLogEntry, AuditLogFilter, AuditLogResponse, ContentClient, ContentClientError,
    ContentDetailResponse, ContentListParams, ContentListResponse, ContentResponse, ContentSummary,
    PublishContentRequest, RevertContentRequest, UpsertContentRequest,
};

// Schema-driven CMS types and client
pub use schema_client::{
    archive_content_item,
    // Also export schema CRUD functions
    create_schema,
    delete_content_item,
    delete_schema,
    get_content_item,
    get_content_item_versions,
    get_schema,
    // Server functions
    get_schemas,
    list_content_items,
    publish_content_item,
    revert_content_item,
    save_content_item,
    update_schema,
    ArchiveContentItem,
    CreateSchema,
    DeleteContentItem,
    DeleteSchema,
    GetContentItem,
    GetContentItemVersions,
    GetSchema,
    // Server function struct types for explicit registration
    GetSchemas,
    ListContentItems,
    PublishContentItem,
    RevertContentItem,
    SaveContentItem,
    UpdateSchema,
};
pub use schema_types::*;

#[cfg(feature = "ssr")]
pub use schema_client::SchemaClient;

#[cfg(feature = "ssr")]
pub use schema_client::register_schema_server_fns;

// Section types (generic) and home page client
pub use section_types::*;

// FAQ types
pub use faq_types::*;

// FAQ page client
pub use faq_client::{
    create_faq_preview, get_faq_page, get_faq_page_versions, publish_faq_page, revert_faq_page,
    save_faq_page, CreateFaqPreview, FaqPageResponse, GetFaqPage, GetFaqPageVersions,
    PublishFaqPage, RevertFaqPage, SaveFaqPage,
};

#[cfg(feature = "ssr")]
pub use faq_client::register_faq_server_fns;

pub use home_client::{
    get_home_page, get_home_page_versions, publish_home_page, revert_home_page, save_home_page,
    GetHomePage, GetHomePageVersions, HomePageResponse, PublishHomePage, RevertHomePage,
    SaveHomePage,
};

#[cfg(feature = "ssr")]
pub use home_client::register_home_server_fns;

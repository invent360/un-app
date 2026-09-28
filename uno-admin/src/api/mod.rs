pub mod config;
pub mod types;
pub mod handlers;
pub mod uno_client;
pub mod content_client;
pub mod wallet_auth;
pub mod schema_types;
pub mod schema_client;
pub mod section_types;
pub mod faq_types;
pub mod faq_client;
pub mod home_client;

#[cfg(feature = "ssr")]
pub mod http_client;

pub use config::UnityApiConfig;
pub use types::*;
pub use handlers::*;

#[cfg(feature = "ssr")]
pub use uno_client::UnoLicenseClient;

#[cfg(feature = "ssr")]
pub use content_client::{
    ContentClient, ContentClientError,
    UpsertContentRequest, ContentListParams, PublishContentRequest, RevertContentRequest,
    ContentSummary, ContentListResponse, ContentDetailResponse, ContentResponse,
    AuditLogFilter, AuditLogEntry, AuditLogResponse,
};

// Wallet authentication (SIWE with ember-multichain)
pub use wallet_auth::{
    Web3AuthRequest, Web3AuthResponse, UnetworkUser, WalletSettings, WalletAuthError,
};

#[cfg(feature = "ssr")]
pub use wallet_auth::WalletAuthService;

// Schema-driven CMS types and client
pub use schema_types::*;
pub use schema_client::{
    // Server functions
    get_schemas, get_schema, list_content_items, get_content_item,
    save_content_item, delete_content_item, publish_content_item,
    archive_content_item, get_content_item_versions, revert_content_item,
    // Also export schema CRUD functions
    create_schema, update_schema, delete_schema,
    // Server function struct types for explicit registration
    GetSchemas, GetSchema, CreateSchema, UpdateSchema, DeleteSchema,
    ListContentItems, GetContentItem, SaveContentItem, DeleteContentItem,
    PublishContentItem, ArchiveContentItem, GetContentItemVersions, RevertContentItem,
};

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
    get_faq_page, save_faq_page, publish_faq_page,
    get_faq_page_versions, revert_faq_page, create_faq_preview,
    FaqPageResponse,
    GetFaqPage, SaveFaqPage, PublishFaqPage,
    GetFaqPageVersions, RevertFaqPage, CreateFaqPreview,
};

#[cfg(feature = "ssr")]
pub use faq_client::register_faq_server_fns;

pub use home_client::{
    get_home_page, save_home_page, publish_home_page,
    get_home_page_versions, revert_home_page,
    HomePageResponse,
    GetHomePage, SaveHomePage, PublishHomePage,
    GetHomePageVersions, RevertHomePage,
};

#[cfg(feature = "ssr")]
pub use home_client::register_home_server_fns;

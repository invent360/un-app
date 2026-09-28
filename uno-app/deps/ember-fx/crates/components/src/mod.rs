//! UI Components for ember-fx.
//!
//! This module contains all UI components built on the core trait system.
//! Components are feature-gated for modularity.
//!
//! ## Available Components
//!
//! - `button` - Button component with variants (primary, secondary, outline, etc.)
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx::components::Button;
//!
//! view! {
//!     <Button variant=ButtonVariant::Primary>
//!         "Click me"
//!     </Button>
//! }
//! ```

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "button")]
pub use button::{Button, ButtonVariant, ButtonSize};

#[cfg(feature = "panel")]
pub mod panel;

#[cfg(feature = "panel")]
pub use panel::{
    StatCard, StatGroup, Trend, Field, InlineField,
    TrendFlag, TrendColorMode, StatCardSize,
};

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "input")]
pub use input::{
    TextInput, PasswordInput, TextArea,
    InputGroup, InputAddon, InputButton,
    InputVariant, InputSize, ValidationState, InputType,
};

#[cfg(feature = "selection")]
pub mod selection;

#[cfg(feature = "selection")]
pub use selection::{
    Checkbox, CheckboxGroup,
    Radio, RadioGroup,
    Switch,
    Select, MultiSelect,
    SelectionSize, CheckStyle, GroupLayout,
    DropdownPlacement, SelectOption, StringOption,
};

#[cfg(feature = "notification")]
pub mod notification;

#[cfg(feature = "notification")]
pub use notification::{
    Alert, AlertType,
    Badge, BadgeStatus,
    Tag, CheckableTag, TagColor,
    Progress, ProgressType, ProgressStatus,
    Spinner, Loading, SpinnerSize,
    Skeleton, SkeletonButton, SkeletonInput, SkeletonImage,
    Toast, ToastProvider, ToastItem, ToastContext, ToastPlacement,
    use_toast, try_use_toast,
};

#[cfg(feature = "layout")]
pub mod layout;

#[cfg(feature = "layout")]
pub use layout::{
    Card, CardSize,
    Modal, ModalSize,
    Drawer, DrawerPlacement, DrawerSize,
    Tabs, TabPane, TabItem, TabPosition, TabType,
    Collapse, CollapsePanel, CollapseIconPosition,
    Divider, DividerType, DividerOrientation,
};

#[cfg(feature = "overlay")]
pub mod overlay;

#[cfg(feature = "overlay")]
pub use overlay::{
    Modal as OverlayModal,
    ConfirmModal,
    ConfirmType,
    ModalSize as OverlayModalSize,
    ModalAnimation,
    ConfirmButtonConfig,
    ConfirmOptions,
    InfoCircleFilled,
    CheckCircleFilled,
    ExclamationCircleFilled,
    CloseCircleFilled,
    ConfirmIcon,
    CloseOutlined,
};

#[cfg(feature = "navigation")]
pub mod navigation;

#[cfg(feature = "navigation")]
pub use navigation::{
    Menu, MenuItem, MenuMode, MenuTheme,
    Breadcrumb, BreadcrumbItem, BreadcrumbSeparator,
    Pagination, PaginationSize,
    Steps, Step, StepItem, StepsDirection, StepsType, StepsSize, StepStatus,
    StepsIconType, StepsLineWeight, StepsLabelPlacement,
};

#[cfg(feature = "data")]
pub mod data;

#[cfg(feature = "data")]
pub use data::{
    Avatar, AvatarGroup, AvatarSize, AvatarShape,
    Tooltip, TooltipPlacement, TooltipTrigger,
    Popover, PopoverPlacement, PopoverTrigger,
    List, ListItem, ListItemMeta, ListSize, ListLayout,
    Table, TableColumn, TableSize, ColumnAlign, ColumnFixed, SortOrder,
};

#[cfg(feature = "visualization")]
pub mod visualization;

#[cfg(feature = "visualization")]
pub use visualization::{
    Empty,
    Result, ResultStatus,
    Statistic, StatisticGroup, Countdown, StatisticValueStyle,
    Timeline, TimelineItemComponent, TimelineItem, TimelineMode, TimelineItemColor,
    Descriptions, DescriptionsItem, DescriptionItem, DescriptionsLayout, DescriptionsSize,
    Tree, TreeNode,
};

#[cfg(feature = "form-advanced")]
pub mod form_advanced;

#[cfg(feature = "form-advanced")]
pub use form_advanced::{
    Slider, SliderSize, SliderMark,
    Rate, RateCharacter,
    DatePicker, DatePickerMode, DatePickerSize,
    TimePicker, TimePickerSize,
    Transfer, TransferItem, TransferDirection,
    Upload, UploadDragger, UploadListType, UploadFileStatus, UploadFile,
};

#[cfg(feature = "guide")]
pub mod guide;

#[cfg(feature = "guide")]
pub use guide::{
    // Types
    GuideStep, GuideMetadata, GuideVariant, GuideSize, GuideTransition,
    DrawerPosition, ProgressStyle, TourPlacement,
    // Components
    GuideCarousel, GuideDrawer, GuideAccordion, GuideTour,
    GuideStepView, GuideProgress, GuideControls,
    ImagePosition, TourBeacon,
};

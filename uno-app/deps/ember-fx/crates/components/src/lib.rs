#![recursion_limit = "512"]

//! # ember-fx-components
//!
//! UI component library for ember-fx.
//!
//! This crate provides a comprehensive set of UI components built on the
//! ember-fx theming system. Components are feature-gated for modularity.
//!
//! ## Component Categories
//!
//! - **Button** - Buttons with variants and states
//! - **Input** - Text inputs, password fields, textareas
//! - **Selection** - Checkboxes, radios, switches, selects
//! - **Notification** - Alerts, badges, tags, progress, spinners, toasts
//! - **Layout** - Cards, modals, drawers, tabs, collapse, dividers
//! - **Navigation** - Menus, breadcrumbs, pagination, steps
//! - **Data** - Avatars, tooltips, popovers, lists, tables
//! - **Visualization** - Empty states, results, statistics, timelines, trees
//! - **Calendar** - Date/time pickers, range pickers, datetime pickers
//! - **Form Advanced** - Sliders, ratings, transfers, uploads
//! - **Form** - Wizard/multi-step forms with validation
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx_components::{Button, ButtonVariant};
//!
//! view! {
//!     <Button variant=ButtonVariant::Primary>
//!         "Click me"
//!     </Button>
//! }
//! ```

// Re-export from dependencies for convenience
pub use ember_fx_common::{DesignSystem, Size, Direction, ThemeMode};
pub use ember_fx_common::validation::{ValidationRule, ValidationResult, ValidateOn, validate_value, validate_all};
pub use ember_fx_core::{
    ThemeContext, use_theme, try_use_theme, Platform,
    ThemeProvider, MinimalThemeProvider,
};

// Core traits module
#[cfg(feature = "core")]
pub mod core;

#[cfg(feature = "core")]
pub use core::{
    FxComponent, ComponentType,
    Sizable, ComponentSize,
    Themed, Colored, Rounded, Bordered, ComponentColor, BorderRadius,
    DisabledState, LoadingState,
    Clickable, Focusable, Hoverable,
    Accessible, AriaRole,
};

// Icon showcase components
#[cfg(feature = "icons")]
pub mod icons;

#[cfg(feature = "icons")]
pub use icons::{
    IconsOverview, CryptoIconsPage, AnimalIconsPage, NatureIconsPage,
    FinanceIconsPage, CommerceIconsPage, SocialIconsPage,
};

// Template system
#[cfg(feature = "template")]
pub mod template;

#[cfg(feature = "template")]
pub use template::{
    InputTemplate, ButtonTemplate, CardTemplate, TemplateVariant,
    TemplateProvider, TemplateContext,
    use_template, try_use_template, use_input_template, use_button_template, use_card_template,
};

// Component modules
#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "button")]
pub use button::{
    // Core Button component
    Button,
    // Modern API types
    ButtonColor, ButtonStyleVariant, IconPosition, HtmlButtonType, LoadingConfig,
    // Legacy types
    ButtonVariant, ButtonSize, ButtonShape,
    // Button Group
    ButtonGroup, ButtonGroupContext, try_use_button_group,
    // Icon Button
    IconButton,
    // Float Button
    FloatButton, FloatButtonGroup, BackTop, FloatButtonPlacement,
    // Split Button
    SplitButton, SplitButtonItem,
    // Speed Dial
    SpeedDial, SpeedDialDirection, SpeedDialAction,
};

#[cfg(feature = "panel")]
pub mod panel;

#[cfg(feature = "panel")]
pub use panel::{
    StatCard, StatGroup, Trend, Field, InlineField,
    TrendFlag, TrendColorMode, StatCardSize, StatIcon,
};

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "input")]
pub use input::{
    TextInput, PasswordInput, TextArea,
    InputGroup, InputAddon, InputButton,
    FormField, ReactiveFormField,
    InputVariant, InputSize, ValidationState, InputType,
    InputNumber, StepperPosition,
    SearchInput, SearchButton,
    OtpInput, OtpLength,
    AutoComplete, AutoCompleteOption,
    InputMask, MaskType,
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
    TreeSelect, TreeSelectNode,
    Cascader, CascaderOption, CascaderTrigger,
    Mentions, MentionTrigger, MentionOption,
    TriaState, TriaStateOption,
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
    StepsIconType, StepsLineWeight, StepsLabelPlacement, StepsResponsive,
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

#[cfg(feature = "chart")]
pub mod chart;

#[cfg(feature = "chart")]
pub use chart::{
    LineChart, AreaChart, SparklineChart,
    DataPoint, LineChartConfig, AreaChartConfig, SparklineConfig,
    AreaSeries, ChartColor, ChartSeries, SeriesType,
};

#[cfg(feature = "observability")]
pub mod observability;

#[cfg(feature = "observability")]
pub use observability::{
    LogViewer, TraceViewer, AlertCard, AlertList, ActivityViewer,
    LogEntry, LogLevel, LogFilter,
    Trace, Span, SpanId, SpanStatus,
    Alert, AlertId, Severity, AlertState,
    Activity, ActivityId, ActivityCategory, ActivityFilter,
    MetricSelector, MetricInfo, MetricType,
    NetworkTopologyMap, TopologyNode, TopologyEdge, TopologyConfig,
    TopologyNodeType, TopologyNodeStatus, TopologyEdgeType, TopologyLayout,
    NodePosition, calculate_force_layout, calculate_circular_layout,
};

#[cfg(feature = "maps")]
pub mod maps;

#[cfg(feature = "maps")]
pub use maps::{
    // HexagonalWorldMap
    HexagonalWorldMap, HexagonalWorldMapLegend,
    HexCell, MapLocation, LocationIcon, HexOrientation, MapProjection, ColorScale,
    HexWorldMapConfig,
    // ProgressRing
    ProgressRing, ProgressRingData, ProgressRingConfig,
    // ArcFlowLines
    ArcFlowLines, ArcPoint, ArcConnection, ArcStyle, ArcFlowConfig,
    // GradientProgressBar
    GradientProgressBar, ProgressSegment, GradientProgressBarConfig,
    // CityMarker
    CityMarker, CityMarkerGroup, CityMarkerData, MarkerIcon, MarkerPosition, MarkerSize, CityMarkerConfig,
    // ActivityList
    ActivityList, ActivityItem, TrendDirection, ActivitySortOrder, ActivityListConfig,
    // Utilities
    hexagon_path, lat_lng_to_pixel, hex_to_pixel, arc_connection_path, progress_arc_path, format_number,
};

#[cfg(feature = "slider")]
pub mod slider;

#[cfg(feature = "slider")]
pub use slider::{
    Slider, RangeSlider, SliderVariant,
    SliderSize, SliderOrientation, SliderMark,
    TooltipVisibility, TooltipConfig,
    RangeConfig, SliderClassNames, SliderStyles,
};

// Re-export slider's TooltipPlacement with an alias to avoid conflict with data::TooltipPlacement
#[cfg(feature = "slider")]
pub use slider::TooltipPlacement as SliderTooltipPlacement;

#[cfg(feature = "calendar")]
pub mod calendar;

#[cfg(feature = "calendar")]
pub use calendar::{
    DatePicker, TimePicker,
    DateRangePicker, TimeRangePicker, DateTimePicker,
    DatePickerMode, DatePickerSize, TimePickerSize, TimeFormat,
    DateRangeValue, TimeRangeValue, DateTimeValue, DateRangePreset,
};

#[cfg(feature = "form-advanced")]
pub mod form_advanced;

#[cfg(feature = "form-advanced")]
pub use form_advanced::{
    Rate, RateCharacter,
    Transfer, TransferItem, TransferDirection,
    Upload, UploadDragger, UploadListType, UploadFileStatus, UploadFile,
    ColorPicker, ColorFormat, ColorPickerSize, ColorPreset, ColorValue,
};

#[cfg(feature = "form")]
pub mod form;

#[cfg(feature = "form")]
pub use form::{
    // Core types
    WizardStage,
    WizardLayout,
    StepperOrientation,
    NavigationAlignment,
    ErrorDisplayStyle,
    ProgressStyle,
    ValidationError,
    // Config
    WizardConfig,
    StepConfig,
    ButtonConfig,
    // Actions
    WizardAction,
    WizardResult,
    // State
    WizardState,
    // Orchestrator
    Wizard,
    // Validators
    StageValidator,
    NoOpValidator,
    FnValidator,
    CompositeValidator,
    AlwaysFailValidator,
    // UI Components
    WizardContainer,
    WizardStep,
    WizardContent,
    WizardStepper,
    WizardProgress,
    WizardCircularProgress,
    WizardStepCounter,
    WizardNavigation,
    WizardCompactNavigation,
    NavigationLabels,
    WizardErrorDisplay,
    WizardSuccessDisplay,
};

#[cfg(feature = "media")]
pub mod media;

#[cfg(feature = "media")]
pub use media::{
    Carousel, CarouselSlide, CarouselSize, CarouselEffect, CarouselMode,
    ThumbnailPosition, PaginationPosition, SlideContent,
    SlidesQty, ResponsiveSlidesQty, LoadingClasses,
};

#[cfg(feature = "task")]
pub mod task;

#[cfg(feature = "task")]
pub use task::{
    TaskCard, TaskCardGrid, TaskData, TaskStatus, TaskDifficulty,
    TaskCardSize, TaskCardVariant, TaskCardLayout, EarningsTier, EarningsPeriod,
};

#[cfg(feature = "guide")]
pub mod guide;

#[cfg(feature = "guide")]
pub use guide::{
    // Types
    GuideStep, GuideMetadata, GuideVariant, GuideSize, GuideTransition,
    DrawerPosition, ProgressStyle as GuideProgressStyle, TourPlacement,
    StepperOrientation as GuideStepperOrientation, StepperHeaderPosition, StepStatus as GuideStepStatus,
    StepLayout, ControlsPosition, StepperHeaderLayout,
    // Components
    GuideCarousel, GuideDrawer, GuideAccordion, GuideTour, GuideStepper,
    GuideStepView, GuideProgress, GuideControls,
    ImagePosition, TourBeacon,
};

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::{DesignSystem, Size, ThemeContext, use_theme, try_use_theme, Platform};

    #[cfg(feature = "icons")]
    pub use crate::{IconsOverview, CryptoIconsPage, AnimalIconsPage, NatureIconsPage, FinanceIconsPage, CommerceIconsPage, SocialIconsPage};

    #[cfg(feature = "core")]
    pub use crate::core::{
        FxComponent, ComponentType, ComponentSize, Sizable,
        Themed, Colored, ComponentColor, DisabledState, LoadingState,
        Clickable, Focusable, Accessible,
    };

    #[cfg(feature = "template")]
    pub use crate::template::{
        InputTemplate, ButtonTemplate, CardTemplate,
        TemplateProvider, use_input_template, use_button_template,
    };

    #[cfg(feature = "button")]
    pub use crate::{Button, ButtonVariant, ButtonSize, ButtonGroup, IconButton, FloatButton, SpeedDial};

    #[cfg(feature = "input")]
    pub use crate::{TextInput, PasswordInput, TextArea, InputVariant, InputSize, InputNumber, SearchInput, OtpInput, AutoComplete};

    #[cfg(feature = "notification")]
    pub use crate::{Alert, AlertType, Badge, Tag, Progress, Spinner, ToastProvider, use_toast};

    #[cfg(feature = "layout")]
    pub use crate::{Card, Modal, Drawer, Tabs, Collapse, Divider};

    #[cfg(feature = "navigation")]
    pub use crate::{Menu, MenuItem, Breadcrumb, Pagination, Steps};

    #[cfg(feature = "data")]
    pub use crate::{Avatar, Tooltip, Popover, List, Table};

    #[cfg(feature = "visualization")]
    pub use crate::{Empty, Result, Statistic, Timeline, Descriptions, Tree};

    #[cfg(feature = "chart")]
    pub use crate::{LineChart, AreaChart, SparklineChart, DataPoint, ChartColor};

    #[cfg(feature = "calendar")]
    pub use crate::{DatePicker, TimePicker, DateRangePicker, TimeRangePicker, DateTimePicker};

    #[cfg(feature = "form")]
    pub use crate::{
        WizardStage, WizardConfig, Wizard, WizardState,
        WizardContainer, WizardStep, WizardStepper, WizardProgress, WizardNavigation,
        StageValidator, NoOpValidator, ValidationError,
    };

    #[cfg(feature = "observability")]
    pub use crate::{LogViewer, TraceViewer, AlertCard, AlertList, ActivityViewer, LogEntry, LogLevel, Activity, ActivityCategory};

    #[cfg(feature = "maps")]
    pub use crate::{
        HexagonalWorldMap, ProgressRing, ArcFlowLines, GradientProgressBar,
        CityMarker, ActivityList, HexCell, MapLocation, ProgressRingData,
    };

    #[cfg(feature = "media")]
    pub use crate::{Carousel, CarouselSlide, CarouselSize};

    #[cfg(feature = "slider")]
    pub use crate::{Slider, RangeSlider, SliderSize, SliderOrientation, SliderMark, TooltipConfig, RangeConfig};

    #[cfg(feature = "task")]
    pub use crate::{TaskCard, TaskCardGrid, TaskData, TaskStatus, EarningsTier};
}

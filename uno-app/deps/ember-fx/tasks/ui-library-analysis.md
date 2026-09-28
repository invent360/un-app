# UI Component Library Analysis

**Date**: 2026-05-09
**Libraries Analyzed**: 9
**Focus**: Input and Button component variants

---

## Executive Summary

Analyzed 9 open-source UI component libraries to catalog Input and Button variants for ember-fx component extension. Total findings:

| Category | Total Variants Found |
|----------|---------------------|
| **Input Components** | 88+ distinct types |
| **Button Components** | 51+ distinct types |

---

## INPUT COMPONENTS

### Consolidated Input Catalog

#### 1. Text Input Variants

| Variant | Found In | Description |
|---------|----------|-------------|
| **Input** | Ant Design, Material UI, PrimeReact, shadcn/ui | Basic text input |
| **Input.TextArea** | Ant Design | Multi-line text input |
| **TextArea** | PrimeReact, shadcn/ui | Multi-line text |
| **TextField** | Material UI | Text field with label integration |
| **InputBase** | Material UI | Unstyled base for custom inputs |
| **OutlinedInput** | Material UI | Outlined variant |
| **FilledInput** | Material UI | Filled variant |
| **Input.OTP** | Ant Design | One-time password input |
| **InputOtp** | PrimeReact | OTP/verification code input |
| **PasscodeInput** | Ant Design Mobile | Mobile passcode entry |
| **VirtualInput** | Ant Design Mobile | Virtual keyboard input |
| **InputMask** | PrimeReact | Masked/formatted input |

#### 2. Password Input Variants

| Variant | Found In | Description |
|---------|----------|-------------|
| **Input.Password** | Ant Design | Password with visibility toggle |
| **Password** | PrimeReact | Password field |

#### 3. Search Input Variants

| Variant | Found In | Description |
|---------|----------|-------------|
| **Input.Search** | Ant Design | Search with icon/button |
| **Search** | Base UI | Headless search input |

#### 4. Numeric Input Variants

| Variant | Found In | Description |
|---------|----------|-------------|
| **InputNumber** | Ant Design | Number with +/- controls |
| **InputNumber** | PrimeReact | Numeric stepper |
| **NumberInput** | Base UI | Headless number input |
| **Number Field** | Base UI | Number with constraints |

#### 5. Selection Inputs

| Variant | Found In | Description |
|---------|----------|-------------|
| **Select** | Ant Design, Base UI, Material UI, PrimeReact, shadcn/ui | Dropdown selection |
| **TreeSelect** | Ant Design | Hierarchical selection |
| **Cascader** | Ant Design | Multi-level cascading selection |
| **AutoComplete** | Ant Design, Material UI, PrimeReact | Type-ahead suggestions |
| **Mentions** | Ant Design | @mention input |
| **MultiSelect** | PrimeReact | Multiple selection dropdown |
| **Listbox** | PrimeReact | List-based selection |

#### 6. Date/Time Inputs

| Variant | Found In | Description |
|---------|----------|-------------|
| **DatePicker** | Ant Design, MUI X | Single date selection |
| **DateRangePicker** | MUI X | Date range selection |
| **TimePicker** | Ant Design, MUI X | Time selection |
| **TimeRangePicker** | MUI X | Time range selection |
| **DateTimePicker** | MUI X | Combined date + time |
| **DateTimeRangePicker** | MUI X | Combined range |
| **Calendar** | Ant Design, PrimeReact | Full calendar picker |
| **MonthPicker** | MUI X | Month-only selection |
| **YearPicker** | MUI X | Year-only selection |
| **DigitalClock** | MUI X | Digital time display |
| **MultiInputDateRangeField** | MUI X | Dual-field date range |
| **MultiInputTimeRangeField** | MUI X | Dual-field time range |
| **SingleInputDateRangeField** | MUI X | Single-field date range |
| **SingleInputTimeRangeField** | MUI X | Single-field time range |

#### 7. File/Upload Inputs

| Variant | Found In | Description |
|---------|----------|-------------|
| **Upload** | Ant Design | File upload |
| **Upload.Dragger** | Ant Design | Drag-and-drop upload |
| **FileUpload** | PrimeReact | File upload component |

#### 8. Toggle/Switch Inputs

| Variant | Found In | Description |
|---------|----------|-------------|
| **Switch** | Ant Design, Base UI, Material UI, PrimeReact, shadcn/ui | Boolean toggle |
| **Checkbox** | Ant Design, Base UI, Material UI, PrimeReact, shadcn/ui | Checkable input |
| **Checkbox.Group** | Ant Design | Grouped checkboxes |
| **Radio** | Ant Design, Base UI, Material UI, PrimeReact, shadcn/ui | Single selection |
| **Radio.Group** | Ant Design | Grouped radios |
| **Radio.Button** | Ant Design | Button-styled radio |
| **ToggleButton** | Material UI, PrimeReact | Toggleable button |
| **ToggleButtonGroup** | Material UI | Grouped toggles |
| **TriStateCheckbox** | PrimeReact | Three-state checkbox |
| **SelectButton** | PrimeReact | Button-based selection |

#### 9. Slider/Range Inputs

| Variant | Found In | Description |
|---------|----------|-------------|
| **Slider** | Ant Design, Base UI, Material UI, PrimeReact | Value slider |
| **Slider.Range** | Ant Design | Dual-handle range slider |
| **Rating** | Ant Design (Rate), Material UI, PrimeReact | Star/icon rating |
| **Knob** | PrimeReact | Circular slider |

#### 10. Color Inputs

| Variant | Found In | Description |
|---------|----------|-------------|
| **ColorPicker** | Ant Design, PrimeReact | Color selection |

#### 11. Rich Text Inputs

| Variant | Found In | Description |
|---------|----------|-------------|
| **Editor** | PrimeReact | Rich text editor |
| **KeyFilter** | PrimeReact | Input key filtering |

#### 12. Pro/Advanced Form Inputs (Ant Design Pro)

| Variant | Found In | Description |
|---------|----------|-------------|
| **ProFormText** | Ant Design Pro | Enhanced text input |
| **ProFormTextArea** | Ant Design Pro | Enhanced textarea |
| **ProFormDigit** | Ant Design Pro | Enhanced number |
| **ProFormSelect** | Ant Design Pro | Enhanced select |
| **ProFormTreeSelect** | Ant Design Pro | Enhanced tree select |
| **ProFormCascader** | Ant Design Pro | Enhanced cascader |
| **ProFormDatePicker** | Ant Design Pro | Enhanced date picker |
| **ProFormDateTimePicker** | Ant Design Pro | Enhanced datetime |
| **ProFormDateRangePicker** | Ant Design Pro | Enhanced date range |
| **ProFormTimePicker** | Ant Design Pro | Enhanced time picker |
| **ProFormUploadButton** | Ant Design Pro | Upload as button |
| **ProFormUploadDragger** | Ant Design Pro | Drag upload |
| **ProFormSwitch** | Ant Design Pro | Enhanced switch |
| **ProFormCheckbox** | Ant Design Pro | Enhanced checkbox |
| **ProFormRadio** | Ant Design Pro | Enhanced radio |
| **ProFormSlider** | Ant Design Pro | Enhanced slider |
| **ProFormRate** | Ant Design Pro | Enhanced rating |
| **ProFormColorPicker** | Ant Design Pro | Enhanced color picker |
| **ProFormMoney** | Ant Design Pro | Currency input |
| **ProFormCaptcha** | Ant Design Pro | Captcha input |

#### 13. Data Grid Inputs (MUI X)

| Variant | Found In | Description |
|---------|----------|-------------|
| **GridFilterInputValue** | MUI X | Grid filter text |
| **GridFilterInputDate** | MUI X | Grid filter date |
| **GridFilterInputBoolean** | MUI X | Grid filter boolean |
| **GridFilterInputMultipleValue** | MUI X | Grid filter multi-select |
| **GridEditInputCell** | MUI X | Inline edit cell |
| **GridEditDateCell** | MUI X | Inline edit date |
| **GridEditSingleSelectCell** | MUI X | Inline edit select |
| **GridEditBooleanCell** | MUI X | Inline edit checkbox |

#### 14. Preline (Tailwind Plugins)

| Plugin | Description |
|--------|-------------|
| **input-number** | Number stepper |
| **pin-input** | PIN/OTP entry |
| **strong-password** | Password strength |
| **select** | Enhanced select |
| **combobox** | Autocomplete combo |
| **toggle-password** | Password visibility |
| **file-upload** | File upload |
| **textarea-auto-height** | Auto-resize textarea |
| **input-with-tags** | Tag input |
| **copy-markup** | Copy-to-clipboard |

---

## BUTTON COMPONENTS

### Consolidated Button Catalog

#### 1. Core Button Variants

| Variant | Found In | Description |
|---------|----------|-------------|
| **Button** | All libraries | Primary button component |
| **Button (primary)** | Ant Design, PrimeReact | Primary action |
| **Button (secondary)** | shadcn/ui | Secondary action |
| **Button (default)** | Ant Design | Default styling |
| **Button (dashed)** | Ant Design | Dashed border |
| **Button (text)** | Ant Design, Material UI | Text-only button |
| **Button (link)** | Ant Design | Link-styled button |
| **Button (outlined)** | Material UI, PrimeReact | Outlined variant |
| **Button (contained)** | Material UI | Filled/solid variant |
| **Button (destructive)** | shadcn/ui | Delete/danger action |
| **Button (ghost)** | shadcn/ui | Transparent background |

#### 2. Icon Buttons

| Variant | Found In | Description |
|---------|----------|-------------|
| **IconButton** | Material UI | Icon-only button |
| **Button (icon)** | Ant Design, shadcn/ui | Icon button variant |
| **Button (circle)** | Ant Design | Circular icon button |
| **FloatingBubble** | Ant Design Mobile | Floating action icon |

#### 3. Button Groups

| Variant | Found In | Description |
|---------|----------|-------------|
| **Button.Group** | Ant Design | Grouped buttons |
| **ButtonGroup** | Material UI, PrimeReact | Button grouping |
| **SplitButton** | PrimeReact | Button with dropdown |

#### 4. Loading/State Buttons

| Variant | Found In | Description |
|---------|----------|-------------|
| **Button (loading)** | Ant Design, PrimeReact, shadcn/ui | Loading state |
| **LoadingButton** | Material UI (Lab) | Async action button |
| **Button (disabled)** | All libraries | Disabled state |

#### 5. Size Variants

| Variant | Found In | Description |
|---------|----------|-------------|
| **Button (small/sm)** | All libraries | Small size |
| **Button (medium/md)** | All libraries | Medium size (default) |
| **Button (large/lg)** | All libraries | Large size |
| **Button (block)** | Ant Design | Full-width button |

#### 6. Floating Action Buttons

| Variant | Found In | Description |
|---------|----------|-------------|
| **FloatButton** | Ant Design | Floating action button |
| **FloatButton.Group** | Ant Design | FAB group |
| **FloatButton.BackTop** | Ant Design | Scroll-to-top FAB |
| **Fab** | Material UI | Floating action button |
| **SpeedDial** | Material UI, PrimeReact | Expanding FAB menu |
| **SpeedDialAction** | Material UI | FAB action item |

#### 7. Special Purpose Buttons

| Variant | Found In | Description |
|---------|----------|-------------|
| **Radio.Button** | Ant Design | Button-styled radio |
| **ToggleButton** | Material UI, PrimeReact | State toggle |
| **ToggleButtonGroup** | Material UI | Grouped toggles |
| **SelectButton** | PrimeReact | Selection button |

#### 8. Pro/Enterprise Buttons (MUI X)

| Variant | Found In | Description |
|---------|----------|-------------|
| **GridActionsCellItem** | MUI X | Data grid action button |
| **GridToolbarExport** | MUI X | Export action |
| **GridToolbarColumnsButton** | MUI X | Column visibility |
| **GridToolbarFilterButton** | MUI X | Filter toggle |
| **GridToolbarDensitySelector** | MUI X | Density toggle |
| **GridToolbarQuickFilterButton** | MUI X | Quick filter |
| **PickersActionBar** | MUI X | Picker action buttons |
| **ChartLegendItemButton** | MUI X | Chart legend toggle |

#### 9. Base/Headless Buttons

| Variant | Found In | Description |
|---------|----------|-------------|
| **Button** | Base UI | Unstyled button primitive |
| **useButton** | Base UI | Button hook |

---

## LIBRARY SUMMARIES

### 1. Ant Design
- **Inputs**: 10 core types + compound variants
- **Buttons**: 10 types + groups
- **Pattern**: Compound components (Input.Password, Button.Group)
- **Strengths**: Complete enterprise component set

### 2. Ant Design Mobile
- **Inputs**: 3 types (Input, PasscodeInput, VirtualInput)
- **Buttons**: 2 types (Button, FloatingBubble)
- **Pattern**: Mobile-optimized, gesture-aware
- **Strengths**: Touch interactions, mobile UX

### 3. Ant Design Pro
- **Inputs**: 20+ ProForm* components
- **Buttons**: Inherits from Ant Design
- **Pattern**: Form-integrated, schema-driven
- **Strengths**: Enterprise forms, validation, layouts

### 4. Material UI (MUI)
- **Inputs**: 10 components
- **Buttons**: 6 core + FAB variants
- **Pattern**: Layered (Base → Styled)
- **Strengths**: Theming system, Material Design compliance

### 5. Base UI
- **Inputs**: 7 headless components
- **Buttons**: 2 (Button, useButton hook)
- **Pattern**: Unstyled primitives + hooks
- **Strengths**: Full styling control, accessibility built-in

### 6. MUI X
- **Inputs**: 22 components (pickers, grid inputs)
- **Buttons**: 14 specialized buttons
- **Pattern**: Feature-rich, data-focused
- **Strengths**: Date/time pickers, data grid integration

### 7. PrimeReact
- **Inputs**: 10 components
- **Buttons**: 5 types
- **Pattern**: Self-contained, theme-aware
- **Strengths**: Rich feature set, multiple themes

### 8. Preline
- **Inputs**: 11 plugins (Tailwind-based)
- **Buttons**: 0 plugins (CSS-only via Tailwind)
- **Pattern**: Plugin + Tailwind classes
- **Strengths**: Lightweight, utility-first

### 9. shadcn/ui
- **Inputs**: 3 types × 14 themes
- **Buttons**: 2 types × 14 themes
- **Pattern**: Copy-paste components, CVA variants
- **Strengths**: Customization, Radix primitives, theming

---

## IMPLEMENTATION RECOMMENDATIONS

### Priority Input Components for ember-fx

1. **Core Inputs** (High Priority)
   - Text, Password, TextArea
   - Number with stepper
   - Search with icon

2. **Selection Inputs** (High Priority)
   - Select (single/multi)
   - AutoComplete
   - Cascader/TreeSelect

3. **Date/Time Inputs** (Medium Priority)
   - DatePicker
   - TimePicker
   - DateRangePicker

4. **Toggle Inputs** (High Priority)
   - Switch
   - Checkbox/CheckboxGroup
   - Radio/RadioGroup

5. **Advanced Inputs** (Lower Priority)
   - Upload/Dragger
   - ColorPicker
   - Slider/Range

### Priority Button Components for ember-fx

1. **Core Buttons** (High Priority)
   - Primary, Default, Dashed, Text, Link variants
   - Icon button
   - Loading state

2. **Button Groups** (Medium Priority)
   - Button.Group
   - SplitButton

3. **Floating Buttons** (Lower Priority)
   - FloatButton
   - BackTop

### Architectural Patterns to Adopt

1. **Compound Components** (Ant Design pattern)
   ```rust
   // Input.Password, Input.Search, Input.OTP
   ```

2. **Variant System** (shadcn/CVA pattern)
   ```rust
   // ButtonVariant::Primary, Secondary, Destructive, Ghost
   ```

3. **Size Tokens** (Universal pattern)
   ```rust
   // Small, Medium, Large, Block
   ```

4. **State Management** (All libraries)
   ```rust
   // Loading, Disabled, Error, Success states
   ```

---

## APPENDIX: Component Count by Library

| Library | Input Types | Button Types | Total |
|---------|------------|--------------|-------|
| Ant Design | 10 | 10 | 20 |
| Ant Design Mobile | 3 | 2 | 5 |
| Ant Design Pro | 20+ | 10 | 30+ |
| Material UI | 10 | 6 | 16 |
| Base UI | 7 | 2 | 9 |
| MUI X | 22 | 14 | 36 |
| PrimeReact | 10 | 5 | 15 |
| Preline | 11 | 0 | 11 |
| shadcn/ui | 3×14 | 2×14 | 70 |
| **TOTAL** | **88+** | **51+** | **139+** |


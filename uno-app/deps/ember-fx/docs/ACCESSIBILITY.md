# Accessibility (a11y) in ember-fx

This document describes the accessibility features and best practices implemented in ember-fx components.

## Overview

ember-fx is designed with accessibility as a core requirement. All interactive components follow WAI-ARIA guidelines and support keyboard navigation, screen readers, and other assistive technologies.

## Utilities

The `ember-fx-utils` crate provides accessibility utilities in the `a11y` module:

```rust
use ember_fx_utils::a11y::{
    FocusTrap, RovingTabindex, Orientation, AnnounceLevel,
    announce, generate_id, navigate_list, is_activation_key, is_dismiss_key,
    focus_first, focus_last, get_focusable_elements,
    keys, SR_ONLY_CLASS, SR_ONLY_CSS, FOCUSABLE_SELECTOR,
};
```

### Focus Trap

Keep keyboard focus within a container (essential for modals, drawers, dialogs):

```rust
let trap = FocusTrap::new("modal-container");
trap.activate();   // Trap focus
trap.deactivate(); // Release focus and restore previous focus
```

### Roving Tabindex

Manage keyboard navigation within component groups (menus, tabs, radio groups):

```rust
let mut roving = RovingTabindex::new(5); // 5 items
roving.orientation = Orientation::Vertical;
roving.wrap = true;

// Navigate with arrow keys
roving.move_next();     // Move to next item
roving.move_previous(); // Move to previous item
```

### Keyboard Navigation Helpers

```rust
// Navigate a list with arrow keys
let new_index = navigate_list("ArrowDown", current_index, total_items, wrap);

// Check for activation keys (Enter, Space)
if is_activation_key(&key) {
    activate_item();
}

// Check for dismiss keys (Escape)
if is_dismiss_key(&key) {
    close_modal();
}
```

### Screen Reader Announcements

```rust
// Announce dynamic content changes
announce("Item added to cart", AnnounceLevel::Polite);
announce("Error: Invalid input", AnnounceLevel::Assertive);
```

### Key Constants

```rust
use ember_fx_utils::a11y::keys;

keys::ENTER      // "Enter"
keys::SPACE      // " "
keys::ESCAPE     // "Escape"
keys::TAB        // "Tab"
keys::ARROW_UP   // "ArrowUp"
keys::ARROW_DOWN // "ArrowDown"
keys::ARROW_LEFT // "ArrowLeft"
keys::ARROW_RIGHT// "ArrowRight"
keys::HOME       // "Home"
keys::END        // "End"
```

## Component Accessibility

### Button

- Full keyboard support (Enter/Space activation)
- `role="button"` for non-button elements
- `aria-disabled` for disabled state (preserves focus)
- `aria-pressed` for toggle buttons
- `aria-label` support for icon-only buttons

```rust
<Button
    aria_label="Close dialog"
    disabled=true
>
    <Icon icon="x" />
</Button>
```

### Modal / Dialog

- `role="dialog"` and `aria-modal="true"`
- `aria-labelledby` pointing to title
- Focus trap (Tab cycles within modal)
- Escape key closes modal
- Focus restored to trigger on close

```rust
<Modal
    open=show_modal
    title="Confirm Action"
    on_close=move || show_modal.set(false)
>
    <p>"Are you sure?"</p>
</Modal>
```

### Drawer

- Same accessibility features as Modal
- `role="dialog"` with `aria-modal="true"`
- Focus trap and Escape key support

### Input / TextInput

- `aria-invalid` for validation errors
- `aria-describedby` linking to error messages
- `aria-required` for required fields
- Label association via `id`

```rust
<TextInput
    id="email"
    value=email
    aria_invalid=has_error
    aria_describedby=error_id
/>
```

### Checkbox / Radio / Switch

- Native `<input type="checkbox/radio">` for built-in a11y
- `aria-checked` for custom implementations
- Label association via `id` or wrapping `<label>`

### Select / Dropdown

- `role="combobox"` on input
- `role="listbox"` on dropdown
- `role="option"` on items
- `aria-expanded` for open state
- `aria-activedescendant` for current selection
- Arrow key navigation

### Menu

- `role="menu"` on container
- `role="menuitem"` on items
- Arrow key navigation (up/down or left/right based on orientation)
- Home/End to jump to first/last
- Type-ahead search

### Slider

- `role="slider"`
- `aria-valuenow`, `aria-valuemin`, `aria-valuemax`
- `aria-valuetext` for human-readable value
- Arrow keys for increment/decrement

### Alert

- `role="alert"` for important messages
- `aria-live="polite"` or `aria-live="assertive"`

### Pagination

- `aria-label="Pagination"`
- `aria-current="page"` for current page
- Clear button labels ("Go to page 3")

### Breadcrumb

- `aria-label="Breadcrumb"`
- `aria-current="page"` for current location

### Steps

- `aria-label="Progress"`
- `aria-current="step"` for current step
- Status indicators (complete, current, pending)

### Collapse / Accordion

- `aria-expanded` on trigger
- `aria-controls` linking to panel
- Panel has unique `id`

### Tooltip

- `role="tooltip"`
- Trigger has `aria-describedby` pointing to tooltip
- Tooltip appears on focus as well as hover

### Tabs

- `role="tablist"` on container
- `role="tab"` on tabs
- `role="tabpanel"` on panels
- `aria-selected` for active tab
- Arrow key navigation between tabs

## Best Practices

### 1. Use Semantic HTML

Prefer native HTML elements when possible:

```rust
// Good - native button
<button on:click=handle_click>"Click me"</button>

// Avoid - div with role
<div role="button" tabindex="0" on:click=handle_click>"Click me"</div>
```

### 2. Provide Text Alternatives

All images and icons should have text alternatives:

```rust
<Icon icon="search" aria_label="Search" />
<img src="logo.png" alt="Company Logo" />
```

### 3. Ensure Keyboard Access

All interactive elements must be keyboard accessible:

```rust
<div
    role="button"
    tabindex="0"
    on:click=handle_click
    on:keydown=move |ev| {
        if is_activation_key(&ev.key()) {
            handle_click(());
        }
    }
>
    "Clickable div"
</div>
```

### 4. Manage Focus

Focus should be predictable and visible:

- Modal opens -> focus moves to modal
- Modal closes -> focus returns to trigger
- Dynamic content added -> announce to screen readers

### 5. Use ARIA Sparingly

ARIA should supplement, not replace, semantic HTML:

```rust
// Prefer this
<button disabled>"Disabled"</button>

// Over this
<button aria-disabled="true">"Disabled"</button>
```

### 6. Test with Assistive Technology

- VoiceOver (macOS)
- NVDA (Windows)
- JAWS (Windows)
- TalkBack (Android)
- Keyboard-only navigation

## Screen Reader Classes

Use `SR_ONLY_CLASS` for text visible only to screen readers:

```rust
<span class=SR_ONLY_CLASS>"Opens in new window"</span>
```

Include the CSS (or use the constant):

```css
.fx-sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}
```

## Testing

### Automated Testing

The E2E test suite includes accessibility tests:

```bash
cd e2e
npm test -- --grep "@a11y"
```

Tests cover:
- ARIA attributes presence
- Keyboard navigation
- Focus management
- Screen reader announcements

### Manual Testing Checklist

- [ ] Navigate entire UI with keyboard only
- [ ] Check all interactive elements have visible focus
- [ ] Verify form labels are associated correctly
- [ ] Test with screen reader (VoiceOver, NVDA)
- [ ] Verify color contrast (4.5:1 minimum)
- [ ] Check zoom to 200% works properly
- [ ] Test with reduced motion preference

## Resources

- [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM](https://webaim.org/)
- [Deque aXe](https://www.deque.com/axe/)
- [Inclusive Components](https://inclusive-components.design/)

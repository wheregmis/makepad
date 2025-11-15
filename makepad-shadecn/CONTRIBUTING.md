# Makepad Shadecn - Contributing & Extending

This guide explains how to add new components or extend existing ones in the makepad-shadecn library.

## Adding a New Component

### 1. Create the Component Crate

```bash
mkdir -p makepad-shadecn/components/label/src
```

### 2. Create Cargo.toml

`makepad-shadecn/components/label/Cargo.toml`:

```toml
[package]
name = "makepad-shadecn-label"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Shadecn Label component for Makepad"

[dependencies]
makepad-shadecn-core = { path = "../../core", version = "0.1.0" }
makepad-draw = { path = "../../../draw", version = "1.0.0" }
makepad-derive-widget = { path = "../../../widgets/derive_widget", version = "1.0.0" }
makepad-widgets = { path = "../../../widgets", version = "1.0.0" }
```

### 3. Implement the Component

`makepad-shadecn/components/label/src/lib.rs`:

```rust
use {
    crate::{
        makepad_derive_widget::*,
        makepad_draw::*,
        makepad_widgets::*,
    }
};

live_design! {
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnLabelBase = <Label> {
        width: Fit,
        height: Fit,
        text_style: {
            font_size: 14.0,
            font: {path: dep("makepad-widgets/resources/IBMPlexSans-Regular.ttf")}
        }
        draw_text: {
            text_style: {
                font_size: 14.0,
            }
            color: #0f172a
        }
    }

    pub ShadecnLabel = <ShadecnLabelBase> {}
}

#[derive(Live, LiveRegisterWidget, WidgetRef)]
pub struct ShadecnLabel {
    #[rust] widget_uid: WidgetUid,
}

impl Widget for ShadecnLabel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Event handling delegated to base Label widget
    }

    fn draw_walk(&mut self, cx: &mut Cx, scope: &mut Scope, walk: Walk) -> DrawStep {
        DrawStep::done()
    }
}

pub fn live_design(cx: &mut Cx) {
    // Component registration handled by LiveRegisterWidget derive macro
}
```

### 4. Update Workspace

Update `makepad-shadecn/Cargo.toml`:

```toml
[workspace]
members = [
    "core",
    "components/button",
    "components/input",
    "components/card",
    "components/checkbox",
    "components/label",  # Add this
    "examples/shadecn-demo",
]
```

Update root `Cargo.toml` workspace members if needed.

### 5. Update Demo

In `makepad-shadecn/examples/shadecn-demo/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
makepad-shadecn-label = { path = "../../components/label", version = "0.1.0" }
```

In `makepad-shadecn/examples/shadecn-demo/src/app.rs`:

```rust
// Add to live_design!
use makepad_shadecn_label::*;

// Add to LiveRegister impl
impl LiveRegister for AppWindow {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_shadecn_core::live_design(cx);
        makepad_shadecn_button::live_design(cx);
        // ... existing components ...
        makepad_shadecn_label::live_design(cx);  // Add this
    }
}
```

## Extending Existing Components

### Creating Button Variants

To add new button variants, extend the existing component in your app:

```rust
live_design! {
    use makepad_shadecn_button::*;

    // Create a custom variant based on ShadecnButtonPrimary
    pub MyCustomButton = <ShadecnButtonPrimary> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                // Custom color logic
                return mix(
                    mix(#6366f1, #4f46e5, self.hover),
                    mix(#4338ca, #3730a3, self.pressed),
                    self.focus
                );
            }
        }
    }
}
```

### Creating Themed Variants

Define variants for different themes in the component's Live DSL:

```rust
live_design! {
    use link::theme::*;

    pub ShadecnButtonDark = <ShadecnButtonBase> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                return mix(
                    mix(#2d3748, #1a202c, self.hover),
                    mix(#0f1419, #000000, self.pressed),
                    self.focus
                );
            }
        }
        draw_text: {
            color: #f7fafc
        }
    }
}
```

## Design Token Conventions

When creating new components, follow these conventions:

### Spacing
- Use `space_1` through `space_6` from the theme for consistent padding/margins
- `space_1` = 4px, `space_2` = 8px, etc.

### Colors
- **Backgrounds**: `bg_primary`, `bg_secondary`, `bg_tertiary`, `bg_hover`, `bg_active`, `bg_disabled`
- **Foregrounds**: `fg_primary`, `fg_secondary`, `fg_tertiary`, `fg_disabled`
- **Borders**: `border_primary`, `border_secondary`
- **Semantic**: `success`, `warning`, `error`, `info`
- **Accent**: Use `accent` for primary interactive color

### Border Radius
- `radius_sm`: 2px for small components
- `radius_md`: 4px for medium components  
- `radius_lg`: 8px for larger components
- `radius_xl`: 12px for large containers

### Font Sizes
- `font_xs`: 10px for small text
- `font_sm`: 12px for secondary text
- `font_base`: 14px for body text
- `font_lg`: 16px for larger text
- `font_xl`: 18px for headings
- `font_2xl`: 20px for large headings

## Component Architecture

### Base Structure

Every component should follow this pattern:

```rust
use {
    crate::{
        makepad_derive_widget::*,
        makepad_draw::*,
        makepad_widgets::*,
    }
};

live_design! {
    // Define Live DSL templates here
}

#[derive(Live, LiveRegisterWidget, WidgetRef)]
pub struct YourComponent {
    #[rust] widget_uid: WidgetUid,
}

impl Widget for YourComponent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) { }
    fn draw_walk(&mut self, cx: &mut Cx, scope: &mut Scope, walk: Walk) -> DrawStep { }
}

pub fn live_design(cx: &mut Cx) { }
```

### Composition

When creating composite components, depend on other component crates:

```toml
[dependencies]
makepad-shadecn-button = { path = "../../components/button", version = "0.1.0" }
makepad-shadecn-input = { path = "../../components/input", version = "0.1.0" }
```

Then compose them in Live DSL:

```rust
live_design! {
    use makepad_shadecn_button::*;
    use makepad_shadecn_input::*;

    pub MyCompositeComponent = <View> {
        button = <ShadecnButton> { }
        input = <ShadecnInput> { }
    }
}
```

## Testing Components

Add the component to the demo app to verify it works correctly with:
- Both light and dark themes
- Different screen sizes
- User interactions (hover, active, disabled states)

## Guidelines

1. **Consistency**: Follow the existing pattern for new components
2. **Composability**: Components should work well together
3. **Accessibility**: Consider keyboard navigation and screen reader support
4. **Documentation**: Add examples to the demo app
5. **Performance**: Keep Live DSL definitions lean and efficient
6. **Type Safety**: Use Rust's type system effectively in Rust code

## Common Patterns

### Hover/Active States

```rust
draw_bg: {
    fn pixel(self) -> vec4 {
        let normal = #ffffff;
        let hover = #f1f5f9;
        let active = #e2e8f0;
        
        return mix(
            mix(normal, hover, self.hover),
            active,
            self.pressed
        );
    }
}
```

### Disabled State

```rust
draw_text: {
    color: mix(#0f172a, #94a3b8, self.disabled)
}
```

### Focus/Focus Visible

```rust
draw_border: {
    border_color: mix(#e2e8f0, #3b82f6, self.focus),
    border_width: mix(1.0, 2.0, self.focus),
}
```

## Questions?

Refer to:
- [Makepad Documentation](https://github.com/makepad/makepad)
- [shadecn/ui](https://ui.shadecn.com/) for design inspiration
- Existing components in `components/` for examples

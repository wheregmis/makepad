# Makepad Shadecn UI Component Library

A comprehensive component library for Makepad inspired by [shadecn/ui](https://ui.shadecn.com/), featuring a design system with composable, customizable components built with Rust and Makepad's declarative UI framework.

## Philosophy

Following shadecn/ui principles:
- **Composable**: Build complex UIs from simple, reusable primitives
- **Customizable**: Each component is designed to be extended and modified
- **Design System**: Unified theming with design tokens for consistency
- **Type Safe**: Built with Rust's strong type system and Makepad's derive macros

## Project Structure

```
makepad-shadecn/
├── core/                    # Core design system, themes, and design tokens
├── components/
│   ├── button/              # Button component with variants
│   ├── input/               # Text input component
│   ├── card/                # Card container component
│   ├── checkbox/            # Checkbox component
│   └── [more components]    # Additional components
└── examples/
    └── shadecn-demo/        # Demo application showcasing all components
```

## Crates

Each component lives in its own crate for maximum flexibility and to enable users to depend only on what they need:

### Core
- **makepad-shadecn-core** (`core/`): Design tokens, theme definitions, and base utilities

### Components
- **makepad-shadecn-button** (`components/button/`): Button with variants (primary, secondary, outline, destructive, ghost)
- **makepad-shadecn-input** (`components/input/`): Text input field
- **makepad-shadecn-card** (`components/card/`): Card container with header, content, and footer
- **makepad-shadecn-checkbox** (`components/checkbox/`): Checkbox input

## Quick Start

### Using Components in Your Project

Add the components you need to your `Cargo.toml`:

```toml
[dependencies]
makepad-shadecn-core = { path = "../makepad-shadecn/core" }
makepad-shadecn-button = { path = "../makepad-shadecn/button" }
makepad-shadecn-input = { path = "../makepad-shadecn/input" }
makepad-shadecn-card = { path = "../makepad-shadecn/card" }
```

### Register Components

In your app's `live_register` function:

```rust
impl LiveRegister for MyApp {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_shadecn_core::live_design(cx);
        makepad_shadecn_button::live_design(cx);
        makepad_shadecn_input::live_design(cx);
        makepad_shadecn_card::live_design(cx);
        // Register other components as needed
    }
}
```

### Use in Live DSL

```rust
live_design! {
    use makepad_shadecn_button::*;
    use makepad_shadecn_input::*;
    use makepad_shadecn_card::*;

    my_view = <View> {
        flow: Down,
        spacing: 16,

        // Card containing form elements
        card = <ShadecnCard> {
            ShadecnCardHeader = <ShadecnCardHeader> {
                ShadecnCardTitle = <ShadecnCardTitle> {
                    text: "Login"
                }
            }

            ShadecnCardContent = <ShadecnCardContent> {
                flow: Down,
                spacing: 12,

                email = <ShadecnInput> {
                    empty_message: "Enter email..."
                }

                password = <ShadecnInput> {
                    empty_message: "Enter password..."
                }

                login_btn = <ShadecnButton> {
                    text: "Sign In"
                }
            }
        }
    }
}
```

## Theming

The library includes two built-in themes inspired by shadecn/ui:
- **shadecn_dark**: Dark theme based on Slate color palette
- **shadecn_light**: Light theme based on Slate color palette

### Theme System

Themes are defined in `core/src/theme.rs` and include design tokens:

- **Spacing**: `space_1` through `space_6` (4px to 24px)
- **Border Radius**: `radius_sm`, `radius_md`, `radius_lg`, `radius_xl`, `radius_full`
- **Colors**: Primary, secondary, tertiary backgrounds and foregrounds, borders, semantic colors (success, warning, error, info)
- **Font Sizes**: From `font_xs` (10px) to `font_2xl` (20px)

### Customizing Themes

To create a custom theme, extend the core theme in your app:

```rust
live_design! {
    use makepad_shadecn_core::theme_shadecn_light;
    
    pub custom_theme = <theme_shadecn_light> {
        accent: #ef4444,  // Red instead of blue
        success: #10b981, // Custom green
    }
}
```

## Design Principles

1. **Consistency**: All components follow the same design language and spacing scale
2. **Accessibility**: Components are built with keyboard navigation and screen reader support in mind
3. **Composability**: Simple primitives combine to build complex interfaces
4. **Customizability**: Full source access allows modification without forking
5. **Type Safety**: Rust's type system ensures correctness at compile time

## Running the Demo

```bash
cd makepad-shadecn/examples/shadecn-demo
cargo run
```

This launches an interactive demo showcasing all available components and theming capabilities.

## Adding New Components

To add a new component:

1. Create a new directory: `makepad-shadecn/components/new-component/`
2. Add `Cargo.toml` with dependencies on `makepad-shadecn-core` and `makepad-widgets`
3. Implement in `src/lib.rs`:
   - Define Live DSL template in `live_design!{}`
   - Create Rust struct with `#[derive(Live, LiveRegisterWidget, WidgetRef)]`
   - Implement `Widget` trait
   - Export `live_design()` function
4. Update `makepad-shadecn/Cargo.toml` workspace members
5. Update root `Cargo.toml` if necessary

## Design Token System

The `core/src/design_tokens.rs` module provides constants for:
- **Spacing**: Predefined scale (4px - 24px)
- **Border Radius**: Predefined values for consistency
- **Font Sizes**: Typography scale
- **Font Weights**: Standard weights
- **Colors**: Semantic and utility color palettes

These can be used in Rust code or referenced in Live DSL templates.

## Contributing

This is an evolving component library. To contribute:
1. Add new components following the existing patterns
2. Ensure components are composable and customizable
3. Document component usage with examples
4. Test across different themes

## License

MIT OR Apache-2.0 (same as Makepad)

## Inspiration

Built with inspiration from:
- [shadecn/ui](https://ui.shadecn.com/) - Copy-paste component philosophy and design system
- [Radix UI](https://www.radix-ui.com/) - Accessibility-first component design
- [Makepad](https://github.com/makepad/makepad) - Declarative UI framework

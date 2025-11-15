# Makepad Shadecn - Quick Reference

## Project Structure

```
makepad-shadecn/
├── Cargo.toml                 # Workspace configuration
├── README.md                  # Main documentation
├── CONTRIBUTING.md            # Guide for adding components
├── DESIGN_SYSTEM.md          # Design tokens and theming
│
├── core/                      # Core design system
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Main exports
│       ├── design_tokens.rs   # Design token constants
│       └── theme.rs           # Theme definitions (dark/light)
│
├── components/                # Component library
│   ├── button/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs        # ShadecnButton with variants
│   ├── input/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs        # ShadecnInput
│   ├── card/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs        # ShadecnCard + header/footer
│   └── checkbox/
│       ├── Cargo.toml
│       └── src/lib.rs        # ShadecnCheckbox
│
└── examples/
    └── shadecn-demo/         # Demo application
        ├── Cargo.toml
        └── src/app.rs        # Showcases all components
```

## Adding to Your Project

### 1. Update Your Cargo.toml

```toml
[dependencies]
makepad-widgets = { path = "../makepad/widgets" }
makepad-draw = { path = "../makepad/draw" }
makepad-shadecn-core = { path = "../makepad/makepad-shadecn/core" }
makepad-shadecn-button = { path = "../makepad/makepad-shadecn/components/button" }
makepad-shadecn-input = { path = "../makepad/makepad-shadecn/components/input" }
makepad-shadecn-card = { path = "../makepad/makepad-shadecn/components/card" }
makepad-shadecn-checkbox = { path = "../makepad/makepad-shadecn/components/checkbox" }
```

### 2. Register in Your App

```rust
impl LiveRegister for MyApp {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_shadecn_core::live_design(cx);
        makepad_shadecn_button::live_design(cx);
        makepad_shadecn_input::live_design(cx);
        makepad_shadecn_card::live_design(cx);
        makepad_shadecn_checkbox::live_design(cx);
    }
}
```

### 3. Use in Live DSL

```rust
live_design! {
    use makepad_shadecn_button::*;
    use makepad_shadecn_card::*;

    my_view = <View> {
        card = <ShadecnCard> {
            button = <ShadecnButton> {
                text: "Click me"
            }
        }
    }
}
```

## Component Quick Reference

### Button

```rust
// Variants: primary (default), secondary, outline, destructive, ghost
<ShadecnButton> { text: "Primary" }
<ShadecnButtonSecondary> { text: "Secondary" }
<ShadecnButtonOutline> { text: "Outline" }
<ShadecnButtonDestructive> { text: "Delete" }
<ShadecnButtonGhost> { text: "Ghost" }
```

### Input

```rust
<ShadecnInput> {
    empty_message: "Enter text..."
}
```

### Card

```rust
<ShadecnCard> {
    ShadecnCardHeader = <ShadecnCardHeader> {
        ShadecnCardTitle = <ShadecnCardTitle> { text: "Title" }
        ShadecnCardDescription = <ShadecnCardDescription> { 
            text: "Description" 
        }
    }

    ShadecnCardContent = <ShadecnCardContent> {
        // Content here
    }

    ShadecnCardFooter = <ShadecnCardFooter> {
        // Footer content
    }
}
```

### Checkbox

```rust
<ShadecnCheckbox> {
    text: "I agree"
}
```

## Design Tokens Quick Reference

### Spacing
- `space_1`: 4px
- `space_2`: 8px
- `space_3`: 12px
- `space_4`: 16px
- `space_5`: 20px
- `space_6`: 24px

### Font Sizes
- `font_xs`: 10px
- `font_sm`: 12px
- `font_base`: 14px
- `font_lg`: 16px
- `font_xl`: 18px
- `font_2xl`: 20px

### Border Radius
- `radius_none`: 0px
- `radius_sm`: 2px
- `radius_md`: 4px
- `radius_lg`: 8px
- `radius_xl`: 12px
- `radius_full`: 9999px

### Colors (Dark Theme)
- `bg_primary`: #0f172a
- `fg_primary`: #f8fafc
- `border_primary`: #475569
- `accent`: #3b82f6
- `success`: #22c55e
- `error`: #ef4444

### Colors (Light Theme)
- `bg_primary`: #ffffff
- `fg_primary`: #0f172a
- `border_primary`: #e2e8f0
- `accent`: #3b82f6
- `success`: #22c55e
- `error`: #ef4444

## Common Patterns

### Full-width Button
```rust
<ShadecnButton> {
    width: Fill,
    text: "Full Width"
}
```

### Form Layout
```rust
<View> {
    flow: Down,
    spacing: (space_4),

    label = <Label> { text: "Email" }
    input = <ShadecnInput> { }
    
    button = <ShadecnButton> { text: "Submit" }
}
```

### Themed Variant
```rust
pub MyButton = <ShadecnButtonPrimary> {
    draw_bg: {
        fn pixel(self) -> vec4 {
            return mix(
                mix(#6366f1, #4f46e5, self.hover),
                #3730a3,
                self.pressed
            );
        }
    }
}
```

## Files to Know

| File | Purpose |
|------|---------|
| `core/src/theme.rs` | Theme definitions |
| `core/src/design_tokens.rs` | Token constants |
| `components/*/src/lib.rs` | Component implementation |
| `examples/shadecn-demo/src/app.rs` | Full working example |
| `DESIGN_SYSTEM.md` | Comprehensive design guide |
| `CONTRIBUTING.md` | How to add new components |

## Running the Demo

```bash
cd makepad-shadecn/examples/shadecn-demo
cargo run
```

## Helpful Commands

```bash
# Build all components
cd makepad-shadecn
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy

# Build documentation
cargo doc --open
```

## Contributing a New Component

1. Create `makepad-shadecn/components/newcomponent/`
2. Create `Cargo.toml` with dependencies
3. Implement in `src/lib.rs` (see CONTRIBUTING.md)
4. Update workspace `Cargo.toml`
5. Add to demo app
6. Test with both themes

## Resources

- [Makepad GitHub](https://github.com/makepad/makepad)
- [shadecn/ui](https://ui.shadecn.com/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Radix UI](https://www.radix-ui.com/)

## FAQ

**Q: Can I use this with any Makepad app?**  
A: Yes! Any Makepad app can import these components as dependencies.

**Q: How do I customize a component?**  
A: Extend it in your app's Live DSL or create a wrapper component.

**Q: Can I create custom themes?**  
A: Yes, define a new theme in your app or in the core crate.

**Q: Are these production-ready?**  
A: The library provides a solid foundation. Test components in your use case.

**Q: How do I add accessibility features?**  
A: Components inherit accessibility from Makepad widgets. Enhance as needed in your extensions.

**Q: What about dark mode switching?**  
A: Define both `theme_shadecn_dark` and `theme_shadecn_light`, then switch via Live DSL at runtime.

## Support

For issues or questions:
1. Check DESIGN_SYSTEM.md and CONTRIBUTING.md
2. Review example code in shadecn-demo
3. Refer to Makepad documentation
4. Open an issue on GitHub

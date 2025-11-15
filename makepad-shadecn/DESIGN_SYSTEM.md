# Shadecn Design System

The makepad-shadecn library is built on a comprehensive design system inspired by shadecn/ui and Tailwind CSS. This document describes the design tokens and system-wide principles.

## Core Design Principles

### 1. Consistency
All components use the same design tokens, ensuring a cohesive visual language across the entire application.

### 2. Customizability  
Design tokens are themeable and overrideable, allowing applications to customize appearance while maintaining system consistency.

### 3. Accessibility
Components are built with accessibility as a foundation, supporting keyboard navigation, color contrast, and screen reader compatibility.

### 4. Composability
Simple primitives combine to create complex interfaces without breaking the design system.

## Color Palette

### Dark Theme (`theme_shadecn_dark`)

**Backgrounds:**
- `bg_primary`: `#0f172a` - Main background (Slate 900)
- `bg_secondary`: `#1e293b` - Secondary background (Slate 800)
- `bg_tertiary`: `#334155` - Tertiary background (Slate 700)

**Foregrounds:**
- `fg_primary`: `#f8fafc` - Main text (Slate 50)
- `fg_secondary`: `#e2e8f0` - Secondary text (Slate 200)
- `fg_tertiary`: `#cbd5e1` - Tertiary text (Slate 300)

**Borders:**
- `border_primary`: `#475569` - Main borders (Slate 600)
- `border_secondary`: `#334155` - Secondary borders (Slate 700)

**Component States:**
- `bg_hover`: `#1e293b` - Hover state
- `bg_active`: `#334155` - Active/pressed state
- `bg_disabled`: `#1e293b` - Disabled state
- `fg_disabled`: `#64748b` - Disabled text (Slate 500)

**Semantic:**
- `success`: `#22c55e` - Success/positive actions (Green)
- `warning`: `#f59e0b` - Warning states (Amber)
- `error`: `#ef4444` - Error/destructive actions (Red)
- `info`: `#3b82f6` - Informational (Blue)

**Primary Accent:**
- `accent`: `#3b82f6` - Interactive elements (Blue)

### Light Theme (`theme_shadecn_light`)

**Backgrounds:**
- `bg_primary`: `#ffffff` - Main background (White)
- `bg_secondary`: `#f8fafc` - Secondary background (Slate 50)
- `bg_tertiary`: `#f1f5f9` - Tertiary background (Slate 100)

**Foregrounds:**
- `fg_primary`: `#0f172a` - Main text (Slate 900)
- `fg_secondary`: `#1e293b` - Secondary text (Slate 800)
- `fg_tertiary`: `#334155` - Tertiary text (Slate 700)

**Borders:**
- `border_primary`: `#e2e8f0` - Main borders (Slate 200)
- `border_secondary`: `#cbd5e1` - Secondary borders (Slate 300)

**Component States:**
- `bg_hover`: `#f1f5f9` - Hover state (Slate 100)
- `bg_active`: `#e2e8f0` - Active/pressed state (Slate 200)
- `bg_disabled`: `#f8fafc` - Disabled state (Slate 50)
- `fg_disabled`: `#94a3b8` - Disabled text (Slate 400)

**Semantic:**
- `success`: `#22c55e` - Success/positive actions
- `warning`: `#f59e0b` - Warning states
- `error`: `#ef4444` - Error/destructive actions
- `info`: `#3b82f6` - Informational

**Primary Accent:**
- `accent`: `#3b82f6` - Interactive elements

## Spacing Scale

The spacing scale provides consistent vertical and horizontal spacing throughout the system:

| Token | Value | Usage |
|-------|-------|-------|
| `space_1` | 4px | Tight spacing, icon padding |
| `space_2` | 8px | Button padding, small gaps |
| `space_3` | 12px | Component spacing |
| `space_4` | 16px | Section spacing |
| `space_5` | 20px | Large spacing |
| `space_6` | 24px | Very large spacing |

### Spacing Examples

- **Button padding**: `space_2` (8px) horizontal, `space_2` (6px) vertical = 8x6
- **Card padding**: `space_6` (24px) on all sides
- **Component gaps**: `space_4` (16px) between major sections
- **Element spacing**: `space_2` (8px) between inline elements

## Typography Scale

### Font Sizes

| Token | Size | Usage |
|-------|------|-------|
| `font_xs` | 10px | Captions, small labels |
| `font_sm` | 12px | Secondary text, hints |
| `font_base` | 14px | Body text, default |
| `font_lg` | 16px | Large text |
| `font_xl` | 18px | Heading level 3 |
| `font_2xl` | 20px | Heading level 2 |

### Font Weights

| Token | Weight | Usage |
|-------|--------|-------|
| `font_normal` | 400 | Body text |
| `font_medium` | 500 | Emphasis |
| `font_semibold` | 600 | Component labels |
| `font_bold` | 700 | Headings |

### Typography Hierarchy

```
Heading 1: 28px, Bold (font_2xl + weight 700)
Heading 2: 20px, Semibold (font_2xl + weight 600)
Heading 3: 18px, Semibold (font_xl + weight 600)
Body: 14px, Normal (font_base + weight 400)
Small: 12px, Normal (font_sm + weight 400)
Caption: 10px, Normal (font_xs + weight 400)
```

## Border Radius

Consistent border radius values for rounded corners:

| Token | Value | Usage |
|-------|-------|-------|
| `radius_none` | 0px | Sharp corners |
| `radius_sm` | 2px | Small, refined corners |
| `radius_md` | 4px | Standard corners |
| `radius_lg` | 8px | Large, prominent corners |
| `radius_xl` | 12px | Extra large corners |
| `radius_full` | 9999px | Fully rounded (pills) |

### Border Radius Guidelines

- **Buttons, Inputs**: `radius_md` (4px)
- **Cards, Containers**: `radius_lg` (8px)
- **Pills, Badges**: `radius_full` (9999px)
- **Slight refinement**: `radius_sm` (2px)

## Component Design Tokens

### Button

```
Padding: space_2 horizontal (8px), space_2 vertical (6px)
Border Radius: radius_md (4px)
Font Size: font_base (14px)
Font Weight: semibold (600)
```

**Variants:**
- **Primary**: bg=accent, text=white
- **Secondary**: bg=slate-200, text=slate-900
- **Outline**: bg=white, border=slate-200, text=slate-900
- **Destructive**: bg=error, text=white
- **Ghost**: bg=transparent, text=slate-900, hover=slate-200

### Input

```
Padding: space_2 horizontal (8px), space_3 vertical (12px)
Border Radius: radius_md (4px)
Font Size: font_base (14px)
Border: 1px solid border_primary
```

**States:**
- Default: bg=white, border=slate-200, text=slate-900
- Hover: bg slightly lighter
- Focus: border=accent (blue), focus ring visible
- Disabled: bg=slate-50, text=disabled, cursor=not-allowed

### Card

```
Padding: space_6 (24px) on all sides
Border Radius: radius_lg (8px)
Border: 1px solid border_primary
Background: bg_primary
```

**Sub-components:**
- **Header**: padding=0, margin-bottom=space_4
- **Title**: font_xl (18px), semibold
- **Description**: font_base (14px), secondary color
- **Content**: default spacing
- **Footer**: margin-top=space_4, layout=row with gap=space_2

### Checkbox

```
Size: 16x16px typical
Border Radius: radius_sm (2px)
Border: 1.5px solid border_primary
Padding: space_1 (4px) when selected
```

**States:**
- **Unchecked**: bg=white, border=slate-200
- **Checked**: bg=accent (blue), border=accent
- **Hover**: border slightly darker
- **Disabled**: bg=slate-50, border=slate-200

## Motion & Animation

Currently, no animation tokens are defined, but components support smooth transitions:
- Hover states: Immediate visual feedback
- State changes: No animation (instant)

In future versions, animation tokens may be added for:
- Transition duration (fast, normal, slow)
- Easing functions (ease-in, ease-out, ease-in-out)

## Accessibility

### Color Contrast

All color combinations meet WCAG AA standards (4.5:1 ratio) for text:
- Dark theme: Light text on dark backgrounds
- Light theme: Dark text on light backgrounds

### Interactive Elements

All interactive elements support:
- **Keyboard Navigation**: Tab/Shift+Tab to navigate
- **Focus Indicators**: Visible focus ring (blue accent color)
- **Disabled State**: Visual indication of unavailable elements
- **Screen Readers**: Proper semantic HTML/structure

## Extending the Design System

### Adding New Design Tokens

Edit `core/src/theme.rs` to add new tokens:

```rust
pub theme_shadecn_dark = {
    // Existing tokens...
    custom_color: #abc123,
    custom_spacing: 32.0,
}
```

### Creating Custom Themes

Define a new theme variant:

```rust
pub theme_shadecn_custom = {
    // Start from dark theme
    name: "Custom Dark",
    bg_primary: #1a1a2e,
    accent: #16213e,
    // Override other tokens as needed
}
```

### Component-Level Customization

Override tokens for specific components:

```rust
live_design! {
    pub ShadecnButtonCustom = <ShadecnButton> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                return #custom_color;
            }
        }
    }
}
```

## Migration Guide

### From shadecn/ui (React/Tailwind)

| Tailwind/shadecn | Makepad Shadecn |
|-------------------|-----------------|
| `p-4` (16px) | `space_4` |
| `text-sm` (12px) | `font_sm` |
| `rounded-md` (4px) | `radius_md` |
| `bg-slate-900` | `bg_primary` (dark) |
| `text-slate-50` | `fg_primary` (dark) |
| `focus:ring-2` | `self.focus` in shader |
| `hover:bg-opacity-80` | `self.hover` in shader |

## References

- [Tailwind CSS Design System](https://tailwindcss.com/docs/customization)
- [shadecn/ui Color System](https://ui.shadecn.com/)
- [Radix UI Design Tokens](https://www.radix-ui.com/docs/primitives/overview/styling)
- [WCAG Color Contrast](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)

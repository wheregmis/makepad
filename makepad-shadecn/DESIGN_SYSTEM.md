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

The color palette is based on OKLCH color space for improved consistency and perceptual uniformity. All colors are derived from shadecn/ui's default color system.

### Light Theme (`theme_shadecn_light`)

**Backgrounds:**
- `bg_primary`: `#ffffff` - `oklch(1 0 0)` - Main background (White)
- `bg_secondary`: `#ffffff` - `oklch(1 0 0)` - Card background (White)
- `bg_tertiary`: `#f7f7f7` - `oklch(0.97 0 0)` - Secondary/Muted background (Very light gray)

**Foregrounds:**
- `fg_primary`: `#252525` - `oklch(0.145 0 0)` - Main text (Very dark gray)
- `fg_secondary`: `#252525` - `oklch(0.145 0 0)` - Card text (Very dark gray)
- `fg_tertiary`: `#8e8e8e` - `oklch(0.556 0 0)` - Muted text (Medium gray)

**Borders:**
- `border_primary`: `#ebebeb` - `oklch(0.922 0 0)` - Main borders (Light gray)
- `border_secondary`: `#ebebeb` - `oklch(0.922 0 0)` - Input borders (Light gray)

**Primary Colors:**
- `primary`: `#343434` - `oklch(0.205 0 0)` - Primary actions (Dark gray/Black)
- `primary_foreground`: `#fbfbfb` - `oklch(0.985 0 0)` - Primary text (Almost white)

**Accent Colors:**
- `accent`: `#f7f7f7` - `oklch(0.97 0 0)` - Accent background (Very light gray)
- `accent_foreground`: `#343434` - `oklch(0.205 0 0)` - Accent text (Dark gray)

**Component States:**
- `bg_hover`: `#f7f7f7` - `oklch(0.97 0 0)` - Hover state (Very light gray)
- `bg_active`: `#f7f7f7` - `oklch(0.97 0 0)` - Active/pressed state (Very light gray)
- `bg_disabled`: `#f7f7f7` - `oklch(0.97 0 0)` - Disabled background (Very light gray)
- `fg_disabled`: `#8e8e8e` - `oklch(0.556 0 0)` - Disabled text (Medium gray)

**Semantic Colors:**
- `success`: `#22c55e` - Success/positive actions (Green)
- `warning`: `#f59e0b` - Warning states (Amber)
- `error`: `#e11d48` - `oklch(0.577 0.245 27.325)` - Destructive actions (Red/Orange)
- `info`: `#b5b5b5` - `oklch(0.708 0 0)` - Focus rings (Medium gray)

### Dark Theme (`theme_shadecn_dark`)

**Backgrounds:**
- `bg_primary`: `#252525` - `oklch(0.145 0 0)` - Main background (Very dark gray)
- `bg_secondary`: `#343434` - `oklch(0.205 0 0)` - Card background (Dark gray)
- `bg_tertiary`: `#444444` - `oklch(0.269 0 0)` - Popover/Secondary background (Dark gray)

**Foregrounds:**
- `fg_primary`: `#fbfbfb` - `oklch(0.985 0 0)` - Main text (Almost white)
- `fg_secondary`: `#fbfbfb` - `oklch(0.985 0 0)` - Card text (Almost white)
- `fg_tertiary`: `#b5b5b5` - `oklch(0.708 0 0)` - Muted text (Medium gray)

**Borders:**
- `border_primary`: `#ffffff1a` - `oklch(1 0 0 / 10%)` - Main borders (White with 10% opacity)
- `border_secondary`: `#ffffff1a` - `oklch(1 0 0 / 10%)` - Sidebar borders (White with 10% opacity)

**Primary Colors:**
- `primary`: `#ebebeb` - `oklch(0.922 0 0)` - Primary actions (Light gray)
- `primary_foreground`: `#343434` - `oklch(0.205 0 0)` - Primary text (Dark gray)

**Accent Colors:**
- `accent`: `#5e5e5e` - `oklch(0.371 0 0)` - Accent background (Medium-dark gray)
- `accent_foreground`: `#fbfbfb` - `oklch(0.985 0 0)` - Accent text (Almost white)

**Component States:**
- `bg_hover`: `#5e5e5e` - `oklch(0.371 0 0)` - Hover state (Medium-dark gray)
- `bg_active`: `#444444` - `oklch(0.269 0 0)` - Active/pressed state (Dark gray)
- `bg_disabled`: `#444444` - `oklch(0.269 0 0)` - Disabled background (Dark gray)
- `fg_disabled`: `#b5b5b5` - `oklch(0.708 0 0)` - Disabled text (Medium gray)

**Semantic Colors:**
- `success`: `#22c55e` - Success/positive actions (Green)
- `warning`: `#f59e0b` - Warning states (Amber)
- `error`: `#dc2626` - `oklch(0.704 0.191 22.216)` - Destructive actions (Dark red)
- `info`: `#8e8e8e` - `oklch(0.556 0 0)` - Focus rings (Medium gray)

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

Consistent border radius values for rounded corners. Default shadecn radius is `0.625rem` (10px).

| Token | Value | Usage |
|-------|-------|-------|
| `radius_none` | 0px | Sharp corners |
| `radius_sm` | 2px | Small, refined corners |
| `radius_md` | 4px | Standard corners |
| `radius_lg` | 8px | Large, prominent corners |
| `radius_xl` | 10px | Default shadecn radius (0.625rem) |
| `radius_full` | 9999px | Fully rounded (pills) |

### Border Radius Guidelines

- **Buttons, Inputs**: `radius_md` (4px)
- **Cards, Containers**: `radius_lg` (8px)
- **Default shadecn**: `radius_xl` (10px) - `--radius: 0.625rem`
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

### Dropdown Menu

```
Padding: space_2 horizontal (8px), space_2 vertical (8px)
Border Radius: radius_md (4px)
Font Size: font_base (14px)
Border: 1px solid border_primary
Arrow Icon: Right-aligned, 28px from right edge
```

**States:**
- **Default**: bg=bg_primary, border=border_primary, text=fg_primary
- **Hover**: bg=bg_hover, border=border_hover
- **Focus**: bg=bg_primary, border=primary (accent color)
- **Disabled**: bg=bg_disabled, text=fg_disabled, border=border_disabled

**Popup Menu:**
- **Background**: bg_primary with border_primary border
- **Border Radius**: radius_md (4px)
- **Menu Item Padding**: space_3 horizontal (12px), space_2 vertical (8px)
- **Menu Item States**:
  - Default: bg=bg_primary, text=fg_primary
  - Hover: bg=bg_hover, text=fg_primary
  - Active: bg=bg_active, text=fg_primary

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

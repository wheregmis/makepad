use crate::makepad_platform::*;

live_design! {
    link shaders;

    // ============================================
    // Design Token Constants
    // ============================================
    // These constants can be referenced across all components

    // Border Radius
    // --radius: 0.625rem = 10px (default shadecn radius)
    pub RADIUS_NONE = 0.0
    pub RADIUS_SM = 2.0
    pub RADIUS_MD = 4.0
    pub RADIUS_LG = 8.0
    pub RADIUS_XL = 10.0  // Default shadecn radius
    pub RADIUS_FULL = 9999.0

    // Spacing Scale
    pub SPACE_1 = 4.0
    pub SPACE_2 = 8.0
    pub SPACE_3 = 12.0
    pub SPACE_4 = 16.0
    pub SPACE_5 = 20.0
    pub SPACE_6 = 24.0

    // Font Sizes
    pub FONT_XS = 10.0
    pub FONT_SM = 12.0
    pub FONT_BASE = 14.0
    pub FONT_LG = 16.0
    pub FONT_XL = 18.0
    pub FONT_2XL = 20.0

    // Light Theme Colors (OKLCH-based from shadecn/ui)
    // --background: oklch(1 0 0) = white
    pub COLOR_BG_PRIMARY = #ffffff
    // --secondary: oklch(0.97 0 0) = very light gray
    pub COLOR_BG_SECONDARY = #f7f7f7
    // --muted: oklch(0.97 0 0) = very light gray
    pub COLOR_BG_TERTIARY = #f7f7f7
    // --accent: oklch(0.97 0 0) = very light gray (for hover states)
    pub COLOR_BG_HOVER = #f7f7f7
    // --secondary: oklch(0.97 0 0) = very light gray
    pub COLOR_BG_ACTIVE = #f7f7f7
    // --muted: oklch(0.97 0 0) = very light gray
    pub COLOR_BG_DISABLED = #f7f7f7

    // --popover: white
    pub COLOR_BG_POPOVER = #ffffff
    
    // --foreground: oklch(0.145 0 0) = very dark gray
    pub COLOR_FG_PRIMARY = #252525
    // --secondary-foreground: oklch(0.205 0 0) = dark gray
    pub COLOR_FG_SECONDARY = #343434
    // --muted-foreground: oklch(0.556 0 0) = medium gray
    pub COLOR_FG_TERTIARY = #8e8e8e
    // --muted-foreground: oklch(0.556 0 0) = medium gray
    pub COLOR_FG_DISABLED = #8e8e8e

    // --border: oklch(0.922 0 0) = light gray
    pub COLOR_BORDER_PRIMARY = #ebebeb
    // --border: oklch(0.922 0 0) = light gray
    pub COLOR_BORDER_SECONDARY = #ebebeb
    // --border: oklch(0.922 0 0) = light gray
    pub COLOR_BORDER_HOVER = #ebebeb
    // --border: oklch(0.922 0 0) with opacity
    pub COLOR_BORDER_DISABLED = #ebebeb80

    // --primary: oklch(0.205 0 0) = dark gray/black
    pub COLOR_PRIMARY = #343434
    // --primary-foreground: oklch(0.985 0 0) = almost white
    pub COLOR_PRIMARY_FOREGROUND = #fbfbfb

    // --accent: oklch(0.97 0 0) = very light gray
    pub COLOR_ACCENT = #f7f7f7
    // --accent-foreground: oklch(0.205 0 0) = dark gray
    pub COLOR_ACCENT_FOREGROUND = #343434
    // Accent hover - slightly darker
    pub COLOR_ACCENT_HOVER = #f0f0f0
    // Accent dark - for pressed states
    pub COLOR_ACCENT_DARK = #e8e8e8
    // Accent darker - for focus
    pub COLOR_ACCENT_DARKER = #d9d9d9
    // --ring: oklch(0.708 0 0) = medium gray
    pub COLOR_ACCENT_FOCUS = #b5b5b5
    // Accent light - lighter variation
    pub COLOR_ACCENT_LIGHT = #fbfbfb
    // Accent lighter - even lighter
    pub COLOR_ACCENT_LIGHTER = #fdfdfd

    // Semantic colors
    pub COLOR_SUCCESS = #22c55e
    pub COLOR_WARNING = #f59e0b
    // --destructive: oklch(0.577 0.245 27.325) = red/orange
    pub COLOR_ERROR = #e11d48
    // Dark destructive: oklch(0.704 0.191 22.216) = dark red
    pub COLOR_ERROR_HOVER = #dc2626
    pub COLOR_ERROR_DARK = #b91c1c
    pub COLOR_ERROR_DARKER = #991b1b
    // --ring: oklch(0.708 0 0) = medium gray (used for focus rings)
    pub COLOR_INFO = #b5b5b5

    pub COLOR_WHITE = #ffffff
    pub COLOR_WHITE_TRANSPARENT_00 = #ffffff00
    pub COLOR_WHITE_TRANSPARENT_55 = #ffffff55
    pub COLOR_WHITE_TRANSPARENT_AA = #ffffffaa

    pub COLOR_DARK_TRANSPARENT_00 = #0f172a00
    pub COLOR_DARK_TRANSPARENT_1A = #0f172a1a
    pub COLOR_DARK_TRANSPARENT_33 = #0f172a33
    pub COLOR_DARK_TRANSPARENT_47 = #0f172a47

    pub COLOR_SLATE_50 = #f8fafc
    pub COLOR_SLATE_100 = #f1f5f9
    pub COLOR_SLATE_200 = #e2e8f0
    pub COLOR_SLATE_300 = #cbd5e1
    pub COLOR_SLATE_400 = #94a3b8
    pub COLOR_SLATE_500 = #64748b
    pub COLOR_SLATE_700 = #334155
    pub COLOR_SLATE_800 = #1e293b
    pub COLOR_SLATE_900 = #0f172a

    // ============================================
    // Dark Theme - Shadecn
    // ============================================
    pub theme_shadecn_dark = {
        // Design token scalars
        name: "ShadeCN Dark",

        // Spacing scale
        space_1: 4.0,
        space_2: 8.0,
        space_3: 12.0,
        space_4: 16.0,
        space_5: 20.0,
        space_6: 24.0,

        // Border radius
        radius_none: 0.0,
        radius_sm: 2.0,
        radius_md: 4.0,
        radius_lg: 8.0,
        radius_xl: 12.0,
        radius_full: 9999.0,

        // Font sizes
        font_xs: 10.0,
        font_sm: 12.0,
        font_base: 14.0,
        font_lg: 16.0,
        font_xl: 18.0,
        font_2xl: 20.0,

        // Colors - Dark theme palette (OKLCH-based from shadecn/ui)
        // --background: oklch(0.145 0 0) = very dark gray
        bg_primary: #252525,
        // --card: oklch(0.205 0 0) = dark gray
        bg_secondary: #343434,
        // --popover: oklch(0.269 0 0) = dark gray
        bg_tertiary: #444444,

        // --foreground: oklch(0.985 0 0) = almost white
        fg_primary: #fbfbfb,
        // --card-foreground: oklch(0.985 0 0) = almost white
        fg_secondary: #fbfbfb,
        // --muted-foreground: oklch(0.708 0 0) = medium gray
        fg_tertiary: #b5b5b5,

        // --border: oklch(1 0 0 / 10%) = white with 10% opacity
        border_primary: #ffffff1a,
        // --sidebar-border: oklch(1 0 0 / 10%) = white with 10% opacity
        border_secondary: #ffffff1a,

        // Component-specific colors
        // --accent: oklch(0.371 0 0) = medium-dark gray
        bg_hover: #5e5e5e,
        // --secondary: oklch(0.269 0 0) = dark gray
        bg_active: #444444,
        // --muted: oklch(0.269 0 0) = dark gray
        bg_disabled: #444444,
        // --muted-foreground: oklch(0.708 0 0) = medium gray
        fg_disabled: #b5b5b5,

        // --primary: oklch(0.922 0 0) = light gray
        primary: #ebebeb,
        // --primary-foreground: oklch(0.205 0 0) = dark gray
        primary_foreground: #343434,

        // Semantic colors
        success: #22c55e,
        warning: #f59e0b,
        // --destructive: oklch(0.704 0.191 22.216) = dark red
        error: #dc2626,
        // --ring: oklch(0.556 0 0) = medium gray
        info: #8e8e8e,

        // --accent: oklch(0.371 0 0) = medium-dark gray
        accent: #5e5e5e,
        // --accent-foreground: oklch(0.985 0 0) = almost white
        accent_foreground: #fbfbfb
    }

    // ============================================
    // Light Theme - Shadecn
    // ============================================
    pub theme_shadecn_light = {
        // Design token scalars
        name: "ShadeCN Light",

        // Spacing scale
        space_1: 4.0,
        space_2: 8.0,
        space_3: 12.0,
        space_4: 16.0,
        space_5: 20.0,
        space_6: 24.0,

        // Border radius
        radius_none: 0.0,
        radius_sm: 2.0,
        radius_md: 4.0,
        radius_lg: 8.0,
        radius_xl: 12.0,
        radius_full: 9999.0,

        // Font sizes
        font_xs: 10.0,
        font_sm: 12.0,
        font_base: 14.0,
        font_lg: 16.0,
        font_xl: 18.0,
        font_2xl: 20.0,

        // Colors - Light theme palette (OKLCH-based from shadecn/ui)
        // --background: oklch(1 0 0) = white
        bg_primary: #ffffff,
        // --card: oklch(1 0 0) = white
        bg_secondary: #ffffff,
        // --secondary: oklch(0.97 0 0) = very light gray
        bg_tertiary: #f7f7f7,

        // --foreground: oklch(0.145 0 0) = very dark gray
        fg_primary: #252525,
        // --card-foreground: oklch(0.145 0 0) = very dark gray
        fg_secondary: #252525,
        // --muted-foreground: oklch(0.556 0 0) = medium gray
        fg_tertiary: #8e8e8e,

        // --border: oklch(0.922 0 0) = light gray
        border_primary: #ebebeb,
        // --input: oklch(0.922 0 0) = light gray
        border_secondary: #ebebeb,

        // Component-specific colors
        // --accent: oklch(0.97 0 0) = very light gray (hover)
        bg_hover: #f7f7f7,
        // --secondary: oklch(0.97 0 0) = very light gray (active)
        bg_active: #f7f7f7,
        // --muted: oklch(0.97 0 0) = very light gray (disabled)
        bg_disabled: #f7f7f7,
        // --muted-foreground: oklch(0.556 0 0) = medium gray
        fg_disabled: #8e8e8e,

        // --primary: oklch(0.205 0 0) = dark gray/black
        primary: #343434,
        // --primary-foreground: oklch(0.985 0 0) = almost white
        primary_foreground: #fbfbfb,

        // Semantic colors
        success: #22c55e,
        warning: #f59e0b,
        // --destructive: oklch(0.577 0.245 27.325) = red/orange
        error: #e11d48,
        // --ring: oklch(0.708 0 0) = medium gray (focus rings)
        info: #b5b5b5,

        // --accent: oklch(0.97 0 0) = very light gray
        accent: #f7f7f7,
        // --accent-foreground: oklch(0.205 0 0) = dark gray
        accent_foreground: #343434
    }
}

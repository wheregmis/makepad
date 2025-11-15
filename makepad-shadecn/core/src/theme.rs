use crate::makepad_platform::*;

live_design! {
    link shaders;
    
    // ============================================
    // Design Token Constants
    // ============================================
    // These constants can be referenced across all components
    
    // Border Radius
    pub RADIUS_NONE = 0.0
    pub RADIUS_SM = 2.0
    pub RADIUS_MD = 4.0
    pub RADIUS_LG = 8.0
    pub RADIUS_XL = 12.0
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
    
    // Light Theme Colors
    pub COLOR_BG_PRIMARY = #ffffff
    pub COLOR_BG_SECONDARY = #f8fafc
    pub COLOR_BG_TERTIARY = #f1f5f9
    pub COLOR_BG_HOVER = #f1f5f9
    pub COLOR_BG_ACTIVE = #e2e8f0
    pub COLOR_BG_DISABLED = #f1f5f9
    
    pub COLOR_FG_PRIMARY = #0f172a
    pub COLOR_FG_SECONDARY = #1e293b
    pub COLOR_FG_TERTIARY = #334155
    pub COLOR_FG_DISABLED = #94a3b8
    
    pub COLOR_BORDER_PRIMARY = #e2e8f0
    pub COLOR_BORDER_SECONDARY = #cbd5e1
    pub COLOR_BORDER_HOVER = #cbd5e1
    pub COLOR_BORDER_DISABLED = #e2e8f080
    
    pub COLOR_ACCENT = #3b82f6
    pub COLOR_ACCENT_HOVER = vec4(0.145, 0.388, 0.922, 1.0)
    pub COLOR_ACCENT_DARK = #1d4ed8
    pub COLOR_ACCENT_DARKER = #1e3a8a
    pub COLOR_ACCENT_FOCUS = #93c5fd
    pub COLOR_ACCENT_LIGHT = #dbeafe
    pub COLOR_ACCENT_LIGHTER = #bfdbfe
    
    pub COLOR_SUCCESS = #22c55e
    pub COLOR_WARNING = #f59e0b
    pub COLOR_ERROR = #ef4444
    pub COLOR_ERROR_HOVER = #dc2626
    pub COLOR_ERROR_DARK = #b91c1c
    pub COLOR_ERROR_DARKER = #991b1b
    pub COLOR_INFO = #3b82f6
    
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

        // Colors - Dark theme palette (Slate-based)
        bg_primary: #0f172a,          // slate-900
        bg_secondary: #1e293b,        // slate-800
        bg_tertiary: #334155,         // slate-700
        
        fg_primary: #f8fafc,          // slate-50
        fg_secondary: #e2e8f0,        // slate-200
        fg_tertiary: #cbd5e1,         // slate-300
        
        border_primary: #475569,      // slate-600
        border_secondary: #334155,    // slate-700
        
        // Component-specific colors
        bg_hover: #1e293b,             // slate-800
        bg_active: #334155,            // slate-700
        bg_disabled: #1e293b,          // slate-800
        fg_disabled: #64748b,          // slate-500
        
        // Semantic colors
        success: #22c55e,
        warning: #f59e0b,
        error: #ef4444,
        info: #3b82f6,
        
        // Accent color
        accent: #3b82f6,               // Blue
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

        // Colors - Light theme palette (Slate-based)
        bg_primary: #ffffff,           // white
        bg_secondary: #f8fafc,         // slate-50
        bg_tertiary: #f1f5f9,          // slate-100
        
        fg_primary: #0f172a,           // slate-900
        fg_secondary: #1e293b,         // slate-800
        fg_tertiary: #334155,          // slate-700
        
        border_primary: #e2e8f0,       // slate-200
        border_secondary: #cbd5e1,     // slate-300
        
        // Component-specific colors
        bg_hover: #f1f5f9,              // slate-100
        bg_active: #e2e8f0,             // slate-200
        bg_disabled: #f8fafc,           // slate-50
        fg_disabled: #94a3b8,           // slate-400
        
        // Semantic colors
        success: #22c55e,
        warning: #f59e0b,
        error: #ef4444,
        info: #3b82f6,
        
        // Accent color
        accent: #3b82f6,                // Blue
    }
}



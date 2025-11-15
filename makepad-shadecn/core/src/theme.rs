use crate::makepad_platform::*;

live_design! {
    link shaders;
    
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



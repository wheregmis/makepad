/// Design tokens for shadecn/Makepad
/// 
/// This module defines the core design tokens used throughout the component library.
/// These tokens are used in theme definitions and Live DSL templates.

pub mod spacing {
    pub const SPACE_1: f64 = 4.0;    // 4px
    pub const SPACE_2: f64 = 8.0;    // 8px
    pub const SPACE_3: f64 = 12.0;   // 12px
    pub const SPACE_4: f64 = 16.0;   // 16px
    pub const SPACE_5: f64 = 20.0;   // 20px
    pub const SPACE_6: f64 = 24.0;   // 24px
}

pub mod border_radius {
    pub const NONE: f64 = 0.0;
    pub const SM: f64 = 2.0;
    pub const MD: f64 = 4.0;
    pub const LG: f64 = 8.0;
    pub const XL: f64 = 12.0;
    pub const FULL: f64 = 9999.0;
}

pub mod font_sizes {
    pub const XS: f64 = 10.0;
    pub const SM: f64 = 12.0;
    pub const BASE: f64 = 14.0;
    pub const LG: f64 = 16.0;
    pub const XL: f64 = 18.0;
    pub const XXL: f64 = 20.0;
}

pub mod font_weights {
    pub const NORMAL: f64 = 400.0;
    pub const MEDIUM: f64 = 500.0;
    pub const SEMIBOLD: f64 = 600.0;
    pub const BOLD: f64 = 700.0;
}

pub mod colors {
    /// Slate color palette for dark and light themes
    pub mod slate {
        pub const DARK_950: &str = "#020617";
        pub const DARK_900: &str = "#0f172a";
        pub const DARK_800: &str = "#1e293b";
        pub const DARK_700: &str = "#334155";
        pub const DARK_600: &str = "#475569";
        pub const DARK_500: &str = "#64748b";
        pub const DARK_400: &str = "#94a3b8";
        pub const DARK_300: &str = "#cbd5e1";
        pub const DARK_200: &str = "#e2e8f0";
        pub const DARK_100: &str = "#f1f5f9";
        pub const DARK_50: &str = "#f8fafc";
    }

    /// Semantic colors
    pub mod semantic {
        pub const SUCCESS: &str = "#22c55e";   // Green
        pub const WARNING: &str = "#f59e0b";   // Amber
        pub const ERROR: &str = "#ef4444";     // Red
        pub const INFO: &str = "#3b82f6";      // Blue
    }
}

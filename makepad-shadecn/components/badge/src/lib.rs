use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    // Base badge: pill background with small text
    pub ShadecnBadgeBase = <RoundedView> {
        width: Fit,
        height: Fit,
        padding: { left: (SPACE_2), right: (SPACE_2), top: (SPACE_1), bottom: (SPACE_1) },
        show_bg: true,
        draw_bg: {
            color: (COLOR_ACCENT),
            border_color: (COLOR_BORDER_PRIMARY),
            border_radius: (RADIUS_XL),
            border_size: 0.0,
        }

        label = <Label> {
            draw_text: {
                text_style: { font_size: (FONT_SM) },
                color: (COLOR_ACCENT_FOREGROUND),
            }
        }
    }

    // Solid badge using the primary background (dark)
    pub ShadecnBadge = <ShadecnBadgeBase> {
        draw_bg: { color: (COLOR_PRIMARY) }
        label = { draw_text: { color: (COLOR_PRIMARY_FOREGROUND) } }
    }

    // Secondary/neutral badge (light gray background)
    pub ShadecnBadgeSecondary = <ShadecnBadgeBase> {
        draw_bg: { color: (COLOR_BG_TERTIARY) }
        label = { draw_text: { color: (COLOR_FG_PRIMARY) } }
    }

    // Destructive badge (danger)
    pub ShadecnBadgeDestructive = <ShadecnBadgeBase> {
        draw_bg: { color: (COLOR_ERROR) }
        label = { draw_text: { color: (COLOR_PRIMARY_FOREGROUND) } }
    }

    // Outline badge: transparent background with subtle stroke
    pub ShadecnBadgeOutline = <ShadecnBadgeBase> {
        draw_bg: {
            color: (COLOR_WHITE_TRANSPARENT_00),
            border_color: (COLOR_BORDER_PRIMARY),
            border_size: 1.0,
            border_radius: (RADIUS_XL),
        }
        label = {
            draw_text: {
                color: (COLOR_FG_PRIMARY),
            }
        }
    }

    // Muted badge for disabled/secondary context
    pub ShadecnBadgeMuted = <ShadecnBadgeBase> {
        draw_bg: { color: (COLOR_BG_DISABLED) }
        label = { draw_text: { color: (COLOR_FG_DISABLED) } }
    }
}

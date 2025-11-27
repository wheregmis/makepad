use makepad_widgets::*;

live_design! {
    link widgets;
    link shadecn_tabs;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    link shadecn_core;
    use link::shadecn_core::*;

    pub ShadecnTabTrigger = <RadioButtonFlat> {
        width: Fit, height: Fit,
        padding: {left: (SPACE_3), right: (SPACE_3), top: (SPACE_2), bottom: (SPACE_2)}
        label_align: {y: 0.5}
        
        draw_bg: {
            // keep instances used by animator existing in base class
            instance down: 0.0,
            instance disabled: 0.0,
            instance active: 0.0,
            instance hover: 0.0,
            instance focus: 0.0,
            
            border_size: 0.0,
            border_radius: (RADIUS_MD),
            size: 24.0,
            
            // use defined shadecn_core tokens
            color: (COLOR_BG_TERTIARY),
            color_hover: (COLOR_BG_HOVER),
            color_down: (COLOR_BG_ACTIVE),
            color_active: (COLOR_BG_PRIMARY),
            color_focus: (COLOR_BG_ACTIVE),
            color_disabled: (COLOR_BG_DISABLED),
            
            border_color: (COLOR_BORDER_PRIMARY),
            border_color_hover: (COLOR_BORDER_PRIMARY),
            border_color_down: (COLOR_BORDER_PRIMARY),
            border_color_active: (COLOR_BORDER_PRIMARY),
            border_color_focus: (COLOR_BORDER_PRIMARY),
            border_color_disabled: (COLOR_BORDER_DISABLED),
            
            mark_color: (COLOR_BG_PRIMARY),
            mark_color_active: (COLOR_BG_PRIMARY),
            mark_color_disabled: (COLOR_BG_DISABLED),
        }
        
        draw_text: {
            text_style: { font_size: (FONT_BASE) }
            color: (COLOR_FG_TERTIARY),
            color_hover: (COLOR_FG_PRIMARY),
            color_down: (COLOR_FG_PRIMARY),
            color_active: (COLOR_FG_PRIMARY),
            color_focus: (COLOR_FG_PRIMARY),
            color_disabled: (COLOR_FG_DISABLED),
        }
    }

    pub ShadecnTabsList = <RoundedView> {
        width: Fit, height: Fit,
        flow: Right,
        padding: (SPACE_1),
        spacing: (SPACE_1),
        show_bg: true,
        draw_bg: {
            color: (COLOR_BG_TERTIARY)
            border_radius: (RADIUS_MD)
        }
    }
    
    pub ShadecnTabsContent = <View> {
        width: Fill, height: Fit,
        padding: {top: (SPACE_4)}
    }
}

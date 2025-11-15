use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnDropdownMenuBase = <DropDownFlat> {
        width: Fit,
        height: Fit,
        padding: {left: (SPACE_2), top: (SPACE_2), right: 28.0, bottom: (SPACE_2)},

        draw_bg: {
            border_size: 1.0,
            border_radius: (RADIUS_MD),
            color: (COLOR_BG_PRIMARY),
            color_hover: (COLOR_BG_HOVER),
            color_focus: (COLOR_BG_PRIMARY),
            color_down: (COLOR_BG_ACTIVE),
            color_disabled: (COLOR_BG_DISABLED),
            border_color: (COLOR_BORDER_PRIMARY),
            border_color_hover: (COLOR_BORDER_HOVER),
            border_color_focus: (COLOR_PRIMARY),
            border_color_down: (COLOR_PRIMARY),
            border_color_disabled: (COLOR_BORDER_DISABLED),
            arrow_color: (COLOR_FG_TERTIARY),
            arrow_color_hover: (COLOR_FG_PRIMARY),
            arrow_color_focus: (COLOR_PRIMARY),
            arrow_color_down: (COLOR_FG_PRIMARY),
            arrow_color_disabled: (COLOR_FG_DISABLED),
        }

        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_FG_PRIMARY),
            color_hover: (COLOR_FG_PRIMARY),
            color_focus: (COLOR_FG_PRIMARY),
            color_down: (COLOR_FG_PRIMARY),
            color_disabled: (COLOR_FG_DISABLED),
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                border_size: 1.0,
                border_radius: (RADIUS_MD),
                gradient_border_horizontal: 0.0,
                gradient_fill_horizontal: 0.0,
                color_dither: 0.0,
                color: (COLOR_BG_PRIMARY),
                color_2: vec4(-1.0, -1.0, -1.0, -1.0),
                border_color: (COLOR_BORDER_PRIMARY),
                border_color_2: vec4(-1.0, -1.0, -1.0, -1.0),
            }

            menu_item = <PopupMenuItem> {
                padding: {left: (SPACE_3), top: (SPACE_2), right: (SPACE_3), bottom: (SPACE_2)},

                draw_bg: {
                    gradient_border_horizontal: 0.0,
                    gradient_fill_horizontal: 0.0,
                    color_dither: 0.0,
                    color: (COLOR_BG_PRIMARY),
                    color_hover: (COLOR_BG_HOVER),
                    color_active: (COLOR_BG_ACTIVE),
                    color_disabled: (COLOR_BG_DISABLED),
                    color_2: vec4(-1.0, -1.0, -1.0, -1.0),
                    color_2_hover: vec4(-1.0, -1.0, -1.0, -1.0),
                    color_2_active: vec4(-1.0, -1.0, -1.0, -1.0),
                    color_2_disabled: vec4(-1.0, -1.0, -1.0, -1.0),
                }

                draw_text: {
                    text_style: {
                        font_size: (FONT_BASE),
                    }
                    color: (COLOR_FG_PRIMARY),
                    color_hover: (COLOR_FG_PRIMARY),
                    color_active: (COLOR_FG_PRIMARY),
                    color_disabled: (COLOR_FG_DISABLED),
                }
            }
        }
    }

    pub ShadecnDropdownMenu = <ShadecnDropdownMenuBase> {}
}

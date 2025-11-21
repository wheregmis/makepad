use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    // Custom PopupMenuItem for Shadecn design system
    pub ShadecnPopupMenuItem = <PopupMenuItemBase> {
        // Fit to label size so the popup can size itself tightly
        width: Fit,
        height: Fit,
        align: { y: 0.5 }
        padding: {left: (SPACE_3), top: (SPACE_2), right: (SPACE_3), bottom: (SPACE_2)},

        draw_text: {
            instance active: 0.0
            instance hover: 0.0
            instance disabled: 0.0

            uniform color: (COLOR_FG_PRIMARY),
            uniform color_hover: (COLOR_FG_PRIMARY),
            uniform color_active: (COLOR_FG_PRIMARY),
            uniform color_disabled: (COLOR_FG_DISABLED),

            text_style: <THEME_FONT_REGULAR> {
                font_size: (FONT_BASE),
            }

            fn get_color(self) -> vec4 {
                // Always return the primary color for visibility
                return self.color
            }
        }

        draw_bg: {
            instance active: 0.0
            instance hover: 0.0
            instance disabled: 0.0

            uniform gradient_border_horizontal: 0.0
            uniform gradient_fill_horizontal: 0.0
            uniform color_dither: 0.0
            uniform border_size: 0.0
            uniform border_radius: 0.0

            uniform color: (COLOR_BG_PRIMARY),
            uniform color_hover: (COLOR_BG_HOVER),
            uniform color_active: (COLOR_BG_ACTIVE),
            uniform color_disabled: (COLOR_BG_DISABLED),

            uniform color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform color_2_hover: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform color_2_active: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform color_2_disabled: vec4(-1.0, -1.0, -1.0, -1.0)

            uniform border_color: vec4(0.0, 0.0, 0.0, 0.0)
            uniform border_color_hover: vec4(0.0, 0.0, 0.0, 0.0)
            uniform border_color_active: vec4(0.0, 0.0, 0.0, 0.0)
            uniform border_color_disabled: vec4(0.0, 0.0, 0.0, 0.0)

            uniform border_color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform border_color_2_hover: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform border_color_2_active: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform border_color_2_disabled: vec4(-1.0, -1.0, -1.0, -1.0)

            uniform mark_color: vec4(0.0, 0.0, 0.0, 0.0)
            uniform mark_color_active: vec4(0.0, 0.0, 0.0, 0.0)
            uniform mark_color_disabled: vec4(0.0, 0.0, 0.0, 0.0)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                
                let bg_color = mix(
                    mix(
                        mix(
                            self.color,
                            self.color_active,
                            self.active
                        ),
                        self.color_hover,
                        self.hover
                    ),
                    self.color_disabled,
                    self.disabled
                );

                sdf.rect(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y
                );
                sdf.fill(bg_color);

                return sdf.result;
            }
        }
    }

    // Custom PopupMenuFlat for Shadecn design system
    pub ShadecnPopupMenuFlat = <PopupMenuBase> {
        // Let the menu size itself to its content instead of stretching full width
        width: Fit,
        height: Fit,
        flow: Down,
        padding: {left: (SPACE_1), top: (SPACE_1), right: (SPACE_1), bottom: (SPACE_1)},

        menu_item: <ShadecnPopupMenuItem> {}

        draw_bg: {
            uniform border_size: 1.0
            uniform gradient_border_horizontal: 0.0
            uniform gradient_fill_horizontal: 0.0
            uniform border_radius: (RADIUS_MD)
            uniform color: (COLOR_BG_PRIMARY)
            uniform color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform border_color: (COLOR_BORDER_PRIMARY)
            uniform border_color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform color_dither: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                
                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.0,
                    self.rect_size.y - self.border_size * 2.0,
                    self.border_radius
                );
                sdf.fill_keep(self.color);

                if self.border_size > 0.0 {
                    sdf.stroke(self.border_color, self.border_size);
                }

                return sdf.result;
            }
        }
    }

    pub ShadecnDropdownMenuBase = <DropDownFlat> {
        width: Fit,
        height: Fit,
        padding: {left: (SPACE_2), top: (SPACE_2), right: 28.0, bottom: (SPACE_2)},
        popup_menu_position: BelowInput,

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

        popup_menu: <ShadecnPopupMenuFlat> {}
    }

    pub ShadecnDropdownMenu = <ShadecnDropdownMenuBase> {}
}

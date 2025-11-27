use makepad_widgets::*;

live_design! {
    link widgets;
    link shadecn_button;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnButtonBase = <ButtonFlat> {
        width: Fit,
        height: Fit,
        padding: {left: (SPACE_3), top: 6, right: (SPACE_3), bottom: 6},
        spacing: (SPACE_2),
        reset_hover_on_click: true,

        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {down: 0.0, hover: 0.0}
                        draw_icon: {down: 0.0, hover: 0.0}
                        draw_text: {down: 0.0, hover: 0.0}
                    }
                }
                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_icon: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {down: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }
                down = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_icon: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {down: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }

    pub ShadecnButtonPrimary = <ShadecnButtonBase> {
        draw_bg: {
            border_size: 0.0,
            border_radius: (RADIUS_MD),
            color: (COLOR_PRIMARY),
            color_hover: (COLOR_ACCENT_DARKER),
            color_down: (COLOR_FG_PRIMARY),
            color_focus: (COLOR_FG_PRIMARY),
            color_disabled: (COLOR_BG_DISABLED),
            border_color: (COLOR_PRIMARY),
            border_color_hover: (COLOR_ACCENT_DARKER),
            border_color_down: (COLOR_FG_PRIMARY),
            border_color_focus: (COLOR_FG_PRIMARY),
            border_color_disabled: (COLOR_BORDER_DISABLED),
        }

        draw_text: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0,

            color: (COLOR_PRIMARY_FOREGROUND),
            uniform color_hover: (COLOR_PRIMARY_FOREGROUND),
            uniform color_down: (COLOR_PRIMARY_FOREGROUND),
            uniform color_focus: (COLOR_PRIMARY_FOREGROUND),
            uniform color_disabled: (COLOR_FG_DISABLED),

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }
    }

    pub ShadecnButtonSecondary = <ShadecnButtonBase> {
        draw_bg: {
            border_size: 0.0,
            border_radius: (RADIUS_MD),
            color: (COLOR_BG_TERTIARY),
            color_hover: (COLOR_ACCENT_DARK),
            color_down: (COLOR_ACCENT_DARKER),
            color_focus: (COLOR_ACCENT_DARKER),
            color_disabled: (COLOR_BG_DISABLED),
            border_color: (COLOR_BG_TERTIARY),
            border_color_hover: (COLOR_ACCENT_DARK),
            border_color_down: (COLOR_ACCENT_DARKER),
            border_color_focus: (COLOR_ACCENT_DARKER),
            border_color_disabled: (COLOR_BORDER_DISABLED),
        }

        draw_text: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0,

            color: (COLOR_FG_PRIMARY),
            uniform color_hover: (COLOR_FG_PRIMARY),
            uniform color_down: (COLOR_FG_PRIMARY),
            uniform color_focus: (COLOR_FG_PRIMARY),
            uniform color_disabled: (COLOR_FG_DISABLED),

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }
    }

    pub ShadecnButtonOutline = <ShadecnButtonBase> {
        draw_bg: {
            border_size: 1.5,
            border_radius: (RADIUS_MD),
            color: (COLOR_BG_PRIMARY),
            color_hover: (COLOR_BG_TERTIARY),
            color_down: (COLOR_BG_ACTIVE),
            color_focus: (COLOR_ACCENT_LIGHT),
            color_disabled: (COLOR_BG_DISABLED),
            border_color: (COLOR_BORDER_PRIMARY),
            border_color_hover: (COLOR_BORDER_HOVER),
            border_color_down: (COLOR_BORDER_PRIMARY),
            border_color_focus: (COLOR_ACCENT_FOCUS),
            border_color_disabled: (COLOR_BORDER_DISABLED),
        }

        draw_text: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0,

            color: (COLOR_FG_PRIMARY),
            uniform color_hover: (COLOR_FG_PRIMARY),
            uniform color_down: (COLOR_FG_PRIMARY),
            uniform color_focus: (COLOR_FG_PRIMARY),
            uniform color_disabled: (COLOR_FG_DISABLED),

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }
    }

    pub ShadecnButtonDestructive = <ShadecnButtonBase> {
        draw_bg: {
            border_size: 0.0,
            border_radius: (RADIUS_MD),
            color: (COLOR_ERROR),
            color_hover: (COLOR_ERROR_HOVER),
            color_down: (COLOR_ERROR_DARK),
            color_focus: (COLOR_ERROR_DARKER),
            color_disabled: (COLOR_BG_DISABLED),
            border_color: (COLOR_ERROR),
            border_color_hover: (COLOR_ERROR_HOVER),
            border_color_down: (COLOR_ERROR_DARK),
            border_color_focus: (COLOR_ERROR_DARKER),
            border_color_disabled: (COLOR_BORDER_DISABLED),
        }

        draw_text: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0,

            color: (COLOR_WHITE),
            uniform color_hover: (COLOR_WHITE),
            uniform color_down: (COLOR_WHITE),
            uniform color_focus: (COLOR_WHITE),
            uniform color_disabled: (COLOR_FG_DISABLED),

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }
    }

    pub ShadecnButtonGhost = <ShadecnButtonBase> {
        draw_bg: {
            border_size: 0.0,
            border_radius: (RADIUS_MD),
            color: (COLOR_DARK_TRANSPARENT_00),
            color_hover: (COLOR_DARK_TRANSPARENT_1A),
            color_down: (COLOR_DARK_TRANSPARENT_33),
            color_focus: (COLOR_DARK_TRANSPARENT_47),
            color_disabled: (COLOR_DARK_TRANSPARENT_00),
            border_color: (COLOR_DARK_TRANSPARENT_00),
            border_color_hover: (COLOR_DARK_TRANSPARENT_1A),
            border_color_down: (COLOR_DARK_TRANSPARENT_33),
            border_color_focus: (COLOR_DARK_TRANSPARENT_47),
            border_color_disabled: (COLOR_DARK_TRANSPARENT_00),
        }

        draw_text: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0,

            color: (COLOR_FG_PRIMARY),
            uniform color_hover: (COLOR_FG_PRIMARY),
            uniform color_down: (COLOR_FG_PRIMARY),
            uniform color_focus: (COLOR_FG_PRIMARY),
            uniform color_disabled: (COLOR_FG_DISABLED),

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }
    }

    pub ShadecnButton = <ShadecnButtonPrimary> {}

    pub ShadecnButtonIcon = <ShadecnButtonBase> {
        spacing: 0.0,
        text: "",
        padding: {left: (SPACE_2), top: (SPACE_2), right: (SPACE_2), bottom: (SPACE_2)},

        draw_bg: {
            border_size: 0.0,
            border_radius: (RADIUS_MD),
            color: (COLOR_DARK_TRANSPARENT_00),
            color_hover: (COLOR_DARK_TRANSPARENT_1A),
            color_down: (COLOR_DARK_TRANSPARENT_33),
            color_focus: (COLOR_DARK_TRANSPARENT_47),
            color_disabled: (COLOR_DARK_TRANSPARENT_00),
            border_color: (COLOR_DARK_TRANSPARENT_00),
            border_color_hover: (COLOR_DARK_TRANSPARENT_1A),
            border_color_down: (COLOR_DARK_TRANSPARENT_33),
            border_color_focus: (COLOR_DARK_TRANSPARENT_47),
            border_color_disabled: (COLOR_DARK_TRANSPARENT_00),
        }

        draw_icon: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0,

            color: (COLOR_FG_PRIMARY),
            uniform color_hover: (COLOR_PRIMARY),
            uniform color_down: (COLOR_ACCENT_DARKER),
            uniform color_focus: (COLOR_ACCENT_FOCUS),
            uniform color_disabled: (COLOR_FG_DISABLED),

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(self.color, self.color_focus, self.focus),
                            self.color_hover,
                            self.hover
                        ),
                        self.color_down,
                        self.down
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }

        icon_walk: {
            width: (SPACE_4),
            height: (SPACE_4),
        }
    }
}

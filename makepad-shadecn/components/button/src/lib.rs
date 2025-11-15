use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use makepad_draw::shader::std::*;

    pub ShadecnButtonBase = <Button> {
        width: Fit,
        height: Fit,
        margin: {left: 0, top: 0, right: 0, bottom: 0},
        padding: {left: (SPACE_3), top: 6, right: (SPACE_3), bottom: 6},
        spacing: (SPACE_2),

        text: "Button"
        align: {x: 0.5, y: 0.5},
        
        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_WHITE)
        }
    }

    pub ShadecnButtonPrimary = <ShadecnButtonBase> {
        draw_bg: {
            uniform border_radius: (RADIUS_MD),
            uniform accent_color: (COLOR_ACCENT),
            uniform accent_hover: (COLOR_ACCENT_HOVER),
            uniform accent_dark: (COLOR_ACCENT_DARK),
            uniform accent_darker: (COLOR_ACCENT_DARKER),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                let color_hover = mix(self.accent_color, self.accent_hover, self.hover);
                let color_press = mix(color_hover, self.accent_dark, self.down);
                let color_focus = mix(color_press, self.accent_darker, self.focus);
                sdf.fill(color_focus);
                return sdf.result;
            }
        }
    }

    pub ShadecnButtonSecondary = <ShadecnButtonBase> {
        draw_bg: {
            uniform border_radius: (RADIUS_MD),
            uniform bg_color: (COLOR_SLATE_200),
            uniform bg_hover: (COLOR_BORDER_HOVER),
            uniform bg_down: (COLOR_SLATE_400),
            uniform bg_focus: (COLOR_BORDER_SECONDARY),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                let color_hover = mix(self.bg_color, self.bg_hover, self.hover);
                let color_press = mix(color_hover, self.bg_down, self.down);
                let color_focus = mix(color_press, self.bg_focus, self.focus);
                sdf.fill(color_focus);
                return sdf.result;
            }
        }
        
        draw_text: {
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnButtonOutline = <ShadecnButtonBase> {
        draw_bg: {
            uniform border_width: 1.5,
            uniform border_radius: (RADIUS_MD),
            uniform bg_color: (COLOR_BG_PRIMARY),
            uniform bg_hover: (COLOR_BG_SECONDARY),
            uniform bg_down: (COLOR_BG_ACTIVE),
            uniform bg_focus: (COLOR_ACCENT_LIGHT),
            uniform border_color: (COLOR_BORDER_PRIMARY),
            uniform border_focus: (COLOR_SLATE_400),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                let base = self.bg_color;
                let hover = mix(base, self.bg_hover, self.hover);
                let down = mix(hover, self.bg_down, self.down);
                let focus = mix(down, self.bg_focus, self.focus);
                sdf.fill_keep(focus);
                let border = mix(self.border_color, self.border_focus, self.focus);
                sdf.stroke(border, self.border_width);
                return sdf.result;
            }
        }

        draw_text: {
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnButtonDestructive = <ShadecnButtonBase> {
        draw_bg: {
            uniform border_radius: (RADIUS_MD),
            uniform error_color: (COLOR_ERROR),
            uniform error_hover: (COLOR_ERROR_HOVER),
            uniform error_dark: (COLOR_ERROR_DARK),
            uniform error_darker: (COLOR_ERROR_DARKER),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                let color_hover = mix(self.error_color, self.error_hover, self.hover);
                let color_press = mix(color_hover, self.error_dark, self.down);
                let color_focus = mix(color_press, self.error_darker, self.focus);
                sdf.fill(color_focus);
                return sdf.result;
            }
        }

        draw_text: {
            color: (COLOR_WHITE)
        }
    }

    pub ShadecnButtonGhost = <ShadecnButtonBase> {
        draw_bg: {
            uniform border_radius: (RADIUS_MD),
            uniform ghost_base: (COLOR_DARK_TRANSPARENT_00),
            uniform ghost_hover: (COLOR_DARK_TRANSPARENT_1A),
            uniform ghost_down: (COLOR_DARK_TRANSPARENT_33),
            uniform ghost_focus: (COLOR_DARK_TRANSPARENT_47),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                let base = self.ghost_base;
                let hover = mix(base, self.ghost_hover, self.hover);
                let down = mix(hover, self.ghost_down, self.down);
                let focus = mix(down, self.ghost_focus, self.focus);
                sdf.fill(focus);
                return sdf.result;
            }
        }

        draw_text: {
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnButton = <ShadecnButtonPrimary> {}
}

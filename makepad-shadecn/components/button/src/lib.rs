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
        padding: {left: 12, top: 6, right: 12, bottom: 6},
        spacing: 8,

        text: "Button"
        align: {x: 0.5, y: 0.5},
        
        draw_text: {
            text_style: {
                font_size: 14.0,
            }
            color: #ffffff
        }
    }

    pub ShadecnButtonPrimary = <ShadecnButtonBase> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                let color_hover = mix(#3b82f6, vec4(0.145, 0.388, 0.922, 1.0), self.hover);
                let color_press = mix(color_hover, #1d4ed8, self.down);
                let color_focus = mix(color_press, #1e3a8a, self.focus);
                sdf.fill(color_focus);
                return sdf.result;
            }
        }
    }

    pub ShadecnButtonSecondary = <ShadecnButtonBase> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                let color_hover = mix(#e2e8f0, #cbd5e1, self.hover);
                let color_press = mix(color_hover, #94a3b8, self.down);
                let color_focus = mix(color_press, #cbd5e1, self.focus);
                sdf.fill(color_focus);
                return sdf.result;
            }
        }
        
        draw_text: {
            color: #0f172a
        }
    }

    pub ShadecnButtonOutline = <ShadecnButtonBase> {
        draw_bg: {
            uniform border_width: 1.5,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 6.0);
                let base = #ffffff;
                let hover = mix(base, #f8fafc, self.hover);
                let down = mix(hover, #e2e8f0, self.down);
                let focus = mix(down, #dbeafe, self.focus);
                sdf.fill_keep(focus);
                let border = mix(#e2e8f0, #94a3b8, self.focus);
                sdf.stroke(border, self.border_width);
                return sdf.result;
            }
        }

        draw_text: {
            color: #0f172a
        }
    }

    pub ShadecnButtonDestructive = <ShadecnButtonBase> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                let color_hover = mix(#ef4444, #dc2626, self.hover);
                let color_press = mix(color_hover, #b91c1c, self.down);
                let color_focus = mix(color_press, #991b1b, self.focus);
                sdf.fill(color_focus);
                return sdf.result;
            }
        }

        draw_text: {
            color: #ffffff
        }
    }

    pub ShadecnButtonGhost = <ShadecnButtonBase> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                let base = #0f172a00;
                let hover = mix(base, #0f172a1a, self.hover);
                let down = mix(hover, #0f172a33, self.down);
                let focus = mix(down, #0f172a47, self.focus);
                sdf.fill(focus);
                return sdf.result;
            }
        }

        draw_text: {
            color: #0f172a
        }
    }

    pub ShadecnButton = <ShadecnButtonPrimary> {}
}

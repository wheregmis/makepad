use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use makepad_draw::shader::std::*;

    pub ShadecnCardBase = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 0,
        margin: {left: 0, top: 0, right: 0, bottom: 0},
        padding: {left: (SPACE_6), top: (SPACE_6), right: (SPACE_6), bottom: (SPACE_6)},
        show_bg: true,

        draw_bg: {
            uniform border_radius: (RADIUS_LG),
            uniform border_width: 1.0,
            uniform bg_color: (COLOR_BG_PRIMARY),
            uniform border_color: (COLOR_BORDER_PRIMARY),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }
    }

    pub ShadecnCard = <ShadecnCardBase> {}

    pub ShadecnCardHeader = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: (SPACE_1),
        margin: {left: 0, top: 0, right: 0, bottom: (SPACE_4)},
        padding: {left: 0, top: 0, right: 0, bottom: 0},
    }

    pub ShadecnCardTitle = <Label> {
        draw_text: {
            text_style: {
                font_size: (FONT_XL),
            }
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnCardDescription = <Label> {
        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_SLATE_500)
        }
    }

    pub ShadecnCardContent = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: (SPACE_4),
    }

    pub ShadecnCardFooter = <View> {
        width: Fill,
        height: Fit,
        flow: Right { wrap: true },
        spacing: (SPACE_2),
        margin: {left: 0, top: (SPACE_4), right: 0, bottom: 0},
        padding: {left: 0, top: 0, right: 0, bottom: 0},
    }
}

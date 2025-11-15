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
        padding: {left: 24, top: 24, right: 24, bottom: 24},
        show_bg: true,

        draw_bg: {
            uniform border_radius: 12.0,
            uniform border_width: 1.0,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill_keep(#ffffff);
                sdf.stroke(#e2e8f0, self.border_width);
                return sdf.result;
            }
        }
    }

    pub ShadecnCard = <ShadecnCardBase> {}

    pub ShadecnCardHeader = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 4,
        margin: {left: 0, top: 0, right: 0, bottom: 16},
        padding: {left: 0, top: 0, right: 0, bottom: 0},
    }

    pub ShadecnCardTitle = <Label> {
        draw_text: {
            text_style: {
                font_size: 18.0,
            }
            color: #0f172a
        }
    }

    pub ShadecnCardDescription = <Label> {
        draw_text: {
            text_style: {
                font_size: 14.0,
            }
            color: #64748b
        }
    }

    pub ShadecnCardContent = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 16,
    }

    pub ShadecnCardFooter = <View> {
        width: Fill,
        height: Fit,
        flow: Right { wrap: true },
        spacing: 8,
        margin: {left: 0, top: 16, right: 0, bottom: 0},
        padding: {left: 0, top: 0, right: 0, bottom: 0},
    }
}

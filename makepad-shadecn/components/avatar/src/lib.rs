use makepad_shadecn_core::theme::*;
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnAvatar = <View> {
        width: 40.0,
        height: 40.0,
        flow: Overlay,
        align: {x: 0.5, y: 0.5},
    }

    pub ShadecnAvatarImage = <Image> {
        width: Fill,
        height: Fill,
        fit: Cover,
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, self.rect_size.x * 0.5);

                let color = self.get_color();
                sdf.fill_keep(color);
                return sdf.result;
            }
        }
    }

    pub ShadecnAvatarFallback = <View> {
        width: Fill,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: (COLOR_SLATE_200),
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, self.rect_size.x * 0.5);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        align: {x: 0.5, y: 0.5},

        label = <Label> {
            draw_text: {
                text_style: { font_size: (FONT_BASE) },
                color: (COLOR_FG_PRIMARY),
            }
            text: "CN"
        }
    }
}

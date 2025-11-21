use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnBreadcrumb = <View> {
        width: Fit, height: Fit,
        flow: Right,
        spacing: (SPACE_2),
        align: {y: 0.5}
    }

    pub ShadecnBreadcrumbLink = <Button> {
        width: Fit, height: Fit,
        padding: 0.0,
        
        draw_bg: {
            fn pixel(self) -> vec4 { return vec4(0.0) }
        }
        
        draw_text: {
            text_style: { font_size: (FONT_SM) }
            color: (COLOR_FG_MUTED)
            color_hover: (COLOR_FG_PRIMARY)
            
            fn get_color(self) -> vec4 {
                return mix(
                    self.color,
                    self.color_hover,
                    self.hover
                )
            }
        }
        
        draw_icon: {
            fn get_color(self) -> vec4 { return vec4(0.0) }
        }
    }

    pub ShadecnBreadcrumbSeparator = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: { font_size: (FONT_SM) }
            color: (COLOR_FG_MUTED)
        }
        text: "/"
    }
    
    pub ShadecnBreadcrumbPage = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: { font_size: (FONT_SM) }
            color: (COLOR_FG_PRIMARY)
        }
    }
}

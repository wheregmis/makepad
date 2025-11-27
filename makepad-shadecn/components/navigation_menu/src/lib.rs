use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnNavigationMenuItem = <ButtonFlat> {
        width: Fit, height: Fit,
        padding: {left: (SPACE_3), right: (SPACE_3), top: (SPACE_2), bottom: (SPACE_2)}
        
        draw_text: {
            text_style: { font_size: (FONT_BASE) }
            color: (COLOR_FG_PRIMARY)
        }
        
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
            color_hover: (COLOR_ACCENT)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    4.0
                );
                
                let bg_color = mix(
                    self.color,
                    self.color_hover,
                    self.hover
                );
                
                sdf.fill(bg_color);
                return sdf.result;
            }
        }
    }

    pub ShadecnNavigationMenu = <View> {
        width: Fit, height: Fit,
        flow: Right,
        spacing: (SPACE_1),
        
        item1 = <ShadecnNavigationMenuItem> { text: "Getting Started" }
        item2 = <ShadecnNavigationMenuItem> { text: "Components" }
        item3 = <ShadecnNavigationMenuItem> { text: "Documentation" }
    }
}

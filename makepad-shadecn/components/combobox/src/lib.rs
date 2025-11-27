use makepad_widgets::*;

live_design! {
    link widgets;
    link shadecn_combobox;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    link shadecn_core;
    use link::shadecn_core::*;

    pub ShadecnCombobox = <DropDown> {
        width: Fill, height: Fit,
        padding: {left: (SPACE_3), right: (SPACE_3), top: (SPACE_2), bottom: (SPACE_2)}
        
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance open: 0.0
            
            color: (COLOR_BG_PRIMARY)
            border_color: (COLOR_BORDER_PRIMARY)
            border_radius: (RADIUS_MD)
            border_size: 1.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.0,
                    self.rect_size.y - self.border_size * 2.0,
                    self.border_radius
                );
                sdf.fill(self.color);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }
        
        draw_text: {
            text_style: { font_size: (FONT_BASE) }
            color: (COLOR_FG_PRIMARY)
        }
        
        popup_menu: <PopupMenu> {}
    }
}

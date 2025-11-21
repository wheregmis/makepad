use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnSelect = <DropDown> {
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
        
        draw_icon: {
            color: (COLOR_FG_MUTED)
        }
        
        popup_menu: <PopupMenu> {
            draw_bg: {
                color: (COLOR_BG_POPOVER)
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
            
            draw_item: {
                color: (COLOR_BG_POPOVER)
                color_selected: (COLOR_ACCENT)
                color_hover: (COLOR_ACCENT_HOVER)
                
                text_style: { font_size: (FONT_BASE) }
                color_text: (COLOR_FG_PRIMARY)
                color_text_selected: (COLOR_FG_ACCENT)
                
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.0,
                        0.0,
                        self.rect_size.x,
                        self.rect_size.y,
                        2.0 // Small radius for items
                    );
                    
                    let bg_color = self.color;
                    if self.selected > 0.5 {
                        bg_color = self.color_selected;
                    } else if self.hover > 0.5 {
                        bg_color = self.color_hover;
                    }
                    
                    sdf.fill(bg_color);
                    return sdf.result;
                }
            }
        }
    }
}

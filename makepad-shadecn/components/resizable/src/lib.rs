use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnResizable = <Splitter> {
        size: 1.0
        
        draw_bg: {
            color: (COLOR_BORDER_PRIMARY)
            color_hover: (COLOR_ACCENT)
            color_drag: (COLOR_ACCENT_FOCUS)
            
            splitter_pad: 0.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                
                if self.is_vertical > 0.5 {
                    sdf.box(
                        0.0,
                        0.0,
                        self.rect_size.x,
                        self.rect_size.y,
                        0.0
                    );
                }
                else {
                    sdf.box(
                        0.0,
                        0.0,
                        self.rect_size.x,
                        self.rect_size.y,
                        0.0
                    );
                }

                return sdf.fill_keep(
                    mix(
                        self.color,
                        mix(
                            self.color_hover,
                            self.color_drag,
                            self.drag
                        ),
                        self.hover
                    )
                );
            }
        }
    }
}

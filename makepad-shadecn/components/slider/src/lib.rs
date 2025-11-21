use makepad_widgets::*;
use makepad_shadecn_core::theme::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnSlider = <Slider> {
        height: 20.0,
        width: Fill,
        padding: 0.0,
        
        // Hide text input and label
        text_input: {
            width: 0.0,
            height: 0.0,
            padding: 0.0,
            margin: 0.0,
            draw_text: { color: #00000000 }
            draw_bg: { color: #00000000 }
        }
        draw_text: {
            color: #00000000
        }
        label_walk: {
            width: 0.0,
            height: 0.0,
            margin: 0.0
        }

        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance drag: 0.0
            instance disabled: 0.0
            
            label_size: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                
                let h = self.rect_size.y;
                let w = self.rect_size.x;
                
                let track_height = 8.0; // SPACE_2
                let track_y = (h - track_height) * 0.5;
                
                // Track Background
                sdf.box(
                    0.0,
                    track_y,
                    w,
                    track_height,
                    track_height * 0.5
                );
                sdf.fill((COLOR_BG_TERTIARY));
                
                // Track Fill
                let fill_width = self.slide_pos * w;
                sdf.box(
                    0.0,
                    track_y,
                    fill_width,
                    track_height,
                    track_height * 0.5
                );
                sdf.fill((COLOR_PRIMARY));
                
                // Thumb
                let thumb_size = 20.0;
                // Center the thumb on the slide position
                // We need to clamp it so it doesn't get cut off if we want, but usually slider thumb center is at value.
                // However, if we draw outside rect, it might be clipped.
                // Slider widget usually handles input mapping.
                
                let thumb_center_x = self.slide_pos * w;
                
                sdf.circle(thumb_center_x, h * 0.5, thumb_size * 0.5);
                sdf.fill((COLOR_BG_PRIMARY));
                sdf.stroke((COLOR_BORDER_PRIMARY), 2.0); 
                
                return sdf.result
            }
        }
    }
}

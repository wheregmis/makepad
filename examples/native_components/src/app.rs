use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { title: "Native Components Demo" },
                body = <View> {
                    flow: Down,
                    spacing: 20,
                    padding: 30,
                    
                    // Header
                    <Label> {
                        text: "Native Component Embedding Demo"
                        draw_text: {
                            text_style: { font_size: 24.0 }
                            color: #fff
                        }
                    }
                    
                    <Label> {
                        text: "This example demonstrates embedding native platform UI components in Makepad."
                        draw_text: {
                            text_style: { font_size: 14.0 }
                            color: #aaa
                        }
                    }
                    
                    // Native Button Section
                    <View> {
                        flow: Down,
                        spacing: 10,
                        padding: 20,
                        show_bg: true,
                        draw_bg: {
                            color: #333
                        }
                        
                        <Label> {
                            text: "Native Button"
                            draw_text: {
                                text_style: { font_size: 16.0 }
                                color: #fff
                            }
                        }
                        
                        native_button = <NativeView> {
                            width: 200,
                            height: 50,
                            view_kind: Button,
                            label: "Click Me!"
                        }
                        
                        button_status = <Label> {
                            text: "Status: Not clicked"
                            draw_text: {
                                text_style: { font_size: 12.0 }
                                color: #888
                            }
                        }
                    }
                    
                    // Native TextField Section
                    <View> {
                        flow: Down,
                        spacing: 10,
                        padding: 20,
                        show_bg: true,
                        draw_bg: {
                            color: #333
                        }
                        
                        <Label> {
                            text: "Native Text Field"
                            draw_text: {
                                text_style: { font_size: 16.0 }
                                color: #fff
                            }
                        }
                        
                        native_text_field = <NativeView> {
                            width: 300,
                            height: 44,
                            view_kind: TextField,
                            placeholder: "Enter text here..."
                            text: ""
                        }
                        
                        text_status = <Label> {
                            text: "Text: (empty)"
                            draw_text: {
                                text_style: { font_size: 12.0 }
                                color: #888
                            }
                        }
                    }
                    
                    // Native Switch Section
                    <View> {
                        flow: Down,
                        spacing: 10,
                        padding: 20,
                        show_bg: true,
                        draw_bg: {
                            color: #333
                        }
                        
                        <Label> {
                            text: "Native Switch"
                            draw_text: {
                                text_style: { font_size: 16.0 }
                                color: #fff
                            }
                        }
                        
                        native_switch = <NativeView> {
                            width: 60,
                            height: 32,
                            view_kind: Switch,
                            switch_on: false
                        }
                        
                        switch_status = <Label> {
                            text: "State: Off"
                            draw_text: {
                                text_style: { font_size: 12.0 }
                                color: #888
                            }
                        }
                    }
                    
                    // Native Slider Section
                    <View> {
                        flow: Down,
                        spacing: 10,
                        padding: 20,
                        show_bg: true,
                        draw_bg: {
                            color: #333
                        }
                        
                        <Label> {
                            text: "Native Slider"
                            draw_text: {
                                text_style: { font_size: 16.0 }
                                color: #fff
                            }
                        }
                        
                        native_slider = <NativeView> {
                            width: 250,
                            height: 32,
                            view_kind: Slider,
                            slider_value: 0.5,
                            slider_min: 0.0,
                            slider_max: 1.0
                        }
                        
                        slider_status = <Label> {
                            text: "Value: 0.50"
                            draw_text: {
                                text_style: { font_size: 12.0 }
                                color: #888
                            }
                        }
                    }
                    
                    // Native Progress Section
                    <View> {
                        flow: Down,
                        spacing: 10,
                        padding: 20,
                        show_bg: true,
                        draw_bg: {
                            color: #333
                        }
                        
                        <Label> {
                            text: "Native Progress Indicator"
                            draw_text: {
                                text_style: { font_size: 16.0 }
                                color: #fff
                            }
                        }
                        
                        native_progress = <NativeView> {
                            width: 250,
                            height: 8,
                            view_kind: ProgressIndicator,
                            progress: 0.3
                        }
                        
                        progress_status = <Label> {
                            text: "Progress: 30%"
                            draw_text: {
                                text_style: { font_size: 12.0 }
                                color: #888
                            }
                        }
                    }
                    
                    // Info section
                    <View> {
                        flow: Down,
                        spacing: 5,
                        padding: 20,
                        
                        <Label> {
                            text: "Note: Native components render to textures and are composited into Makepad's GPU pipeline."
                            draw_text: {
                                text_style: { font_size: 11.0 }
                                color: #666
                            }
                        }
                        
                        <Label> {
                            text: "This allows seamless integration with Makepad's layout and rendering system."
                            draw_text: {
                                text_style: { font_size: 11.0 }
                                color: #666
                            }
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] click_count: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        log!("Native Components Demo started");
    }
    
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle native button clicks
        let native_button = self.ui.native_view(ids!(native_button));
        if native_button.clicked(actions) {
            self.click_count += 1;
            self.ui.label(ids!(button_status))
                .set_text(cx, &format!("Status: Clicked {} times!", self.click_count));
        }
        
        // Handle text field changes
        let native_text_field = self.ui.native_view(ids!(native_text_field));
        if let Some(text) = native_text_field.text_changed(actions) {
            let display = if text.is_empty() { "(empty)".to_string() } else { text };
            self.ui.label(ids!(text_status))
                .set_text(cx, &format!("Text: {}", display));
        }
        
        // Handle switch changes
        let native_switch = self.ui.native_view(ids!(native_switch));
        if let Some(on) = native_switch.switch_changed(actions) {
            self.ui.label(ids!(switch_status))
                .set_text(cx, &format!("State: {}", if on { "On" } else { "Off" }));
        }
        
        // Handle slider changes
        let native_slider = self.ui.native_view(ids!(native_slider));
        if let Some(value) = native_slider.slider_changed(actions) {
            self.ui.label(ids!(slider_status))
                .set_text(cx, &format!("Value: {:.2}", value));
            
            // Update progress bar to match slider
            self.ui.native_view(ids!(native_progress))
                .set_progress(cx, value);
            self.ui.label(ids!(progress_status))
                .set_text(cx, &format!("Progress: {:.0}%", value * 100.0));
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

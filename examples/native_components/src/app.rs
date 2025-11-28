use makepad_native_components::*;
use makepad_widgets::*;

live_design! {
    link widgets;
    link native_components;
    use link::widgets::*;
    use link::native_components::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { title: "Native Components Demo" },
                body = <ScrollYView> {
                    flow: Down,
                    spacing: 20,
                    padding: 30,
                    width: Fill,
                    height: Fill,
                    scroll_bars: <ScrollBars> {},

                    <Label> {
                        text: "Native Component Foundation"
                        draw_text: { text_style: { font_size: 24.0 }, color: #fff }
                    }

                    <Label> {
                        text: "Minimal native components wired into Makepad."
                        draw_text: { text_style: { font_size: 14.0 }, color: #aaa }
                    }

                    // Button
                    <View> {
                        flow: Down,
                        spacing: 16,
                        padding: 24,
                        show_bg: true,
                        draw_bg: { color: #333 }

                        <Label> {
                            text: "Native Button"
                            draw_text: { text_style: { font_size: 16.0 }, color: #fff }
                        }

                        button_status = <Label> {
                            text: "Status: Not clicked"
                            draw_text: { text_style: { font_size: 12.0 }, color: #fff }
                        }

                        native_button = <NativeButton> {
                            width: 140,
                            height: 36,
                            label: "Click Me!"
                        }

                        <View> { height: 12 }
                    }

                    // Text field
                    <View> {
                        flow: Down,
                        spacing: 16,
                        padding: 24,
                        show_bg: true,
                        draw_bg: { color: #333 }

                        <Label> {
                            text: "Native Text Field"
                            draw_text: { text_style: { font_size: 16.0 }, color: #fff }
                        }

                        text_status = <Label> {
                            text: "Text: (empty)"
                            draw_text: { text_style: { font_size: 12.0 }, color: #fff }
                        }

                        native_text_field = <NativeTextField> {
                            width: 320,
                            height: 36,
                            placeholder: "Enter text here...",
                            text: ""
                        }

                        <View> { height: 12 }
                    }

                    // Switch
                    <View> {
                        flow: Down,
                        spacing: 16,
                        padding: 24,
                        show_bg: true,
                        draw_bg: { color: #333 }

                        <Label> {
                            text: "Native Switch"
                            draw_text: { text_style: { font_size: 16.0 }, color: #fff }
                        }

                        switch_status = <Label> {
                            text: "State: Off"
                            draw_text: { text_style: { font_size: 12.0 }, color: #fff }
                        }

                        native_switch = <NativeSwitch> {
                            width: 60,
                            height: 32,
                            switch_on: false
                        }

                        <View> { height: 12 }
                    }

                    // Slider + Progress
                    <View> {
                        flow: Down,
                        spacing: 16,
                        padding: 24,
                        show_bg: true,
                        draw_bg: { color: #333 }

                        <Label> {
                            text: "Native Slider"
                            draw_text: { text_style: { font_size: 16.0 }, color: #fff }
                        }

                        slider_status = <Label> {
                            text: "Value: 0.50"
                            draw_text: { text_style: { font_size: 12.0 }, color: #fff }
                        }

                        native_slider = <NativeSlider> {
                            width: 320,
                            height: 32,
                            slider_value: 0.5,
                            slider_min: 0.0,
                            slider_max: 1.0
                        }

                        <View> { height: 12 }

                        <Label> {
                            text: "Native Progress"
                            draw_text: { text_style: { font_size: 16.0 }, color: #fff }
                        }

                        native_progress = <NativeProgress> {
                            width: 320,
                            height: 8,
                            progress: 0.5
                        }

                        progress_status = <Label> {
                            text: "Progress: 50%"
                            draw_text: { text_style: { font_size: 12.0 }, color: #fff }
                        }

                        <View> { height: 6 }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    click_count: usize,
    #[rust]
    slider_value: f64,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_native_components::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        makepad_native_components::init_native_views(cx);
        self.slider_value = 0.5;
        self.update_switch_label(cx, false);
        self.update_button_label(cx, 0);
        self.update_text_label(cx, "");
        self.update_slider_label(cx, self.slider_value);
        self.update_progress_label(cx, self.slider_value);

        self.ui
            .native_slider(ids!(native_slider))
            .set_value(cx, self.slider_value);
        self.ui
            .native_progress(ids!(native_progress))
            .set_progress(cx, self.slider_value);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Button
        let native_button = self.ui.native_button(ids!(native_button));
        if native_button.clicked(actions) {
            self.click_count += 1;
            self.update_button_label(cx, self.click_count);
        }

        // Text field
        let native_text_field = self.ui.native_text_field(ids!(native_text_field));
        if let Some(text) = native_text_field.text_changed(actions) {
            self.update_text_label(cx, &text);
        }

        // Switch
        let native_switch = self.ui.native_switch(ids!(native_switch));
        if let Some(on) = native_switch.switch_changed(actions) {
            self.update_switch_label(cx, on);
        }

        // Slider
        let native_slider = self.ui.native_slider(ids!(native_slider));
        if let Some(value) = native_slider.value_changed(actions) {
            let clamped = value.clamp(0.0, 1.0);
            self.slider_value = clamped;
            self.update_slider_label(cx, clamped);
            self.ui
                .native_progress(ids!(native_progress))
                .set_progress(cx, clamped);
            self.update_progress_label(cx, clamped);
        }
    }
}

impl App {
    fn update_button_label(&mut self, cx: &mut Cx, count: usize) {
        self.ui
            .label(ids!(button_status))
            .set_text(cx, &format!("Status: Clicked {}x", count));
    }

    fn update_text_label(&mut self, cx: &mut Cx, text: &str) {
        let display = if text.is_empty() { "(empty)" } else { text };
        self.ui
            .label(ids!(text_status))
            .set_text(cx, &format!("Text: {}", display));
    }

    fn update_switch_label(&mut self, cx: &mut Cx, on: bool) {
        self.ui
            .label(ids!(switch_status))
            .set_text(cx, &format!("State: {}", if on { "On" } else { "Off" }));
    }

    fn update_slider_label(&mut self, cx: &mut Cx, value: f64) {
        self.ui
            .label(ids!(slider_status))
            .set_text(cx, &format!("Value: {:.2}", value));
    }

    fn update_progress_label(&mut self, cx: &mut Cx, value: f64) {
        self.ui
            .label(ids!(progress_status))
            .set_text(cx, &format!("Progress: {:.0}%", value * 100.0));
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

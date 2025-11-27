use makepad_widgets::*;

live_design! {
    link widgets;
    link shadecn_menubar;
    link shadecn_button;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    link shadecn_core;
    use link::shadecn_core::*;
    use link::shadecn_button::*;

    pub ShadecnMenubar = <RoundedView> {
        width: Fit, height: Fit,
        flow: Right,
        padding: (SPACE_1),
        spacing: (SPACE_1),
        show_bg: true,
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
            border_color: (COLOR_BORDER_PRIMARY)
            border_size: 1.0
            border_radius: (RADIUS_MD)
        }
        
        file = <ShadecnButtonGhost> { text: "File" }
        edit = <ShadecnButtonGhost> { text: "Edit" }
        view = <ShadecnButtonGhost> { text: "View" }
        profile = <ShadecnButtonGhost> { text: "Profile" }
    }
}

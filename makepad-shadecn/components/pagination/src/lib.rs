use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::shadecn_button::*;

    pub ShadecnPagination = <View> {
        width: Fit, height: Fit,
        flow: Right,
        spacing: (SPACE_2),
        align: {y: 0.5},

        prev = <ShadecnButtonGhost> { text: "Previous" }

        page1 = <ShadecnButtonOutline> { text: "1" }
        page2 = <ShadecnButtonGhost> { text: "2" }
        page3 = <ShadecnButtonGhost> { text: "3" }

        next = <ShadecnButtonGhost> { text: "Next" }
    }
}

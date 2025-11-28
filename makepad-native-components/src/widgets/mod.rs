pub mod switch;
pub mod button;
pub mod text_field;
pub mod slider;
pub mod progress;

pub use switch::*;
pub use button::*;
pub use text_field::*;
pub use slider::*;
pub use progress::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    switch::live_design(cx);
    button::live_design(cx);
    text_field::live_design(cx);
    slider::live_design(cx);
    progress::live_design(cx);
}

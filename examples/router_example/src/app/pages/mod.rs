pub mod about;
pub mod admin;
pub mod hero;
pub mod home;
pub mod not_found;
pub mod settings;
pub mod stack_demo;
pub mod user_profile;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    home::live_design(cx);
    settings::live_design(cx);
    about::live_design(cx);
    hero::live_design(cx);
    user_profile::live_design(cx);
    admin::live_design(cx);
    not_found::live_design(cx);
    stack_demo::live_design(cx);
}


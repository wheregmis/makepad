use makepad_router::{NavigationHistory, Route, RouteParams, RoutePattern, RouteQuery, RouterState};
use makepad_router::makepad_live_id::*;
use makepad_router::makepad_micro_serde::{DeRon, SerRon};

#[test]
fn router_state_ron_roundtrip_preserves_history_and_query() {
    let mut params = RouteParams::default();
    params.data.push((live_id!(id), live_id!(user_42)));

    let mut query = RouteQuery::default();
    query.data.push(("tab".to_string(), "settings".to_string()));
    query.data.push(("empty".to_string(), "".to_string()));

    let route = Route {
        id: live_id!(user_profile),
        params,
        query,
        hash: "#section".to_string(),
        pattern: Some(RoutePattern::parse("/user/:id").unwrap()),
    };

    let history = NavigationHistory::from_parts(vec![Route::new(live_id!(home)), route], 1);
    let state = RouterState {
        history,
        url_path_override: Some("/admin/dashboard".to_string()),
    };

    let ron = state.serialize_ron();
    let de = RouterState::deserialize_ron(&ron).unwrap();
    assert_eq!(de, state);
}


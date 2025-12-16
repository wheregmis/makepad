use crate::{navigation::NavigationHistory, route::Route};
use makepad_live_id::*;
use makepad_micro_serde::*;

/// Router configuration and state
#[derive(Clone, Debug, SerBin, DeBin, SerRon, DeRon)]
pub struct Router {
    /// Navigation history
    pub history: NavigationHistory,
    /// Whether to persist router state
    pub persist_state: bool,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            history: NavigationHistory::empty(),
            persist_state: false,
        }
    }
}

impl Router {
    /// Create a new router with an initial route
    pub fn new(initial_route: Route) -> Self {
        Self {
            history: NavigationHistory::new(initial_route),
            persist_state: false,
        }
    }

    /// Create a router with state persistence enabled
    pub fn with_persistence(initial_route: Route) -> Self {
        Self {
            history: NavigationHistory::new(initial_route),
            persist_state: true,
        }
    }

    /// Navigate to a new route
    pub fn navigate(&mut self, route: Route) {
        self.history.push(route);
    }

    /// Navigate to a route by ID
    pub fn navigate_to(&mut self, route_id: LiveId) {
        self.navigate(Route::new(route_id));
    }

    /// Replace the current route
    pub fn replace(&mut self, route: Route) {
        self.history.replace(route);
    }

    /// Replace the current route by ID
    pub fn replace_with(&mut self, route_id: LiveId) {
        self.replace(Route::new(route_id));
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        self.history.back()
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        self.history.forward()
    }

    /// Get the current route
    pub fn current_route(&self) -> Option<&Route> {
        self.history.current()
    }

    /// Get the current route ID
    pub fn current_route_id(&self) -> Option<LiveId> {
        self.current_route().map(|r| r.id)
    }

    /// Check if we can navigate back
    pub fn can_go_back(&self) -> bool {
        self.history.can_go_back()
    }

    /// Check if we can navigate forward
    pub fn can_go_forward(&self) -> bool {
        self.history.can_go_forward()
    }

    /// Reset to a specific route, clearing all history
    pub fn reset(&mut self, route: Route) {
        self.history.reset(route);
    }

    /// Clear all history except current route
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get the depth of the navigation history
    pub fn depth(&self) -> usize {
        self.history.depth()
    }
}

/// Router actions for event handling
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RouterAction {
    /// Navigate to a route
    Navigate(Route),
    /// Replace the current route
    Replace(Route),
    /// Go back in history
    Back,
    /// Go forward in history
    Forward,
    /// Reset to a route
    Reset(Route),
    /// Route changed notification
    RouteChanged { from: Option<LiveId>, to: LiveId },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_navigate() {
        let mut router = Router::new(Route::new(live_id!(home)));
        assert_eq!(router.current_route_id(), Some(live_id!(home)));

        router.navigate_to(live_id!(settings));
        assert_eq!(router.current_route_id(), Some(live_id!(settings)));
        assert_eq!(router.depth(), 2);
    }

    #[test]
    fn test_router_back() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.navigate_to(live_id!(settings));
        router.navigate_to(live_id!(profile));

        assert!(router.can_go_back());
        assert!(router.back());
        assert_eq!(router.current_route_id(), Some(live_id!(settings)));

        assert!(router.back());
        assert_eq!(router.current_route_id(), Some(live_id!(home)));
        assert!(!router.can_go_back());
    }

    #[test]
    fn test_router_replace() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.replace_with(live_id!(settings));

        assert_eq!(router.current_route_id(), Some(live_id!(settings)));
        assert_eq!(router.depth(), 1);
        assert!(!router.can_go_back());
    }
}

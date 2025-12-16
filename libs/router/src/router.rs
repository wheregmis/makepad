use crate::{navigation::NavigationHistory, route::{Route, RoutePattern}};
use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;

/// Route registry entry
#[derive(Clone, Debug)]
struct RouteEntry {
    route_id: LiveId,
    pattern: Option<RoutePattern>,
    priority: usize,
}

/// Registry for pattern-based routes
#[derive(Clone, Debug, Default)]
pub struct RouteRegistry {
    /// Routes by LiveId (for exact matches)
    by_id: HashMap<LiveId, RouteEntry>,
    /// Routes by pattern (for path-based matching)
    by_pattern: Vec<RouteEntry>,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_pattern: Vec::new(),
        }
    }

    /// Register a route by LiveId
    pub fn register_by_id(&mut self, route_id: LiveId) {
        let entry = RouteEntry {
            route_id,
            pattern: None,
            priority: 0, // Highest priority
        };
        self.by_id.insert(route_id, entry);
    }

    /// Register a route pattern
    pub fn register_pattern(&mut self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        let route_pattern = RoutePattern::parse(pattern)?;
        let priority = route_pattern.priority();
        let entry = RouteEntry {
            route_id,
            pattern: Some(route_pattern),
            priority,
        };
        
        // Insert in sorted order by priority (lower priority value = higher priority)
        // Find insertion point
        let pos = self.by_pattern.iter()
            .position(|e| e.priority > priority)
            .unwrap_or(self.by_pattern.len());
        self.by_pattern.insert(pos, entry);
        Ok(())
    }

    /// Resolve a path to a route
    pub fn resolve_path(&self, path: &str) -> Option<Route> {
        // First try exact LiveId match if path is a single identifier
        // For now, we'll skip this and go straight to pattern matching
        
        // Try pattern matching
        for entry in &self.by_pattern {
            if let Some(ref pattern) = entry.pattern {
                if let Some(params) = pattern.matches(path) {
                    return Some(Route {
                        id: entry.route_id,
                        params,
                        pattern: Some(pattern.clone()),
                    });
                }
            }
        }
        
        None
    }

    /// Check if a route ID is registered
    pub fn has_route(&self, route_id: LiveId) -> bool {
        self.by_id.contains_key(&route_id)
    }

    /// Get pattern for a route ID
    pub fn get_pattern(&self, route_id: LiveId) -> Option<&RoutePattern> {
        self.by_pattern.iter()
            .find(|e| e.route_id == route_id)
            .and_then(|e| e.pattern.as_ref())
    }
}

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

    #[test]
    fn test_route_registry_register_pattern() {
        let mut registry = RouteRegistry::new();
        registry.register_pattern("/user/:id", live_id!(user_profile)).unwrap();
        assert!(registry.has_route(live_id!(user_profile)));
    }

    #[test]
    fn test_route_registry_resolve_path() {
        let mut registry = RouteRegistry::new();
        registry.register_pattern("/user/:id", live_id!(user_profile)).unwrap();
        
        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert_eq!(route.get_param(LiveId::from_str("id")), Some(LiveId::from_str("123")));
    }

    #[test]
    fn test_route_registry_priority() {
        let mut registry = RouteRegistry::new();
        // Register in reverse priority order
        registry.register_pattern("/user/**", live_id!(user_wildcard)).unwrap();
        registry.register_pattern("/user/*", live_id!(user_single)).unwrap();
        registry.register_pattern("/user/:id", live_id!(user_dynamic)).unwrap();
        registry.register_pattern("/user/profile", live_id!(user_static)).unwrap();

        // Most specific should match first
        let route = registry.resolve_path("/user/profile").unwrap();
        assert_eq!(route.id, live_id!(user_static));

        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_dynamic));

        let route = registry.resolve_path("/user/other").unwrap();
        assert_eq!(route.id, live_id!(user_single));

        let route = registry.resolve_path("/user/123/posts").unwrap();
        assert_eq!(route.id, live_id!(user_wildcard));
    }

    #[test]
    fn test_router_navigate_by_path() {
        let mut router = Router::new(Route::new(live_id!(home)));
        router.register_route_pattern("/user/:id", live_id!(user_profile)).unwrap();
        
        router.navigate_by_path("/user/123").unwrap();
        let route = router.current_route().unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert_eq!(route.get_param(LiveId::from_str("id")), Some(LiveId::from_str("123")));
    }
}

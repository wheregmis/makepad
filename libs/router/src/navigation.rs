use crate::route::Route;
use makepad_micro_serde::*;

/// Navigation history stack for managing route navigation
#[derive(Clone, Debug, Default, SerBin, DeBin, SerRon, DeRon)]
pub struct NavigationHistory {
    /// Stack of routes representing navigation history
    stack: Vec<Route>,
    /// Current position in the history (for back/forward navigation)
    current_index: usize,
}

impl NavigationHistory {
    /// Create a new navigation history with an initial route
    pub fn new(initial_route: Route) -> Self {
        Self {
            stack: vec![initial_route],
            current_index: 0,
        }
    }

    /// Create an empty navigation history
    pub fn empty() -> Self {
        Self {
            stack: Vec::new(),
            current_index: 0,
        }
    }

    /// Get the current route
    pub fn current(&self) -> Option<&Route> {
        self.stack.get(self.current_index)
    }

    /// Push a new route onto the history
    pub fn push(&mut self, route: Route) {
        // Remove any forward history when pushing a new route
        self.stack.truncate(self.current_index + 1);
        self.stack.push(route);
        self.current_index = self.stack.len() - 1;
    }

    /// Replace the current route without adding to history
    pub fn replace(&mut self, route: Route) {
        if !self.stack.is_empty() {
            self.stack[self.current_index] = route;
        } else {
            self.stack.push(route);
            self.current_index = 0;
        }
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        if self.can_go_back() {
            self.current_index -= 1;
            true
        } else {
            false
        }
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        if self.can_go_forward() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Check if we can go forward
    pub fn can_go_forward(&self) -> bool {
        self.current_index < self.stack.len().saturating_sub(1)
    }

    /// Get the depth of the history stack
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Clear all history except the current route
    pub fn clear(&mut self) {
        if let Some(current) = self.current().cloned() {
            self.stack = vec![current];
            self.current_index = 0;
        } else {
            self.stack.clear();
            self.current_index = 0;
        }
    }

    /// Reset to a specific route, clearing all history
    pub fn reset(&mut self, route: Route) {
        self.stack = vec![route];
        self.current_index = 0;
    }

    /// Get all routes in the stack
    pub fn all_routes(&self) -> &[Route] {
        &self.stack
    }
}

#[cfg(test)]
mod tests {
    use makepad_live_id::live_id;

    use super::*;

    #[test]
    fn test_navigation_push() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        assert_eq!(history.current().unwrap().id, live_id!(home));

        history.push(Route::new(live_id!(settings)));
        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 2);
    }

    #[test]
    fn test_navigation_back() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.push(Route::new(live_id!(settings)));

        assert!(history.can_go_back());
        assert!(history.back());
        assert_eq!(history.current().unwrap().id, live_id!(home));
        assert!(!history.can_go_back());
    }

    #[test]
    fn test_navigation_replace() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.replace(Route::new(live_id!(settings)));

        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 1);
        assert!(!history.can_go_back());
    }
}

use crate::route::Route;
use makepad_live_id::*;
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

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn into_parts(self) -> (Vec<Route>, usize) {
        (self.stack, self.current_index)
    }

    pub fn from_parts(stack: Vec<Route>, current_index: usize) -> Self {
        if stack.is_empty() {
            return Self::empty();
        }
        let current_index = current_index.min(stack.len().saturating_sub(1));
        Self { stack, current_index }
    }

    /// Sets the entire stack (stack-style semantics).
    ///
    /// - If `stack` is empty, the history becomes empty.
    /// - If `stack` is non-empty, the current route becomes the last element.
    pub fn set_stack(&mut self, stack: Vec<Route>) {
        if stack.is_empty() {
            self.stack.clear();
            self.current_index = 0;
            return;
        }
        self.stack = stack;
        self.current_index = self.stack.len() - 1;
    }

    /// Pops the current route (stack-style semantics).
    ///
    /// Unlike `back()`, this removes the current route from the stack and does not keep forward history.
    pub fn pop(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return false;
        }
        self.stack.pop();
        self.current_index = self.stack.len() - 1;
        true
    }

    /// Pops routes until `route_id` is the current route (stack-style semantics).
    ///
    /// Returns `false` if `route_id` does not exist in the stack or it is already the current route.
    pub fn pop_to(&mut self, route_id: LiveId) -> bool {
        let current = self.current().map(|r| r.id);
        if current == Some(route_id) {
            return false;
        }
        let Some(pos) = self.stack.iter().rposition(|r| r.id == route_id) else {
            return false;
        };
        self.stack.truncate(pos + 1);
        self.current_index = pos;
        true
    }

    /// Pops to the root route (stack-style semantics).
    pub fn pop_to_root(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return false;
        }
        self.stack.truncate(1);
        self.current_index = 0;
        true
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

    #[test]
    fn test_stack_pop() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.push(Route::new(live_id!(settings)));
        history.push(Route::new(live_id!(profile)));

        assert!(history.pop());
        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 2);
        assert!(!history.can_go_forward());
    }

    #[test]
    fn test_stack_pop_to() {
        let mut history = NavigationHistory::new(Route::new(live_id!(home)));
        history.push(Route::new(live_id!(settings)));
        history.push(Route::new(live_id!(profile)));

        assert!(history.pop_to(live_id!(home)));
        assert_eq!(history.current().unwrap().id, live_id!(home));
        assert_eq!(history.depth(), 1);
    }

    #[test]
    fn test_stack_set_stack() {
        let mut history = NavigationHistory::empty();
        history.set_stack(vec![Route::new(live_id!(home)), Route::new(live_id!(settings))]);
        assert_eq!(history.current().unwrap().id, live_id!(settings));
        assert_eq!(history.depth(), 2);
        assert!(!history.can_go_forward());
    }
}

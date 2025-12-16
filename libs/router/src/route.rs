use makepad_live_id::*;
use makepad_micro_serde::*;

/// Represents a route in the application
#[derive(Clone, Debug, PartialEq, Eq, Hash, SerBin, DeBin, SerRon, DeRon)]
pub struct Route {
    /// The unique identifier for this route
    pub id: LiveId,
    /// Optional parameters for the route
    pub params: RouteParams,
}

/// Route parameters - can be extended with typed parameters in the future
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, SerBin, DeBin, SerRon, DeRon)]
pub struct RouteParams {
    /// Generic parameters stored as LiveId key-value pairs
    pub data: Vec<(LiveId, LiveId)>,
}

impl Route {
    /// Create a new route with the given ID
    pub fn new(id: LiveId) -> Self {
        Self {
            id,
            params: RouteParams::default(),
        }
    }

    /// Create a new route with parameters
    pub fn with_params(id: LiveId, params: RouteParams) -> Self {
        Self { id, params }
    }

    /// Add a parameter to the route
    pub fn param(mut self, key: LiveId, value: LiveId) -> Self {
        self.params.data.push((key, value));
        self
    }

    /// Get a parameter value by key
    pub fn get_param(&self, key: LiveId) -> Option<LiveId> {
        self.params
            .data
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
    }
}

impl RouteParams {
    /// Create empty route parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a parameter
    pub fn add(&mut self, key: LiveId, value: LiveId) {
        self.data.push((key, value));
    }

    /// Get a parameter value by key
    pub fn get(&self, key: LiveId) -> Option<LiveId> {
        self.data.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
    }
}

/// Macro to create routes easily
#[macro_export]
macro_rules! route {
    ($id:ident) => {
        Route::new(live_id!($id))
    };
    ($id:ident, $($key:ident = $value:ident),+) => {
        {
            let mut route = Route::new(live_id!($id));
            $(
                route = route.param(live_id!($key), live_id!($value));
            )+
            route
        }
    };
}

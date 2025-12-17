#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouterUrl {
    pub path: String,
    pub query: String,
    pub hash: String,
}

impl RouterUrl {
    pub fn parse(input: &str) -> Self {
        let mut s = input.trim().to_string();
        if s.is_empty() {
            return Self {
                path: "/".to_string(),
                query: String::new(),
                hash: String::new(),
            };
        }

        // Accept full URLs (e.g. https://host/path?query#hash) and path-only inputs.
        if let Some((_, after_scheme)) = s.split_once("://") {
            let mut rest = after_scheme;
            if let Some((_, after_host_slash)) = rest.split_once('/') {
                rest = after_host_slash;
                s = format!("/{}", rest);
            } else {
                s = "/".to_string();
            }
        }

        let s_trim = s.trim();
        let (before_hash, hash) = match s_trim.split_once('#') {
            Some((a, b)) => (a, format!("#{}", b)),
            None => (s_trim, String::new()),
        };
        let (path, query) = match before_hash.split_once('?') {
            Some((a, b)) => (a, format!("?{}", b)),
            None => (before_hash, String::new()),
        };

        let mut path = path.trim().to_string();
        if path.is_empty() {
            path = "/".to_string();
        } else if !path.starts_with('/') {
            path.insert(0, '/');
        }

        Self { path, query, hash }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.path, self.query, self.hash)
    }
}

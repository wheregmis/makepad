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

    pub fn parse_query_map(&self) -> Vec<(String, String)> {
        parse_query_map(&self.query)
    }
}

pub fn parse_query_map(query: &str) -> Vec<(String, String)> {
    let q = query.trim();
    let q = q.strip_prefix('?').unwrap_or(q);
    if q.is_empty() {
        return Vec::new();
    }
    q.split('&')
        .filter_map(|pair| {
            if pair.is_empty() {
                return None;
            }
            let (k, v) = match pair.split_once('=') {
                Some((k, v)) => (k, v),
                None => (pair, ""),
            };
            let key = decode_www_form_component(k);
            if key.is_empty() {
                return None;
            }
            let val = decode_www_form_component(v);
            Some((key, val))
        })
        .collect()
}

pub fn build_query_string(map: &[(String, String)]) -> String {
    if map.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push('?');
    for (i, (k, v)) in map.iter().enumerate() {
        if i > 0 {
            out.push('&');
        }
        out.push_str(&encode_www_form_component(k));
        if !v.is_empty() {
            out.push('=');
            out.push_str(&encode_www_form_component(v));
        }
    }
    out
}

fn decode_www_form_component(input: &str) -> String {
    let mut bytes = Vec::<u8>::with_capacity(input.len());
    let mut iter = input.as_bytes().iter().copied().peekable();
    while let Some(b) = iter.next() {
        match b {
            b'+' => bytes.push(b' '),
            b'%' => {
                let hi = iter.next();
                let lo = iter.next();
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    if let (Some(hi), Some(lo)) = (hex_val(hi), hex_val(lo)) {
                        bytes.push((hi << 4) | lo);
                    }
                }
            }
            _ => bytes.push(b),
        }
    }
    String::from_utf8(bytes).unwrap_or_else(|_| input.to_string())
}

fn encode_www_form_component(input: &str) -> String {
    let mut out = String::new();
    for &b in input.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => {
                out.push('%');
                out.push(hex_char((b >> 4) & 0x0f));
                out.push(hex_char(b & 0x0f));
            }
        }
    }
    out
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn hex_char(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'A' + (n - 10)) as char,
        _ => '0',
    }
}

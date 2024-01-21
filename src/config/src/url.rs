pub const SERVER_URL_BASE: &'static str = if cfg!(test) {
    "http://localhost"
} else {
    "https://ganbeach.com"
};

pub const SERVER_URL_ACCOUNT: &'static str = if cfg!(test) {
    "http://localhost:8080"
} else {
    "https://user.ganbeach.com"
};

pub const SERVER_URL_BOOKMARK: &'static str = if cfg!(test) {
    "http://localhost:8080"
} else {
    "https://bookmark.ganbeach.com"
};

pub const SERVER_URL_FILE: &'static str = if cfg!(test) {
    "http://localhost:8080"
} else {
    "https://file.ganbeach.com"
};

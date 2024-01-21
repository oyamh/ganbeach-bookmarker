#[derive(Debug, Default, Clone, Copy)]
pub enum HotKey {
    OpenPopup,
    #[default]
    Unassigned,
}

impl From<String> for HotKey {
    fn from(src: String) -> Self {
        match src.as_str() {
            "open-popup" => Self::OpenPopup,
            _ => Self::Unassigned,
        }
    }
}

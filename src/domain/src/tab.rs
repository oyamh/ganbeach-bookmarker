pub struct TabId(i32);

impl From<i32> for TabId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

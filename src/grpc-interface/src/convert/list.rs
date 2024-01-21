use crate::gooscut::List as PbList;
use domain::{List, ListBuilder};

impl From<PbList> for List {
    fn from(value: PbList) -> List {
        let PbList {
            user_id,
            bookmark_id,
            parent_id,
            type_code,
            child_count,
            position,
            created_at,
            updated_at,
            last_visit,
            title,
        } = value;

        ListBuilder::new()
            .set_user_id(user_id)
            .set_bookmark_id(bookmark_id)
            .set_parent_id(parent_id)
            .set_type_code(type_code)
            .set_child_count(child_count)
            .set_position(position)
            .set_created_at(created_at)
            .set_updated_at(updated_at)
            .set_last_visit(last_visit)
            .set_title(title)
            .build()
    }
}

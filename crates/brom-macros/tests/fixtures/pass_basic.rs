use brom_macros::BromEntity;
use brom_core::EntitySchema;

#[derive(BromEntity)]
pub struct Post {
    pub id: i64,
    pub title: String,
}

fn main() {
    assert_eq!(Post::table_name(), "post");
    let fields = Post::fields();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "id");
    assert_eq!(fields[1].name, "title");
}

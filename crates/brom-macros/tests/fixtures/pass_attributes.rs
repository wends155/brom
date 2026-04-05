use brom_macros::BromEntity;
use brom_core::{EntitySchema, Constraint};
use serde::{Deserialize, Serialize};
pub use brom_server::utoipa::ToSchema;

#[derive(BromEntity, Serialize, Deserialize, ToSchema)]
#[brom(table = "users")]
pub struct User {
    #[brom(unique)]
    pub username: String,
    #[brom(not_null, default = "Unknown")]
    pub display_name: String,
    #[brom(hidden)]
    pub password_hash: String,
}

fn main() {
    assert_eq!(User::table_name(), "users");
    let fields = User::fields();
    assert_eq!(fields.len(), 3);

    // username
    assert_eq!(fields[0].name, "username");
    assert!(fields[0].constraints.contains(&Constraint::Unique));

    // display_name
    assert_eq!(fields[1].name, "display_name");
    assert!(fields[1].constraints.contains(&Constraint::NotNull));
    assert!(fields[1].constraints.contains(&Constraint::Default("Unknown".to_string())));

    // password_hash
    assert_eq!(fields[2].name, "password_hash");
    assert!(fields[2].hidden);
}

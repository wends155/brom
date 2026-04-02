use brom_macros::BromEntity;

#[derive(BromEntity)]
#[brom(unknown_struct_attr = "oops")]
pub struct Broken {
    #[brom(invalid_field_attr)]
    pub field: String,
    #[brom(table = 123)]
    pub bad_type: String,
}

fn main() {}

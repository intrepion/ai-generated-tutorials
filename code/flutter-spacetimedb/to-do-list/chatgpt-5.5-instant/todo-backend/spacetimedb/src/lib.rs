use spacetimedb::{table, reducer, Identity};

#[table(name = todo)]
pub struct Todo {
    #[primary_key]
    pub id: u64,

    pub title: String,

    pub description: String,

    pub completed: bool,

    pub owner: Identity,
}

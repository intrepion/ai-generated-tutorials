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

#[reducer]
pub fn create_todo(
    ctx: &ReducerContext,
    title: String,
    description: String,
) {
    let id = ctx.db.todo().count() as u64 + 1;

    ctx.db.todo().insert(Todo {
        id,
        title,
        description,
        completed: false,
        owner: ctx.sender,
    });
}

use ::init_client;

use kickstarter::models::User;
use kickstarter::db::{column, table};

use rustorm::query::{Equality, Query};
use rustorm::dao::FromValue;

#[test]
fn upsert_user() {
    let client = init_client();

    // Ensure user insert
    let name = "Snickerdoodles";
    let id_val = User::upsert(&client, name).unwrap();
    let id = FromValue::from_type(id_val.clone());

    let user: User = Query::select_all()
        .from_table(&client.table(table::user))
        .filter(column::user_id, Equality::EQ, &id_val)
        .collect_one(client.db())
        .unwrap();

    assert_eq!(user.user_id, id);
    assert_eq!(user.name, name);

    // Ensure user select
    let existing_id = User::upsert(&client, name).unwrap();
    assert_eq!(id_val, existing_id);

    // Ensure no extra users were inserted
    let users: Vec<User> = Query::select_all()
        .from_table(&client.table(table::user))
        .collect(client.db())
        .unwrap();

    assert_eq!(1, users.len());
}

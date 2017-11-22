use ::{init_test_projects, init_test_pledges, 
       NAMES, USERS, CARDS, CONTRIBUTIONS, NUM_PLEDGES};

use kickstarter::Error;
use kickstarter::models::Pledge;
use kickstarter::db::table;

use postgres::error::SqlState;

#[test]
fn create_pledge() {
    let (client, _) = init_test_projects();
    let pledges = init_test_pledges(&client);

    // Check that all Pledge::create calls returned the right information.
    for i in (0..NUM_PLEDGES) {
        let ref pledge = pledges[i];
        assert_eq!(CARDS[i], pledge.card);
        assert_eq!(CONTRIBUTIONS[i], pledge.amount);
    }

    // Query and cross-check with the expected information.
    let found_pledges: Vec<Pledge> = Query::select_all()
        .from_table(&client.table(table::pledge))
        .collect(client.db())
        .unwrap();

    assert_eq!(NUM_PLEDGES, found_pledges.len());
}

#[test]
fn pledge_once_user() {
    let (client, _) = init_test_projects();
    let _ = init_test_pledges(&client);

    let result = Pledge::create(&client, USERS[0], NAMES[0], "4298708533045499", CONTRIBUTIONS[0]);
    assert!(result.is_err());

    if let Err(Error::Database(ref err)) = result {
        assert_eq!(Some(SqlState::UniqueViolation), err.code);
    }
}

#[test]
fn pledge_once_card() {
    let (client, _) = init_test_projects();
    let _ = init_test_pledges(&client);

    let result = Pledge::create(&client, "Charlie_Chaplin", NAMES[0], CARDS[0], CONTRIBUTIONS[0]);
    assert!(result.is_err());

    if let Err(Error::Database(ref err)) = result {
        assert_eq!(Some(SqlState::UniqueViolation), err.code);
    }    
}

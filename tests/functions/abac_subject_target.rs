use diesel::{self, prelude::*};
use uuid::Uuid;

use abac::functions::abac_subject_target;
use abac::AbacAttribute;

use shared;

#[test]
fn test() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let outbound = AbacAttribute {
        namespace_id: ns_id,
        key: "uri".to_owned(),
        value: "account/1".to_owned(),
    };
    let mut list = diesel::select(abac_subject_target(vec![outbound.clone()]))
        .get_results::<AbacAttribute>(&conn)
        .unwrap();

    let index = list.iter().position(|x| *x == outbound).unwrap();
    list.remove(index);

    let index = list
        .iter()
        .position(|x| {
            *x == AbacAttribute {
                namespace_id: ns_id,
                key: "role".to_owned(),
                value: "user".to_owned(),
            }
        })
        .unwrap();
    list.remove(index);

    assert!(list.is_empty());
}

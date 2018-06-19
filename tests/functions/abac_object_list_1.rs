use diesel::{self, prelude::*};
use uuid::Uuid;

use abac::functions::abac_object_list_1;
use abac::types::AbacAttribute;

use shared;

#[test]
fn test_abac_object_list_1() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let outbound = AbacAttribute {
        namespace_id: ns_id,
        key: "type".to_owned(),
        value: "fruit".to_owned(),
    };
    let list = diesel::select(abac_object_list_1(outbound, 0, 10))
        .get_results::<AbacAttribute>(&conn)
        .unwrap();

    let expected = vec![
        AbacAttribute {
            namespace_id: ns_id,
            key: "type".to_owned(),
            value: "apple".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "type".to_owned(),
            value: "pear".to_owned(),
        },
    ];

    assert_eq!(list, expected);
}

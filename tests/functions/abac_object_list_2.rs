use diesel::{self, prelude::*};
use uuid::Uuid;

use abac::functions::abac_object_list_2;
use abac::types::AbacAttribute;

use shared;

#[test]
fn test_abac_object_list_2() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let attr_1 = AbacAttribute {
        namespace_id: ns_id,
        key: "type".to_owned(),
        value: "apple".to_owned(),
    };
    let attr_2 = AbacAttribute {
        namespace_id: ns_id,
        key: "color".to_owned(),
        value: "red".to_owned(),
    };
    let list = diesel::select(abac_object_list_2(attr_1, attr_2, 0, 10))
        .get_results::<AbacAttribute>(&conn)
        .unwrap();

    let expected = vec![
        AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "object/4".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "object/5".to_owned(),
        },
    ];

    assert_eq!(list, expected);
}

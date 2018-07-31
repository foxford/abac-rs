use diesel::{self, prelude::*};
use uuid::Uuid;

use abac::functions::abac_authorize;
use abac::AbacAttribute;

use shared;

#[test]
fn test() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let granted: bool = diesel::select(abac_authorize(
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "account/1".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "object/1".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "operation".to_owned(),
            value: "read".to_owned(),
        }],
        vec![ns_id],
    )).get_result(&conn)
        .unwrap();

    assert!(granted);

    let granted: bool = diesel::select(abac_authorize(
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "account/1".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "object/1".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "operation".to_owned(),
            value: "update".to_owned(),
        }],
        vec![ns_id],
    )).get_result(&conn)
        .unwrap();

    assert!(!granted);

    let granted: bool = diesel::select(abac_authorize(
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "account/1".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "object/3".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "operation".to_owned(),
            value: "update".to_owned(),
        }],
        vec![ns_id],
    )).get_result(&conn)
        .unwrap();

    assert!(granted);

    let granted: bool = diesel::select(abac_authorize(
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "account/1".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "uri".to_owned(),
            value: "object/4".to_owned(),
        }],
        vec![AbacAttribute {
            namespace_id: ns_id,
            key: "operation".to_owned(),
            value: "update".to_owned(),
        }],
        vec![ns_id],
    )).get_result(&conn)
        .unwrap();

    assert!(!granted);
}

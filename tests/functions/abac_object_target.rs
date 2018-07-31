use diesel::{self, prelude::*};
use uuid::Uuid;

use abac::functions::abac_object_target;
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
        value: "object/2".to_owned(),
    };
    let mut list = diesel::select(abac_object_target(vec![outbound.clone()]))
        .get_results::<AbacAttribute>(&conn)
        .unwrap();

    let index = list.iter().position(|x| *x == outbound).unwrap();
    list.remove(index);

    let index = list
        .iter()
        .position(|x| {
            *x == AbacAttribute {
                namespace_id: ns_id,
                key: "type".to_owned(),
                value: "fruit".to_owned(),
            }
        })
        .unwrap();
    list.remove(index);

    let index = list
        .iter()
        .position(|x| {
            *x == AbacAttribute {
                namespace_id: ns_id,
                key: "type".to_owned(),
                value: "pear".to_owned(),
            }
        })
        .unwrap();
    list.remove(index);

    let index = list
        .iter()
        .position(|x| {
            *x == AbacAttribute {
                namespace_id: ns_id,
                key: "color".to_owned(),
                value: "light".to_owned(),
            }
        })
        .unwrap();
    list.remove(index);

    let index = list
        .iter()
        .position(|x| {
            *x == AbacAttribute {
                namespace_id: ns_id,
                key: "color".to_owned(),
                value: "yellow".to_owned(),
            }
        })
        .unwrap();
    list.remove(index);

    let index = list
        .iter()
        .position(|x| {
            *x == AbacAttribute {
                namespace_id: ns_id,
                key: "taste".to_owned(),
                value: "sweet".to_owned(),
            }
        })
        .unwrap();
    list.remove(index);

    assert!(list.is_empty());
}

use diesel::{self, prelude::*};
use uuid::Uuid;

use abac::functions::abac_object_list;
use abac::AbacAttribute;

use shared;

#[test]
fn without_attrs() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let attrs: Vec<AbacAttribute> = vec![];
    let err = diesel::select(abac_object_list(attrs, 0, 10))
        .get_results::<AbacAttribute>(&conn)
        .unwrap_err();

    use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
    match err {
        DatabaseError(DatabaseErrorKind::__Unknown, info) => {
            assert_eq!(info.message(), "bad argument");
            assert_eq!(
                info.details(),
                Some("length of _attrs array shoud be less or equal to 3")
            );
        }
        _ => assert!(false),
    }
}

#[test]
fn with_one_attr() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let attrs = vec![AbacAttribute {
        namespace_id: ns_id,
        key: "type".to_owned(),
        value: "fruit".to_owned(),
    }];
    let list = diesel::select(abac_object_list(attrs, 0, 10))
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

#[test]
fn with_two_attrs() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let attrs = vec![
        AbacAttribute {
            namespace_id: ns_id,
            key: "type".to_owned(),
            value: "apple".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "color".to_owned(),
            value: "red".to_owned(),
        },
    ];
    let list = diesel::select(abac_object_list(attrs, 0, 10))
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

#[test]
fn with_three_attrs() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let attrs = vec![
        AbacAttribute {
            namespace_id: ns_id,
            key: "type".to_owned(),
            value: "apple".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "color".to_owned(),
            value: "red".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "taste".to_owned(),
            value: "sour".to_owned(),
        },
    ];
    let list = diesel::select(abac_object_list(attrs, 0, 10))
        .get_results::<AbacAttribute>(&conn)
        .unwrap();

    let expected = vec![AbacAttribute {
        namespace_id: ns_id,
        key: "uri".to_owned(),
        value: "object/5".to_owned(),
    }];

    assert_eq!(list, expected);
}

#[test]
fn with_more_attrs() {
    let conn = shared::establish_connection();
    conn.begin_test_transaction().unwrap();

    let ns_id = Uuid::new_v4();
    shared::prepare_data(&conn, ns_id);

    let attrs = vec![
        AbacAttribute {
            namespace_id: ns_id,
            key: "type".to_owned(),
            value: "apple".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "color".to_owned(),
            value: "red".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "taste".to_owned(),
            value: "sour".to_owned(),
        },
        AbacAttribute {
            namespace_id: ns_id,
            key: "foo".to_owned(),
            value: "bar".to_owned(),
        },
    ];
    let err = diesel::select(abac_object_list(attrs, 0, 10))
        .get_results::<AbacAttribute>(&conn)
        .unwrap_err();

    use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
    match err {
        DatabaseError(DatabaseErrorKind::__Unknown, info) => {
            assert_eq!(info.message(), "bad argument");
            assert_eq!(
                info.details(),
                Some("length of _attrs array shoud be less or equal to 3")
            );
        }
        _ => assert!(false),
    }
}

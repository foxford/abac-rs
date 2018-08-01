use diesel::{self, prelude::*};
use uuid::Uuid;

use abac::models::prelude::*;
use abac::schema::*;
use abac::AbacAttribute;

pub fn establish_connection() -> PgConnection {
    let database_url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn prepare_data(conn: &PgConnection, ns_id: Uuid) {
    diesel::insert_into(abac_subject::table)
        .values(vec![NewAbacSubject {
            inbound: AbacAttribute {
                namespace_id: ns_id,
                key: "uri".to_owned(),
                value: "account/1".to_owned(),
            },
            outbound: AbacAttribute {
                namespace_id: ns_id,
                key: "role".to_owned(),
                value: "user".to_owned(),
            },
        }])
        .execute(conn)
        .unwrap();

    diesel::insert_into(abac_object::table)
        .values(vec![
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "apple".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "fruit".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "pear".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "fruit".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "green".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "light".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "read".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "light".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "yellow".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "light".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/1".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "pear".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/1".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "green".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/1".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "taste".to_owned(),
                    value: "sweet".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/2".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "pear".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/2".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "yellow".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/2".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "taste".to_owned(),
                    value: "sweet".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/3".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "apple".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/3".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "green".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/3".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "taste".to_owned(),
                    value: "sour".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/4".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "apple".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/4".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "red".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/4".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "taste".to_owned(),
                    value: "sweet".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/5".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "apple".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/5".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "color".to_owned(),
                    value: "red".to_owned(),
                },
            },
            NewAbacObject {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "uri".to_owned(),
                    value: "object/5".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "taste".to_owned(),
                    value: "sour".to_owned(),
                },
            },
        ])
        .execute(conn)
        .unwrap();

    diesel::insert_into(abac_action::table)
        .values(vec![
            NewAbacAction {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "create".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "any".to_owned(),
                },
            },
            NewAbacAction {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "read".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "any".to_owned(),
                },
            },
            NewAbacAction {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "update".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "any".to_owned(),
                },
            },
            NewAbacAction {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "delete".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "any".to_owned(),
                },
            },
            NewAbacAction {
                inbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "list".to_owned(),
                },
                outbound: AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "any".to_owned(),
                },
            },
        ])
        .execute(conn)
        .unwrap();

    diesel::insert_into(abac_policy::table)
        .values(vec![
            NewAbacPolicy {
                subject: vec![AbacAttribute {
                    namespace_id: ns_id,
                    key: "role".to_owned(),
                    value: "user".to_owned(),
                }],
                object: vec![AbacAttribute {
                    namespace_id: ns_id,
                    key: "type".to_owned(),
                    value: "fruit".to_owned(),
                }],
                action: vec![AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "read".to_owned(),
                }],
                namespace_id: ns_id,
            },
            NewAbacPolicy {
                subject: vec![AbacAttribute {
                    namespace_id: ns_id,
                    key: "role".to_owned(),
                    value: "user".to_owned(),
                }],
                object: vec![
                    AbacAttribute {
                        namespace_id: ns_id,
                        key: "type".to_owned(),
                        value: "apple".to_owned(),
                    },
                    AbacAttribute {
                        namespace_id: ns_id,
                        key: "color".to_owned(),
                        value: "green".to_owned(),
                    },
                ],
                action: vec![AbacAttribute {
                    namespace_id: ns_id,
                    key: "operation".to_owned(),
                    value: "any".to_owned(),
                }],
                namespace_id: ns_id,
            },
        ])
        .execute(conn)
        .unwrap();
}

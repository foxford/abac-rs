table! {
    use diesel::sql_types::*;
    use sql_types::AbacAttribute;

    abac_action (inbound, outbound) {
        inbound -> AbacAttribute,
        outbound -> AbacAttribute,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use sql_types::AbacAttribute;

    abac_object (inbound, outbound) {
        inbound -> AbacAttribute,
        outbound -> AbacAttribute,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::{Array, Uuid};
    use sql_types::AbacAttribute;

    abac_policy (subject, object, action, namespace_id) {
        subject -> Array<AbacAttribute>,
        object -> Array<AbacAttribute>,
        action -> Array<AbacAttribute>,
        namespace_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use sql_types::AbacAttribute;

    abac_subject (inbound, outbound) {
        inbound -> AbacAttribute,
        outbound -> AbacAttribute,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(abac_action, abac_object, abac_policy, abac_subject,);

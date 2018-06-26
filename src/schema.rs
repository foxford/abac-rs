table! {
    use types::AbacAttributeSqlType;

    abac_action (inbound, outbound) {
        inbound -> AbacAttributeSqlType,
        outbound -> AbacAttributeSqlType,
    }
}

table! {
    use types::AbacAttributeSqlType;

    abac_object (inbound, outbound) {
        inbound -> AbacAttributeSqlType,
        outbound -> AbacAttributeSqlType,
    }
}

table! {
    use diesel::sql_types::{Array, Uuid};
    use types::AbacAttributeSqlType;

    abac_policy (subject, object, action, namespace_id) {
        subject -> Array<AbacAttributeSqlType>,
        object -> Array<AbacAttributeSqlType>,
        action -> Array<AbacAttributeSqlType>,
        namespace_id -> Uuid,
    }
}

table! {
    use types::AbacAttributeSqlType;

    abac_subject (inbound, outbound) {
        inbound -> AbacAttributeSqlType,
        outbound -> AbacAttributeSqlType,
    }
}

allow_tables_to_appear_in_same_query!(abac_action, abac_object, abac_policy, abac_subject,);

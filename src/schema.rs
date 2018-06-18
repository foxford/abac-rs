table! {
    abac_action (inbound, outbound) {
        inbound -> Abac_attribute,
        outbound -> Abac_attribute,
    }
}

table! {
    abac_object (inbound, outbound) {
        inbound -> Abac_attribute,
        outbound -> Abac_attribute,
    }
}

table! {
    abac_policy (subject, object, action, namespace_id) {
        subject -> Array<Abac_attribute>,
        object -> Array<Abac_attribute>,
        action -> Array<Abac_attribute>,
        namespace_id -> Uuid,
    }
}

table! {
    abac_subject (inbound, outbound) {
        inbound -> Abac_attribute,
        outbound -> Abac_attribute,
    }
}

allow_tables_to_appear_in_same_query!(
    abac_action,
    abac_object,
    abac_policy,
    abac_subject,
);

create table abac_policy (
    subject abac_attribute[],
    object abac_attribute[],
    action abac_attribute[],
    namespace_id uuid,

    primary key (subject, object, action, namespace_id)
);

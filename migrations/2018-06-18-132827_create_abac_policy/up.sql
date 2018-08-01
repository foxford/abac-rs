create table abac_policy (
    subject abac_attribute[],
    object abac_attribute[],
    action abac_attribute[],
    namespace_id uuid,
    created_at timestamptz not null default now(),

    primary key (subject, object, action, namespace_id)
);

create table abac_subject (
    inbound abac_attribute,
    outbound abac_attribute,
    created_at timestamptz not null default now(),

    primary key (inbound, outbound)
);

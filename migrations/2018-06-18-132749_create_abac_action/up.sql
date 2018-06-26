create table abac_action (
    inbound abac_attribute,
    outbound abac_attribute,

    primary key (inbound, outbound)
);

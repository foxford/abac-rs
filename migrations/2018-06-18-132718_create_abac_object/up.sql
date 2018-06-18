create table abac_object (
    inbound abac_attribute,
    outbound abac_attribute,

    primary key (inbound, outbound)
);

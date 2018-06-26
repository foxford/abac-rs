create table abac_subject (
    inbound abac_attribute,
    outbound abac_attribute,

    primary key (inbound, outbound)
);

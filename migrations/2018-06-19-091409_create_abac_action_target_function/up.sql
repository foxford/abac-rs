create function abac_action_target(_attrs abac_attribute[])
returns table (attr abac_attribute) as $$
    with recursive target as (
        select (outbound).value, (outbound).key, (outbound).namespace_id
            from abac_action
            where array[inbound] <@ _attrs
        union
        select (r.outbound).value, (r.outbound).key, (r.outbound).namespace_id
            from target as t
            inner join abac_action as r on r.inbound = (t.value, t.key, t.namespace_id) ::abac_attribute
    )
    select (unnest(_attrs)).*
    union
    select * from target
$$ language sql stable;

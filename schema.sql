drop extension "uuid-ossp" cascade;
create extension "uuid-ossp";

drop type abac_attribute cascade;
create type abac_attribute as (
    value text,
    key text,
    namespace_id uuid
);

drop table if exists abac_subject cascade;
create table abac_subject (
    inbound abac_attribute,
    outbound abac_attribute,

    primary key (inbound, outbound)
);

drop table if exists abac_object cascade;
create table abac_object (
    inbound abac_attribute,
    outbound abac_attribute,

    primary key (inbound, outbound)
);

drop table if exists abac_action cascade;
create table abac_action (
    inbound abac_attribute,
    outbound abac_attribute,

    primary key (inbound, outbound)
);

drop table if exists abac_policy cascade;
create table abac_policy (
    subject abac_attribute[],
    object abac_attribute[],
    action abac_attribute[],
    namespace_id uuid,

    primary key (subject, object, action, namespace_id)
);

create or replace function abac_object_target_namespace_list(_value_prefix text, _key text, _namespace_id uuid, _attrs abac_attribute[])
returns table (namespace_id uuid) as $$
    select substring(value from '\A' || _value_prefix || '(.*)') ::uuid
        from abac_object_target(_attrs)
        where value ~~ (_value_prefix || '%')
            and key = _key
            and namespace_id = _namespace_id;
$$ language sql stable;

create or replace function abac_object_target_namespace_array(_value_prefix text, _key text, _namespace_id uuid, _attrs abac_attribute[])
returns uuid[] as $$
    select array_agg(attr) from abac_object_target_namespace_list(_value_prefix, _key, _namespace_id, _attrs) as attr;
$$ language sql stable;

create or replace function abac_object_target(_attrs abac_attribute[])
returns table (attr abac_attribute) as $$
    with recursive target as (
        select (outbound).value, (outbound).key, (outbound).namespace_id
            from abac_object
            where array[inbound] <@ _attrs
        union
        select (r.outbound).value, (r.outbound).key, (r.outbound).namespace_id
            from target as t
            inner join abac_object as r on r.inbound = (t.value, t.key, t.namespace_id) ::abac_attribute
    )
    select (unnest(_attrs)).*
    union
    select * from target
$$ language sql stable;

create or replace function abac_subject_target(_attrs abac_attribute[])
returns table (attr abac_attribute) as $$
    with recursive target as (
        select (outbound).value, (outbound).key, (outbound).namespace_id
            from abac_subject
            where array[inbound] <@ _attrs
        union
        select (r.outbound).value, (r.outbound).key, (r.outbound).namespace_id
            from target as t
            inner join abac_subject as r on r.inbound = (t.value, t.key, t.namespace_id) ::abac_attribute
    )
    select (unnest(_attrs)).*
    union
    select * from target
$$ language sql stable;

create or replace function abac_action_target(_attrs abac_attribute[])
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

create or replace function abac_authorize(_subject abac_attribute[], _object abac_attribute[], _action abac_attribute[], _namespace_id uuid[])
returns boolean as $$
    select exists(
        select 1
            from abac_policy
            where
                subject <@ (select array_agg(distinct q) from abac_subject_target(_subject) as q)
                and object <@ (select array_agg(distinct q) from abac_object_target(_object) as q)
                and action <@ (select array_agg(distinct q) from abac_action_target(_action) as q)
                and array[namespace_id] <@ _namespace_id
            limit 1
    );
$$ language sql stable;

create or replace function abac_object_preflight_list(_attrs abac_attribute[], _key text)
returns table (attr abac_attribute) as $$
    with recursive target as (
        select (inbound).value, (inbound).key, (inbound).namespace_id
            from abac_object
            where array[outbound] <@ _attrs
                and (inbound).key != _key
        union
        select (r.inbound).value, (r.inbound).key, (r.inbound).namespace_id
            from target as t
            inner join abac_object as r on r.outbound = (t.value, t.key, t.namespace_id) ::abac_attribute
                and (r.inbound).key != _key
    )
    select (unnest(_attrs)).*
    union
    select * from target
$$ language sql stable;

create or replace function abac_object_preflight_array(_attrs abac_attribute[], _key text)
returns abac_attribute[] as $$
    select array_agg(t) from abac_object_preflight_list(_attrs, _key) as t;
$$ language sql stable;

create or replace function abac_object_list_1(_attr abac_attribute, _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
    select inbound
        from abac_object
        where outbound = _attr
        offset _offset
        limit _limit
$$ language sql stable;

create or replace function abac_object_list_2(_attr1 abac_attribute, _attr2 abac_attribute, _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
    select t1.inbound
        from abac_object as t1
        inner join abac_object as t2 on t1.inbound = t2.inbound
        where
            t1.outbound = _attr1
            and t2.outbound = _attr2
        offset _offset
        limit _limit
$$ language sql stable;

create or replace function abac_object_list_3(_attr1 abac_attribute, _attr2 abac_attribute, _attr3 abac_attribute, _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
    select t1.inbound
        from abac_object as t1
        inner join abac_object as t2 on t1.inbound = t2.inbound
        inner join abac_object as t3 on t1.inbound = t3.inbound
        where
            t1.outbound = _attr1
            and t2.outbound = _attr2
            and t3.outbound = _attr3
        offset _offset
        limit _limit
$$ language sql stable;

create or replace function abac_object_list(_attrs abac_attribute[], _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
begin
    case array_length(_attrs, 1)
        when 1 then return query select * from  abac_object_list_1(_attrs[1], _offset, _limit);
        when 2 then return query select * from  abac_object_list_2(_attrs[1], _attrs[2], _offset, _limit);
        when 3 then return query select * from  abac_object_list_3(_attrs[1], _attrs[2], _attrs[3], _offset, _limit);
        else raise exception 'bad argument' using detail = 'length of _attrs array shoud be less or equal to 3';
    end case;
end
$$
language plpgsql stable;

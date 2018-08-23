create function abac_object_target_namespace_list(_value_prefix text, _key text, _namespace_id uuid, _attrs abac_attribute[])
returns table (namespace_id uuid) as $$
    select substring(value from '\A' || _value_prefix || '(.*)') ::uuid
        from abac_object_target(_attrs)
        where value ~~ (_value_prefix || '%')
            and key = _key
            and namespace_id = _namespace_id;
$$ language sql stable;

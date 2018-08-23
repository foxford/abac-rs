create function abac_object_target_namespace_array(_value_prefix text, _key text, _namespace_id uuid, _attrs abac_attribute[])
returns uuid[] as $$
    select array_agg(attr) from abac_object_target_namespace_list(_value_prefix, _key, _namespace_id, _attrs) as attr;
$$ language sql stable;

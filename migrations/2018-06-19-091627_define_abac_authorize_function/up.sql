create function abac_authorize(_subject abac_attribute[], _object abac_attribute[], _action abac_attribute[], _namespace_id uuid[])
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

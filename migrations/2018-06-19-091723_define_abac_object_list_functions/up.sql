create function abac_object_list_1(_attr abac_attribute, _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
    select inbound
        from abac_object
        where outbound = _attr
        order by created_at
        offset _offset
        limit _limit
$$ language sql stable;

create function abac_object_list_2(_attr1 abac_attribute, _attr2 abac_attribute, _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
    select t1.inbound
        from abac_object as t1
        inner join abac_object as t2 on t1.inbound = t2.inbound
        where
            t1.outbound = _attr1
            and t2.outbound = _attr2
        order by t1.created_at
        offset _offset
        limit _limit
$$ language sql stable;

create function abac_object_list_3(_attr1 abac_attribute, _attr2 abac_attribute, _attr3 abac_attribute, _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
    select t1.inbound
        from abac_object as t1
        inner join abac_object as t2 on t1.inbound = t2.inbound
        inner join abac_object as t3 on t1.inbound = t3.inbound
        where
            t1.outbound = _attr1
            and t2.outbound = _attr2
            and t3.outbound = _attr3
        order by t1.created_at
        offset _offset
        limit _limit
$$ language sql stable;

create function abac_object_list(_attrs abac_attribute[], _offset integer, _limit integer)
returns table (attr abac_attribute) as $$
begin
    case array_length(_attrs, 1)
        when 1 then return query select * from  abac_object_list_1(_attrs[1], _offset, _limit);
        when 2 then return query select * from  abac_object_list_2(_attrs[1], _attrs[2], _offset, _limit);
        when 3 then return query select * from  abac_object_list_3(_attrs[1], _attrs[2], _attrs[3], _offset, _limit);
        else raise exception 'bad argument' using detail = 'length of _attrs array shoud be less or equal to 3';
    end case;
end
$$ language plpgsql stable;

create function abac_object_list(_attrs abac_attribute[], _offset integer, _limit integer)
returns setof abac_attribute as $$
begin
    case array_length(_attrs, 1)
        when 1 then return query
            select (t.inbound).*
            from abac_object as t
            where t.outbound = _attrs[1]
            order by t.created_at
            offset _offset
            limit _limit;
        when 2 then return query
            select (t1.inbound).*
            from abac_object as t1
            inner join abac_object as t2 on t1.inbound = t2.inbound
            where
                t1.outbound = _attrs[1]
                and t2.outbound = _attrs[2]
            order by t1.created_at
            offset _offset
            limit _limit;
        when 3 then return query
            select (t1.inbound).*
            from abac_object as t1
            inner join abac_object as t2 on t1.inbound = t2.inbound
            inner join abac_object as t3 on t1.inbound = t3.inbound
            where
                t1.outbound = _attrs[1]
                and t2.outbound = _attrs[2]
                and t3.outbound = _attrs[3]
            order by t1.created_at
            offset _offset
            limit _limit;
        else raise exception 'bad argument' using detail = 'array length should be less or equal to 3';
    end case;
end
$$ language plpgsql stable;

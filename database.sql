
create procedure add_new_value(arg_user_address character varying, arg_adder_address character varying, arg_amount numeric)
    language plpgsql
as
$$
DECLARE
    var_user_id INTEGER := (SELECT * FROM get_user_id(arg_user_address));
    var_adder_id INTEGER := (SELECT * FROM get_value_adder_id(arg_adder_address));
    add_id INTEGER;
BEGIN
        add_id = ( SELECT id FROM total_values_for_users WHERE sender_id=var_adder_id AND user_id=var_user_id);
        IF add_id IS NULL THEN
            INSERT INTO total_values_for_users (user_id, sender_id) VALUES
            (var_user_id,var_adder_id) RETURNING id INTO add_id;
        END IF;
        UPDATE total_values_for_users SET amount=amount+arg_amount WHERE id=add_id;
        UPDATE value_senders SET amount=amount+arg_amount WHERE id=var_adder_id;
        COMMIT;
        return;
END;
$$;

create function add_new_value_func(arg_user_address character varying, arg_adder_address character varying, arg_amount numeric) returns void
    language plpgsql
as
$$
DECLARE
    var_user_id INTEGER := (SELECT * FROM get_user_id(arg_user_address));
    var_adder_id INTEGER := (SELECT * FROM get_value_adder_id(arg_adder_address));
    add_id INTEGER;
BEGIN
        add_id = ( SELECT id FROM total_values_for_users WHERE sender_id=var_adder_id AND user_id=var_user_id);
        IF add_id IS NULL THEN
            INSERT INTO total_values_for_users (user_id, sender_id) VALUES
            (var_user_id,var_adder_id) RETURNING id INTO add_id;
        END IF;
        UPDATE total_values_for_users SET amount=amount+arg_amount WHERE id=add_id;
        UPDATE value_senders SET amount=amount+arg_amount WHERE id=var_adder_id;
        return;
END;
$$;

create function diesel_manage_updated_at(_tbl regclass) returns void
    language plpgsql
as
$$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;

create function get_user_id(arg_address character varying) returns integer
    language plpgsql
as
$$
DECLARE
    user_id INTEGER := (
        SELECT id FROM users WHERE address=lower(arg_address));
BEGIN
        IF user_id is null THEN
            INSERT INTO users (address) VALUES (lower(arg_address)) RETURNING id INTO user_id;
        END IF;
        RETURN user_id;
END;
$$;

CREATE OR REPLACE VIEW user_achievements AS
    SELECT
           u.id AS user_id,
           u.address,
           a.id,
           a.name,
           a.description,
           a.icon
    FROM users AS u
        JOIN achievements_to_users AS atu
            ON atu.user_id = u.id
        JOIN achievements AS a
            ON a.id = atu.achievement_id;

create view values_by_users(sender_address, name, sender_id, user_address, user_id, amount) as
SELECT s.address AS sender_address,
       s.name,
       s.id      AS sender_id,
       u.address AS user_address,
       u.id      AS user_id,
       tvfu.amount
FROM value_senders s
         JOIN total_values_for_users tvfu ON s.id = tvfu.sender_id
         JOIN users u ON tvfu.user_id = u.id;

create function get_users_values(arg_address character varying)
    returns TABLE(sender_address character varying, name character varying, sender_id integer, user_address character varying, user_id integer, amount numeric)
    language sql
as
$$
SELECT * FROM values_by_users WHERE values_by_users.user_id=(SELECT id FROM users WHERE address=arg_address);
$$;

create function get_value_adder_id(arg_address character varying) returns integer
    language plpgsql
as
$$
DECLARE
    adder_id INTEGER := (
        SELECT id FROM value_senders WHERE address=lower(arg_address));
BEGIN
        IF adder_id is null THEN
            INSERT INTO value_senders (address) VALUES (lower(arg_address)) RETURNING id INTO adder_id;
        END IF;
        RETURN adder_id;
END;
$$;

create procedure give_achievement(arg_address character varying, arg_achievement_id integer)
    language plpgsql
as
$$
declare
    var_user_id INTEGER := (SELECT * FROM get_user_id(arg_address));
begin
    INSERT INTO achievements_to_users(user_id, achievement_id) VALUES (var_user_id,arg_achievement_id);
    commit;
    return;
end;
$$;

create procedure givefee(arg_user_addr character varying, arg_vote_id integer, INOUT res boolean DEFAULT false)
    language plpgsql
as
$$
declare
    count bigint;
begin
    if not exists(select * from voters where user_address=arg_user_addr and round_id=arg_vote_id) then
        INSERT INTO voters(round_id, user_address) VALUES (arg_vote_id,arg_user_addr);
        res = true;
        commit;
        return;
    end if;
    count = (
        select
            vote_times
        from
            voters
        where
            user_address=arg_user_addr
        and
            round_id=arg_vote_id
    );
    if count > 5 then
        res=false;
        rollback;
        return;
    end if;
    update voters set vote_times=vote_times+1 where user_address=arg_user_addr and round_id=arg_vote_id;
    res = true;
    commit;
    return;
end;
$$;

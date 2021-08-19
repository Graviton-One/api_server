-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL UNIQUE,
    twitter_account VARCHAR UNIQUE
);
CREATE TABLE achievements (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR NOT NULL,
    icon    VARCHAR NOT NULL
);
CREATE TABLE achievements_to_users (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    achievement_id INT REFERENCES achievements(id)
);

CREATE TABLE value_senders (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    name VARCHAR NOT NULL DEFAULT 'undefined sender, probably should be someone from dev team',
    amount BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE total_values_for_users (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    sender_id INTEGER NOT NULL REFERENCES  value_senders(id),
    amount BIGINT NOT NULL DEFAULT 0
);


CREATE OR REPLACE FUNCTION get_value_adder_id(arg_address VARCHAR)
RETURNS INTEGER AS $$
DECLARE
    adder_id INTEGER := (
        SELECT id FROM value_senders WHERE address=lower(arg_address));
BEGIN
        IF adder_id is null THEN
            INSERT INTO value_senders (address) VALUES (lower(arg_address)) RETURNING id INTO adder_id;
        END IF;
        RETURN adder_id;
END;
$$  LANGUAGE plpgsql;
CREATE OR REPLACE FUNCTION get_user_id(arg_address VARCHAR)
RETURNS INTEGER AS $$
DECLARE
    user_id INTEGER := (
        SELECT id FROM users WHERE address=lower(arg_address));
BEGIN
        IF user_id is null THEN
            INSERT INTO users (address) VALUES (lower(arg_address)) RETURNING id INTO user_id;
        END IF;
        RETURN user_id;
END;
$$  LANGUAGE plpgsql;
create or replace procedure give_achievement(
   arg_address varchar,
   arg_achievement_id int
)
language plpgsql
as $$
declare
    var_user_id INTEGER := (SELECT * FROM get_user_id(arg_address));
begin
    INSERT INTO achievements_to_users(user_id, achievement_id) VALUES (var_user_id,arg_achievement_id);
    commit;
    return;
end;$$;
CREATE OR REPLACE PROCEDURE add_new_value(
    arg_user_address VARCHAR,
    arg_adder_address VARCHAR,
    arg_amount BIGINT
) AS $$
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
$$  LANGUAGE plpgsql;
create or replace procedure give_achievement(
   arg_address varchar,
   arg_achievement_id int
)
language plpgsql
as $$
declare
    var_user_id INTEGER := (SELECT * FROM get_user_id(arg_address));
begin
    INSERT INTO achievements_to_users(user_id, achievement_id) VALUES (var_user_id,arg_achievement_id);
    commit;
    return;
end;$$;
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


CREATE OR REPLACE VIEW values_by_users AS
    SELECT
        s.address AS sender_address,
        s.name,
        s.id AS sender_id,
        u.address AS user_address,
        u.id AS user_id,
        tvfu.amount
    FROM value_senders AS s
        JOIN total_values_for_users tvfu on s.id = tvfu.sender_id
        JOIN users u on tvfu.user_id = u.id;

CREATE OR REPLACE FUNCTION get_users_values(arg_address VARCHAR) RETURNS TABLE (
    sender_address VARCHAR,
    name VARCHAR,
    sender_id INTEGER,
    user_address VARCHAR,
    user_id INTEGER,
    amount BIGINT
)
AS $$
    SELECT * FROM values_by_users WHERE values_by_users.user_id=(SELECT id FROM users WHERE address=arg_address);
$$ LANGUAGE  sql;



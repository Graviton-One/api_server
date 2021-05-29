-- Your SQL goes here
DROP TABLE value_senders CASCADE;
DROP TABLE total_values_for_users CASCADE;
DROP FUNCTION get_users_values;

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



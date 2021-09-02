-- This file should undo anything in `up.sql`
DROP TABLE users CASCADE;
DROP TABLE value_senders CASCADE;
DROP TABLE total_values_for_users CASCADE;
DROP TABLE achievements CASCADE;
DROP TABLE achievements_to_users CASCADE;

DROP FUNCTION get_users_values;

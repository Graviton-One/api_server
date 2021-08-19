CREATE TABLE voters (
  id SERIAL PRIMARY KEY,
  round_id INT NOT NULL,
  user_address VARCHAR NOT NULL,
  vote_times INT NOT NULL DEFAULT 1
);

CREATE TABLE gton_price (
    id SERIAL PRIMARY KEY,
    price DOUBLE PRECISION NOT NULL,
    market_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create or replace procedure GiveFee(
   arg_user_addr varchar,
   arg_vote_id int,
   inout  res bool default false
)
language plpgsql
as $$
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
end;$$;

-- migrate:up
drop table if exists game_result;

-- migrate:down
create table if not exists game_result(
    game_id uuid not null references game(id) on delete cascade,
    account_id uuid not null references account(id) on delete cascade,
    placement int not null,
    primary key (game_id, account_id)
);

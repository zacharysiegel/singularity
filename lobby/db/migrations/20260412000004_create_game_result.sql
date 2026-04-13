-- migrate:up
create table if not exists game_result(
    game_id uuid not null references game(id) on delete cascade,
    account_id uuid not null references account(id) on delete cascade,
    placement int not null,
    accolades jsonb not null default '{}',
    stats jsonb not null default '{}',
    primary key (game_id, account_id)
);

-- migrate:down
drop table if exists game_result;

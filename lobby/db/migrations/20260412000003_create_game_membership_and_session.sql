-- migrate:up
create table if not exists game_membership(
    game_id uuid not null references game(id) on delete cascade,
    account_id uuid not null references account(id) on delete cascade,
    joined timestamptz not null default now(),
    primary key (game_id, account_id)
);

create table if not exists game_session(
    id uuid primary key default uuidv7(),
    game_id uuid not null references game(id) on delete cascade,
    account_id uuid not null references account(id) on delete cascade,
    session_id uuid not null references session(id) on delete cascade,
    entered timestamptz not null default now(),
    exited timestamptz
);

create unique index if not exists idx_game_session_active
    on game_session(game_id, account_id)
    where exited is null;

-- migrate:down
drop index if exists idx_game_session_active;
drop table if exists game_session;
drop table if exists game_membership;

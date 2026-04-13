-- migrate:up
create table if not exists game(
    id uuid primary key default uuidv7(),
    name text not null,
    creator_id uuid not null references account(id),
    status int not null default 0,
    max_players int not null default 8,
    created timestamptz not null default now(),
    updated timestamptz not null default now()
);

create index if not exists idx_game_status on game(status);
create index if not exists idx_game_creator_id on game(creator_id);

-- migrate:down
drop index if exists idx_game_creator_id;
drop index if exists idx_game_status;
drop table if exists game;

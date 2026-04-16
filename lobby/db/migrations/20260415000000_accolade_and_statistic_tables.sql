-- migrate:up
alter table game_result drop column if exists accolades;
alter table game_result drop column if exists stats;

create table if not exists accolade(
    id uuid primary key default uuidv7(),
    account_id uuid not null references account(id) on delete cascade,
    game_id uuid not null references game(id) on delete cascade,
    accolade_type text not null,
    awarded timestamptz not null default now()
);

create index if not exists idx_accolade_account_id on accolade(account_id);
create index if not exists idx_accolade_game_id on accolade(game_id);

create table if not exists statistic(
    id uuid primary key default uuidv7(),
    account_id uuid not null references account(id) on delete cascade,
    game_id uuid references game(id) on delete cascade,
    statistic_type text not null,
    value double precision not null default 0,
    updated timestamptz not null default now()
);

create index if not exists idx_statistic_account_id on statistic(account_id);
create index if not exists idx_statistic_game_id on statistic(game_id);

-- migrate:down
drop index if exists idx_statistic_game_id;
drop index if exists idx_statistic_account_id;
drop table if exists statistic;

drop index if exists idx_accolade_game_id;
drop index if exists idx_accolade_account_id;
drop table if exists accolade;

alter table game_result add column if not exists accolades jsonb not null default '{}';
alter table game_result add column if not exists stats jsonb not null default '{}';

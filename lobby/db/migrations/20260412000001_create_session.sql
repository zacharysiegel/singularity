-- migrate:up
create table if not exists session(
    id uuid primary key default uuidv7(),
    account_id uuid not null references account(id) on delete cascade,
    token text not null unique,
    created timestamptz not null default now(),
    expires timestamptz not null
);

create index if not exists idx_session_token on session(token);
create index if not exists idx_session_account_id on session(account_id);

-- migrate:down
drop index if exists idx_session_account_id;
drop index if exists idx_session_token;
drop table if exists session;

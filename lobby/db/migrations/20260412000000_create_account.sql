-- migrate:up
drop table if exists users;

create table if not exists account(
    id uuid primary key default uuidv7(),
    email text not null unique,
    username text not null unique,
    password_hash text not null,
    created timestamptz not null default now(),
    updated timestamptz not null default now()
);

-- migrate:down
drop table if exists account;

create table if not exists users(
    id uuid primary key
);

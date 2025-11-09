-- migrate:up
create table users(
    id uuid primary key
);

-- migrate:down

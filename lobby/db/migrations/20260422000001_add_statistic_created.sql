-- migrate:up
alter table statistic add column if not exists created timestamptz not null default now();

-- migrate:down
alter table statistic drop column if exists created;

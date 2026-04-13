-- migrate:up
alter table account add column if not exists deleted_at timestamptz;

-- migrate:down
alter table account drop column if exists deleted_at;

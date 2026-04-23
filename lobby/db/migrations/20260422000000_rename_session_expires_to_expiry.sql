-- migrate:up
alter table session rename column expires to expiry;

-- migrate:down
alter table session rename column expiry to expires;

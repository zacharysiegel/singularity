-- migrate:up
create table if not exists follow(
    source_account_id uuid not null references account(id) on delete cascade,
    target_account_id uuid not null references account(id) on delete cascade,
    created timestamptz not null default now(),
    primary key (source_account_id, target_account_id),
    check (source_account_id != target_account_id)
);

create index if not exists idx_follow_target on follow(target_account_id);
comment on index idx_follow_target is 'Reverse lookup; the composite PK already covers (source_account_id, target_account_id)';

create or replace view mutual_follow_view as
select
    f1.source_account_id as account_id,
    f1.target_account_id as mutual_account_id
from follow f1
inner join follow f2
    on f1.source_account_id = f2.target_account_id
    and f1.target_account_id = f2.source_account_id
;
comment on view mutual_follow_view is 'Accounts that follow each other. Not materialized; both sides hit the composite PK index.';

-- migrate:down
drop view if exists mutual_follow_view;
drop index if exists idx_follow_target;
drop table if exists follow;

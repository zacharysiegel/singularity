-- migrate:up
create table if not exists follow(
    follower_account_id uuid not null references account(id) on delete cascade,
    followed_account_id uuid not null references account(id) on delete cascade,
    created timestamptz not null default now(),
    primary key (follower_account_id, followed_account_id),
    check (follower_account_id != followed_account_id)
);

create index if not exists idx_follow_followed on follow(followed_account_id);
comment on index idx_follow_followed is 'Reverse lookup; the composite PK already covers (follower_account_id, followed_account_id)';

create or replace view mutual_follow_view as
select
    f1.follower_account_id as account_id,
    f1.followed_account_id as mutual_account_id
from follow f1
inner join follow f2
    on f1.follower_account_id = f2.followed_account_id
    and f1.followed_account_id = f2.follower_account_id
;
comment on view mutual_follow_view is 'Accounts that follow each other. Not materialized; both sides hit the composite PK index.';

-- migrate:down
drop view if exists mutual_follow_view;
drop index if exists idx_follow_followed;
drop table if exists follow;


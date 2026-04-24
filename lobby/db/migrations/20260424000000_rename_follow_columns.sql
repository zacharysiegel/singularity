-- migrate:up
drop view if exists mutual_follow_view;

alter table follow rename column follower_account_id to source_account_id;
alter table follow rename column followed_account_id to target_account_id;

drop index if exists idx_follow_followed;
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

alter table follow rename column source_account_id to follower_account_id;
alter table follow rename column target_account_id to followed_account_id;

drop index if exists idx_follow_target;
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

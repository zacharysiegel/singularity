-- migrate:up
create or replace view game_member_count_view as
select
    game_membership.game_id,
    count(*) as member_count
from game_membership
group by game_membership.game_id
;

-- migrate:down
drop view if exists game_member_count_view;

-- migrate:up
create or replace view game_member_count_view as
select
    game.id as game_id,
    count(game_membership.game_id) as member_count
from game
left join game_membership on game_membership.game_id = game.id
group by game.id
;

-- migrate:down
drop view if exists game_member_count_view;

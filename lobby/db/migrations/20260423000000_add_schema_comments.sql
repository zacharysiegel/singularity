-- migrate:up
comment on column account.deleted_at is 'NULL indicates active account; non-NULL indicates soft-deleted with PII anonymized';
comment on column session.expiry is 'Session is rejected when expiry < now(); extended via debounced sliding window on authenticated requests';
comment on column game.status is 'Integer enum: 0=Pending, 1=Active, 2=Completed (see shared::schema::game::GameStatus)';
comment on column game_session.exited is 'NULL indicates active session; non-NULL indicates the player has disconnected from the game';
comment on index idx_game_session_active is 'Partial unique index preventing duplicate active sessions per player per game';
comment on view game_member_count_view is 'Member count per game including zero-member games via left join';

-- migrate:down
comment on column account.deleted_at is NULL;
comment on column session.expiry is NULL;
comment on column game.status is NULL;
comment on column game_session.exited is NULL;
comment on index idx_game_session_active is NULL;
comment on view game_member_count_view is NULL;


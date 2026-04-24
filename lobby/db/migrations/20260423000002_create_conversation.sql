-- migrate:up
create table if not exists conversation(
    id uuid primary key default uuidv7(),
    game_id uuid references game(id) on delete cascade,
    name text,
    created timestamptz not null default now()
);
comment on column conversation.game_id is 'NULL for global conversations, non-NULL for in-game conversations';

create index if not exists idx_conversation_game_id on conversation(game_id);

create table if not exists conversation_member(
    conversation_id uuid not null references conversation(id) on delete cascade,
    account_id uuid not null references account(id) on delete cascade,
    entered timestamptz not null default now(),
    exited timestamptz,
    primary key (conversation_id, account_id)
);
comment on column conversation_member.exited is 'NULL indicates active membership';

create table if not exists conversation_message(
    id uuid primary key default uuidv7(),
    conversation_id uuid not null references conversation(id) on delete cascade,
    sender_account_id uuid not null references account(id),
    content text not null,
    created timestamptz not null default now()
);

create index if not exists idx_conversation_message_conversation_created
    on conversation_message(conversation_id, created);
comment on index idx_conversation_message_conversation_created is 'Messages in a conversation sorted by time';

create index if not exists idx_conversation_message_sender_account_id
    on conversation_message(sender_account_id);

create or replace view conversation_latest_message_view as
select
    conversation_message.conversation_id,
    max(conversation_message.created) as latest_message_created
from conversation_message
group by conversation_message.conversation_id
;
comment on view conversation_latest_message_view is 'Most recent message timestamp per conversation, for sorting conversation lists';

-- migrate:down
drop view if exists conversation_latest_message_view;
drop index if exists idx_conversation_message_sender_account_id;
drop index if exists idx_conversation_message_conversation_created;
drop table if exists conversation_message;
drop table if exists conversation_member;
drop index if exists idx_conversation_game_id;
drop table if exists conversation;
